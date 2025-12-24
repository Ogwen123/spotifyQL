use crate::app_context::{AppContext};
use crate::utils::logger::fatal;
use crate::utils::url::build_url;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use std::cmp::PartialEq;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use serde_json::Value;
use tokio::runtime::Runtime;
use warp::Filter;
use warp::trace::request;
use crate::query::data::{AlbumData, PlaylistData, TrackData};

#[derive(Debug)]
pub struct APIQuery {
    url: String,
    limit: Option<u32>,
    offset: Option<u32>,
    fields: Option<String>
}

#[derive(Debug, PartialEq)]
pub enum QueryType {
    UserPlaylist,
    UserFollowing,
    UserSavedAlbums,
    UserSavedTracks,
    Search,
    None,
}

impl QueryType {
    fn make_endpoint(&self, start: &str) -> String {
        match self {
            QueryType::UserPlaylist => start.to_string() + "/me/playlists",
            QueryType::UserFollowing => start.to_string() + "/me/following",
            QueryType::UserSavedAlbums => start.to_string() + "/me/albums",
            QueryType::UserSavedTracks => start.to_string() + "/me/tracks",
            QueryType::Search => start.to_string() + "/search",
            QueryType::None => start.to_string(),
        }
    }
}


struct ResultParser;

impl ResultParser {
    fn parse_playlists(str_data: String) -> Result<Vec<PlaylistData>, String> {
        println!("{}", str_data);
        let mut playlists: Vec<PlaylistData> = Vec::new();
        let val: Value = serde_json::from_str(str_data.as_str()).map_err(|x| x.to_string())?;


        let raw_playlists: Vec<Value>;

        if let Value::Array(pl) = &val["items"] {
            raw_playlists = pl.clone();
        } else {
            return Err("'items' field in response data is an unexpected type. (1)".to_string())
        }


        for i in raw_playlists {
            match i {
                Value::Object(obj) => {
                    let id = match &obj["id"] {
                        Value::String(res) => res.clone(),
                        _ => {
                            return Err("Value in field 'items' in response data is an unexpected type. (id)".to_string())
                        }
                    };
                    let name = match &obj["name"] {
                        Value::String(res) => res.clone(),
                        _ => {
                            return Err("Value in field 'items' in response data is an unexpected type. (name)".to_string())
                        }
                    };
                    let track_data = match &obj["tracks"] {
                        Value::Object(tracks_obj) => {
                            let api = match &tracks_obj["href"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!("Value in field 'tracks' of playlist {} in response data is an unexpected type. (href)", name))
                                }
                            };
                            let total = match &tracks_obj["total"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        res.as_u64().expect("You shouldn't see this error message")
                                    } else {
                                        return Err(format!("Value in field 'tracks' of playlist {} in response data is not a positive integer. (total)", name))
                                    }
                                },
                                _ => {
                                    return Err(format!("Value in field 'tracks' of playlist {} in response data is an unexpected type. (total)", name))
                                }
                            };
                            (api, total)
                        },
                        _ => {
                            return Err(format!("Value of field 'tracks' of playlist {} in response data is an unexpected type. (1)", name))
                        }
                    };



                    playlists.push(PlaylistData {
                        id,
                        name,
                        tracks: Vec::new(),
                        tracks_api: track_data.0,
                        track_count: track_data.1
                    })
                },
                _ => {
                    return Err("Value in field 'items' in response data is an unexpected type. (1)".to_string())
                }
            }
            
            
            // fetch track data
        }

        println!("{:?}", playlists);

        Ok(playlists)
    }

    fn parse_albums(str_data: String) -> Result<Vec<AlbumData>, String> {
        println!("{}", str_data);
        Ok(Vec::new())
    }

    fn parse_tracks(str_data: String) -> Result<Vec<TrackData>, String> {
        println!("{}", str_data);
        Ok(Vec::new())
    }
}

impl<'a> APIQuery {
    const API_ENDPOINT: &'a str = "https://api.spotify.com/v1";

    pub fn new() {}

    /// Get all of a users playlists
    pub fn get_playlists(
        cx: &AppContext,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<PlaylistData>, String> {
        let url = QueryType::UserPlaylist.make_endpoint(Self::API_ENDPOINT);

        let query = APIQuery { url, limit, offset, fields: None };

        let raw_data = query.send(cx)?;

        Ok(ResultParser::parse_playlists(raw_data)?)
    }

    pub fn get_saved_albums(
        cx: &AppContext,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<AlbumData>, String> {
        let url = QueryType::UserSavedAlbums.make_endpoint(Self::API_ENDPOINT);

        let query = APIQuery { url, limit, offset, fields: None };

        let raw_data = query.send(cx)?;

        Ok(ResultParser::parse_albums(raw_data)?)
    }

    pub fn get_playlist_tracks(
        cx: &AppContext,
        playlist_id: String
    ) -> Result<Vec<TrackData>, String> {
        let url = QueryType::UserSavedAlbums.make_endpoint(Self::API_ENDPOINT);

        let query = APIQuery { url, limit: None, offset: None, fields: Some(String::from("")) };

        let raw_data = query.send(cx)?;

        Ok(ResultParser::parse_tracks(raw_data)?)
    }

    /// Spawns a thread to send the API request async, returns data using a channel
    fn send_async(url: String, tx: Sender<Result<String, String>>, token: String) {
        thread::spawn(move || {
            let rt = Runtime::new().expect("Could not init tokio runtime");
            rt.block_on(async move {
                println!("{}", token);
                println!("{}", url);
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

        let final_url = build_url(self.url, params);

        let (tx, rx) = channel::<Result<String, String>>();

        Self::send_async(final_url, tx, cx.token.clone());

        let res = rx.recv().expect("API request thread stopped unexpectedly.");

        res
    }
}
