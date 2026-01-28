use crate::app_context::AppContext;
use crate::query::data::{AlbumData, PlaylistData, ResultParser, TrackData};
use crate::utils::logger::fatal;
use crate::utils::url::build_url;
use regex::Regex;
use reqwest::Response;
use std::cmp::PartialEq;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct APIQuery {
    url: String,
    limit: usize,
    offset: usize,
    fields: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum QueryType {
    UserPlaylist,
    UserPlaylistTracks,
    UserFollowing,
    UserSavedAlbums,
    AlbumTracks,
}

impl QueryType {
    fn make_endpoint(&self, start: &str, id: Option<String>) -> String {
        let mut url = match self {
            QueryType::UserPlaylist => start.to_string() + "/me/playlists",
            QueryType::UserPlaylistTracks => start.to_string() + "/playlists/{id}/tracks",
            QueryType::UserFollowing => start.to_string() + "/me/following",
            QueryType::UserSavedAlbums => start.to_string() + "/me/albums",
            QueryType::AlbumTracks => start.to_string() + "/albums/{id}/tracks",
        };

        if id.is_some() {
            // verify the url can accept an id
            let id_url =
                Regex::new(r"[/\w]+\/\{id\}\/[/\w]+").expect("id_url Regex failed to init.");

            if id_url.is_match(url.as_str()) {
                url = url.replace("{id}", id.unwrap().as_str());
            } else {
                fatal!("Could not insert id into URL.")
            }
        }

        url
    }
}

static API_ENDPOINT: &str = "https://api.spotify.com/v1";
static MAX_RESPONSE_ITEMS: usize = 50;

impl APIQuery {
    /// Get all of a users playlists
    pub fn get_playlists(cx: &AppContext) -> Result<Vec<PlaylistData>, String> {
        let url = QueryType::UserPlaylist.make_endpoint(API_ENDPOINT, None);

        let mut playlists: Vec<PlaylistData> = Vec::new();

        let mut count = 0;
        /*
        spotify only allows for fetching 50 items per request so this loop will keep fetching 50
        items at a time until a request returns less than 50 items and the loop will break,
        there is a limit of 50 requests to avoid the possibility of an infinite loop
        */
        loop {
            if count == 49 {
                break;
            } // allows for 50*MAX_RESPONSE_ITEMS playlists to be fetched

            let query = APIQuery {
                url: url.clone(),
                limit: MAX_RESPONSE_ITEMS,
                offset: MAX_RESPONSE_ITEMS * count,
                fields: None,
            };

            let raw_data = query.send(cx)?;

            let mut temp_playlists = &mut ResultParser::parse_playlists(raw_data)?;

            // get playlist data
            for i in temp_playlists.iter_mut() {
                let tracks = APIQuery::get_playlist_tracks(cx, i.id.clone())?;

                i.tracks = tracks;
            }

            let loaded_playlists = temp_playlists.len();

            playlists.append(&mut temp_playlists);

            if loaded_playlists < MAX_RESPONSE_ITEMS {
                break;
            }

            count += 1;
        }

        Ok(playlists)
    }

    pub fn get_saved_albums(cx: &AppContext) -> Result<Vec<AlbumData>, String> {
        let url = QueryType::UserSavedAlbums.make_endpoint(API_ENDPOINT, None);

        let mut albums: Vec<AlbumData> = Vec::new();

        let mut count: usize = 0;

        loop {
            if count == 49 {
                break;
            }

            let query = APIQuery {
                url: url.clone(),
                limit: MAX_RESPONSE_ITEMS,
                offset: MAX_RESPONSE_ITEMS * count,
                fields: None,
            };

            let raw_data = query.send(cx)?;

            let mut temp_albums = ResultParser::parse_albums(raw_data)?;

            for i in albums.iter_mut() {
                let tracks = APIQuery::get_album_tracks(cx, i.id.clone())?;

                i.tracks = tracks;
            }

            let loaded_albums = temp_albums.len();

            albums.append(&mut temp_albums);

            if loaded_albums < MAX_RESPONSE_ITEMS {
                break;
            }

            count += 1;
        }

        Ok(albums)
    }

    pub fn get_playlist_tracks(
        cx: &AppContext,
        playlist_id: String,
    ) -> Result<Vec<TrackData>, String> {
        let url =
            QueryType::UserPlaylistTracks.make_endpoint(API_ENDPOINT, Some(playlist_id.clone()));

        let mut tracks: Vec<TrackData> = Vec::new();
        let mut count: usize = 0;

        loop {
            if count == 99 {
                break;
            } // allows for 100*MAX_RESPONSE_ITEMS to be fetched

            let query = APIQuery {
                url: url.clone(),
                limit: MAX_RESPONSE_ITEMS,
                offset: MAX_RESPONSE_ITEMS * count,
                fields: Some(String::from(
                    "items(added_at,track(id,name,duration_ms,popularity,album(id,name),artists(id,name))",
                )),
            };

            let raw_data = query.send(cx)?;

            let mut temp_tracks = ResultParser::parse_tracks(raw_data, &playlist_id)?;

            let loaded_tracks = temp_tracks.len();

            tracks.append(&mut temp_tracks);

            if loaded_tracks < MAX_RESPONSE_ITEMS {
                break;
            }

            count += 1;
        }

        Ok(tracks)
    }

    pub fn get_album_tracks(
        cx: &AppContext,
        playlist_id: String,
    ) -> Result<Vec<TrackData>, String> {
        let url = QueryType::AlbumTracks.make_endpoint(API_ENDPOINT, Some(playlist_id.clone()));

        let mut tracks: Vec<TrackData> = Vec::new();
        let mut count: usize = 0;

        loop {
            if count == 99 {
                break;
            } // allows for 100*MAX_RESPONSE_ITEMS to be fetched

            let query = APIQuery {
                url: url.clone(),
                limit: MAX_RESPONSE_ITEMS,
                offset: MAX_RESPONSE_ITEMS * count,
                fields: Some(String::from(
                    "items(added_at,track(id,name,duration_ms,popularity,album(id,name),artists(id,name))",
                )),
            };

            let raw_data = query.send(cx)?;

            let mut temp_tracks = ResultParser::parse_tracks(raw_data, &playlist_id)?;

            let loaded_tracks = temp_tracks.len();

            tracks.append(&mut temp_tracks);

            if loaded_tracks < MAX_RESPONSE_ITEMS {
                break;
            }

            count += 1;
        }

        Ok(tracks)
    }

    /// Spawns a thread to send the API request async, returns data using a channel
    fn send_async(url: String, tx: Sender<Result<String, String>>, token: String) {
        thread::spawn(move || {
            let rt = Runtime::new().expect("Could not init tokio runtime");
            rt.block_on(async move {
                let client = reqwest::Client::new();
                let resp_result = client
                    .get(url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;

                let resp: Response = match resp_result {
                    Ok(res) => res,
                    Err(err) => {
                        tx.send(Err(err.to_string()))
                            .expect("Failed to send error down request channel. (1)");
                        return;
                    }
                };

                if resp.status() != 200 {
                    tx.send(Err(format!("API query failed with code {}", resp.status()))).expect("Failed to send success response down request channel. (1)");
                    return
                }

                let body_result = resp.text().await;

                let body = match body_result {
                    Ok(res) => res,
                    Err(err) => {
                        tx.send(Err(err.to_string()))
                            .expect("Failed to send error down request channel. (2)");
                        return;
                    }
                };

                tx.send(Ok(body))
                    .expect("Failed to send success response down request channel. (1)");
            });
        });
    }

    /// Send the given request
    fn send(self, cx: &AppContext) -> Result<String, String> {
        let mut params: Vec<(&str, String)> = Vec::new();
        // build the param list
        params.push(("limit", self.limit.to_string()));

        params.push(("offset", self.offset.to_string()));

        if self.fields.is_some() {
            params.push(("fields", self.fields.unwrap()))
        }

        let final_url = build_url(self.url, params);

        let (tx, rx) = channel::<Result<String, String>>();
        // println!("{:?}", final_url);
        Self::send_async(final_url, tx, cx.token.clone());

        let res = rx.recv().expect("API request thread stopped unexpectedly.");

        res
    }
}
