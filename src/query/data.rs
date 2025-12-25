use std::collections::HashMap;
use std::iter::Map;
use serde_json::Value;
use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::tokenise::{DataSource, Token};
use crate::utils::utils::secs_now;

pub const DATA_TTL: u64 = 60*30;

#[derive(Clone, Debug)]
pub struct TrackData {
    pub id: String,
    pub name: String,
    pub duration: u64,
    pub album_name: String,
    pub album_id: String,
    pub artists: HashMap<String, String>,
    pub added_at: String,
    pub popularity: u8 // value between 0 and 100
}

#[derive(Clone, Debug)]
pub struct PlaylistData {
    pub id: String,
    pub name: String,
    pub tracks: Vec<TrackData>,
    pub tracks_api: String,
    pub track_count: u64
}

#[derive(Clone)]
pub struct AlbumData {
    pub name: String,
}

#[derive(Clone)]
pub struct Data {
    /// Creation time of the playlist data
    pub playlist_data_ct: u64,
    pub playlist_data: Option<Vec<PlaylistData>>,
    /// Creation time of the saved album data
    pub saved_album_data_ct: u64,
    pub saved_album_data: Option<Vec<AlbumData>>,
}

pub fn load_data_source(cx: &mut AppContext, source: DataSource) -> Result<(), String> {

    //check playlist data
    match source {
        DataSource::Playlist(_) => {
            let mut load = false;

            if cx.data.playlist_data.is_some() {
                if cx.data.playlist_data_ct + DATA_TTL < secs_now() {
                    load = true;
                }
            } else {
                load = true;
            }

            if load {
                cx.data.playlist_data = Some(APIQuery::get_playlists(cx, None, None)?);
                cx.data.playlist_data_ct = secs_now();
            }
        },
        DataSource::SavedAlbums(_) => {
            let mut load = false;

            if cx.data.saved_album_data.is_some() {
                if cx.data.saved_album_data_ct + DATA_TTL < secs_now() {
                    load = true;
                }
            } else {
                load = true;
            }

            if load {
                cx.data.saved_album_data = Some(APIQuery::get_saved_albums(cx, None, None)?);
                cx.data.saved_album_data_ct = secs_now();
            }
        }
    }

    Ok(())
}

/// Extract data with complete error handling
pub struct ResultParser;

impl ResultParser {
    pub fn parse_playlists(str_data: String) -> Result<Vec<PlaylistData>, String> {
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
                            return Err("Value 'id' in field 'items' in response data is an unexpected type.".to_string())
                        }
                    };
                    let name = match &obj["name"] {
                        Value::String(res) => res.clone(),
                        _ => {
                            return Err("Value 'name' in field 'items' in response data is an unexpected type.".to_string())
                        }
                    };
                    let track_data = match &obj["tracks"] {
                        Value::Object(tracks_obj) => {
                            let api = match &tracks_obj["href"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!("Value 'href' in field 'tracks' of playlist {} in response data is an unexpected type.", name))
                                }
                            };
                            let total = match &tracks_obj["total"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        res.as_u64().expect("You shouldn't see this error message")
                                    } else {
                                        return Err(format!("Value 'total' in field 'tracks' of playlist {} in response data is not a positive integer.", name))
                                    }
                                },
                                _ => {
                                    return Err(format!("Value 'total' in field 'tracks' of playlist {} in response data is an unexpected type.", name))
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
        }
        Ok(playlists)
    }

    pub fn parse_albums(str_data: String) -> Result<Vec<AlbumData>, String> {
        println!("{}", str_data);

        Ok(Vec::new())
    }

    pub fn parse_tracks(str_data: String) -> Result<Vec<TrackData>, String> {
        let mut tracks: Vec<TrackData> = Vec::new();
        let val: Value = serde_json::from_str(str_data.as_str()).map_err(|x| x.to_string())?;


        let raw_tracks: Vec<Value>;

        if let Value::Array(pl) = &val["items"] {
            raw_tracks = pl.clone();
        } else {
            return Err("'items' field in response data is an unexpected type. (2)".to_string())
        }


        for i in raw_tracks {
            match i {
                Value::Object(obj) => {
                    let added_at = match &obj["added_at"] {
                        Value::String(res) => res.clone(),
                        _ => {
                            return Err("Value 'added_at' in field 'items' in response data is an unexpected type.".to_string())
                        }
                    };

                    let track_data = match &obj["track"] {
                        Value::Object(track) => {

                            let id = match &track["id"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err("Value 'added_at' in field 'items' in response data is an unexpected type.".to_string())
                                }
                            };
                            let name = match &track["name"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err("Value 'added_at' in field 'items' in response data is an unexpected type.".to_string())
                                }
                            };
                            let duration = match &track["duration_ms"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        res.as_u64().expect("You shouldn't see this error message")
                                    } else {
                                        return Err(format!("Value 'total' in field 'tracks' of playlist {} in response data is not a positive integer.", name))
                                    }
                                },
                                _ => {
                                    return Err(format!("Value 'total' in field 'tracks' of playlist {} in response data is an unexpected type.", name))
                                }
                            };
                            let popularity = match &track["popularity"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        let temp = res.as_u64().expect("You shouldn't see this error message");

                                        temp as u8 // this should be fine as popularity must be between 0 and 100
                                    } else {
                                        return Err(format!("Value 'total' in field 'tracks' of playlist {} in response data is not a positive integer.", name))
                                    }
                                },
                                _ => {
                                    return Err(format!("Value 'total' in field 'tracks' of playlist {} in response data is an unexpected type.", name))
                                }
                            };

                            let album_data = match &track["album"] {
                                Value::Object(album) => {
                                    let id = match &album["id"] {
                                        Value::String(res) => res.clone(),
                                        _ => {
                                            return Err(format!("Value 'id' in field 'album' of track {} in response data is an unexpected type.", name))
                                        }
                                    };

                                    let name = match &album["name"] {
                                        Value::String(res) => res.clone(),
                                        _ => {
                                            return Err(format!("Value 'name' in field 'album' of track {} in response data is an unexpected type.", name))
                                        }
                                    };

                                    (id, name)
                                }
                                _ => {
                                    return Err(format!("Value of field 'album' of track {} in response data is an unexpected type.", name))
                                }
                            };

                            let artists = match &track["artists"] {
                                Value::Array(res) => {
                                    let mut map: HashMap<String, String> = HashMap::new();

                                    for artist in res {
                                        let id = match &artist["id"] {
                                            Value::String(res) => res.clone(),
                                            _ => {
                                                return Err(format!("Value 'id' in field 'album' of track {} in response data is an unexpected type.", name))
                                            }
                                        };

                                        let name = match &artist["name"] {
                                            Value::String(res) => res.clone(),
                                            _ => {
                                                return Err(format!("Value 'name' in field 'album' of track {} in response data is an unexpected type.", name))
                                            }
                                        };

                                        map.insert(id, name);
                                    }

                                    map
                                },
                                _ => {
                                    return Err(format!("Value of field 'artists' of track {} in response data is an unexpected type.", name))
                                }
                            };

                            (id, name, duration, album_data.0, album_data.1, artists, popularity)

                        }
                        _ => {
                            return Err("Value 'track' in response data is an unexpected type.".to_string())
                        }
                    };

                    tracks.push(TrackData {
                        id: track_data.0,
                        name: track_data.1,
                        duration: track_data.2,
                        album_name: track_data.3,
                        album_id: track_data.4,
                        artists: track_data.5,
                        added_at,
                        popularity: track_data.6,
                    })
                },
                _ => {
                    return Err("Value in field 'items' in response data is an unexpected type. (1)".to_string())
                }
            }
        }
        Ok(tracks)
    }
}