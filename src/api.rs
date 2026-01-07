use crate::app_context::AppContext;
use crate::query::data::{AlbumData, PlaylistData, ResultParser, TrackData};
use crate::utils::logger::fatal;
use crate::utils::url::build_url;
use regex::Regex;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::cmp::PartialEq;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use tokio::runtime::Runtime;
use warp::Filter;
use warp::trace::request;

#[derive(Debug)]
pub struct APIQuery {
    url: String,
    limit: Option<u32>,
    offset: Option<u32>,
    fields: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum QueryType {
    UserPlaylist,
    UserPlaylistTracks,
    UserFollowing,
    UserSavedAlbums,
    UserSavedTracks,
    Search,
    None,
}

impl QueryType {
    fn make_endpoint(&self, start: &str, id: Option<String>) -> String {
        let mut url = match self {
            QueryType::UserPlaylist => start.to_string() + "/me/playlists",
            QueryType::UserPlaylistTracks => start.to_string() + "/playlists/{id}/tracks",
            QueryType::UserFollowing => start.to_string() + "/me/following",
            QueryType::UserSavedAlbums => start.to_string() + "/me/albums",
            QueryType::UserSavedTracks => start.to_string() + "/me/tracks",
            QueryType::Search => start.to_string() + "/search",
            QueryType::None => start.to_string(),
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

impl<'a> APIQuery {
    const API_ENDPOINT: &'a str = "https://api.spotify.com/v1";

    /// Get all of a users playlists
    pub fn get_playlists(
        cx: &AppContext,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<PlaylistData>, String> {
        let url = QueryType::UserPlaylist.make_endpoint(Self::API_ENDPOINT, None);

        let query = APIQuery {
            url,
            limit,
            offset,
            fields: None,
        };

        let raw_data = query.send(cx)?;

        let mut playlists = ResultParser::parse_playlists(raw_data)?;

        // get playlist data
        for i in playlists.iter_mut() {
            let tracks = APIQuery::get_playlist_tracks(cx, i.id.clone())?;

            i.tracks = tracks;
        }

        Ok(playlists)
    }

    pub fn get_saved_albums(
        cx: &AppContext,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<AlbumData>, String> {
        let url = QueryType::UserSavedAlbums.make_endpoint(Self::API_ENDPOINT, None);

        let query = APIQuery {
            url,
            limit,
            offset,
            fields: None,
        };

        let raw_data = query.send(cx)?;

        Ok(ResultParser::parse_albums(raw_data)?)
    }

    pub fn get_playlist_tracks(
        cx: &AppContext,
        playlist_id: String,
    ) -> Result<Vec<TrackData>, String> {
        let url = QueryType::UserPlaylistTracks
            .make_endpoint(Self::API_ENDPOINT, Some(playlist_id.clone()));

        let query = APIQuery {
            url,
            limit: None,
            offset: None,
            fields: Some(String::from(
                "items(added_at,track(id,name,duration_ms,popularity,album(id,name),artists(id,name))",
            )),
        };

        let raw_data = query.send(cx)?;

        Ok(ResultParser::parse_tracks(raw_data, playlist_id)?)
    }

    /// Spawns a thread to send the API request async, returns data using a channel
    fn send_async(url: String, tx: Sender<Result<String, String>>, token: String) {
        thread::spawn(move || {
            let rt = Runtime::new().expect("Could not init tokio runtime");
            rt.block_on(async move {
                let client = reqwest::Client::new();
                println!("{}", url);
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
        if self.limit.is_some() {
            params.push(("limit", self.limit.unwrap().to_string()));
        }
        if self.offset.is_some() {
            params.push(("offset", self.offset.unwrap().to_string()));
        }
        if self.fields.is_some() {
            params.push(("fields", self.fields.unwrap()))
        }

        let final_url = build_url(self.url, params);

        let (tx, rx) = channel::<Result<String, String>>();

        Self::send_async(final_url, tx, cx.token.clone());

        let res = rx.recv().expect("API request thread stopped unexpectedly.");

        res
    }
}
