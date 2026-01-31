use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::{tokenise::DataSource, tokenise::Value as DValue};
use crate::utils::utils::secs_now;
use serde_json::Value;
use std::fmt::Display;

pub const DATA_TTL: u64 = 60 * 30;

pub trait KeyAccess {
    fn access<T>(&self, key: T) -> Result<DValue, String>
    where
        T: AsRef<str> + Display;

    fn attributes() -> Vec<String>;
}

#[derive(Clone, Debug)]
pub struct TrackData {
    pub id: String,
    pub name: String,
    pub duration: u64,
    pub album_name: String,
    pub album_id: String,
    pub artists: Vec<String>,
    pub added_at: String,
    pub popularity: u8, // value between 0 and 100
}

impl KeyAccess for TrackData {
    fn access<T>(&self, key: T) -> Result<DValue, String>
    where
        T: AsRef<str> + Display,
    {
        match key.as_ref() {
            "id" => Ok(DValue::Str(self.id.clone())),
            "name" => Ok(DValue::Str(self.name.clone())),
            "duration" => Ok(DValue::Int(self.duration.cast_signed())),
            "album_name" => Ok(DValue::Str(self.album_name.clone())),
            "album_id" => Ok(DValue::Str(self.album_id.clone())),
            "artists" => Ok(DValue::List(
                self.artists
                    .clone()
                    .into_iter()
                    .map(|x| DValue::Str(x))
                    .collect(),
            )),
            "added_at" => Ok(DValue::Str(self.added_at.clone())),
            "popularity" => Ok(DValue::Int(self.popularity.cast_signed().into())),
            _ => Err(format!(
                "SYNTAX ERROR: {} is not a valid attribute for track data.",
                key
            )),
        }
    }

    fn attributes() -> Vec<String> {
        vec![
            "id",
            "name",
            "duration",
            "album_name",
            "album_id",
            "artists",
            "added_at",
            "popularity",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
    }
}

#[derive(Clone, Debug)]
pub struct PlaylistData {
    pub id: String,
    pub name: String,
    pub tracks: Vec<TrackData>,
    pub tracks_api: String,
    pub track_count: u64,
}

impl KeyAccess for PlaylistData {
    fn access<T>(&self, key: T) -> Result<DValue, String>
    where
        T: AsRef<str> + Display,
    {
        match key.as_ref() {
            "id" => Ok(DValue::Str(self.id.clone())),
            "name" => Ok(DValue::Str(self.name.clone())),
            "tracks_api" => Ok(DValue::Str(self.tracks_api.clone())),
            "track_count" => Ok(DValue::Int(self.track_count.clone().cast_signed())),
            _ => Err(format!(
                "SYNTAX ERROR: {} is not a valid attribute for playlist data.",
                key
            )),
        }
    }

    fn attributes() -> Vec<String> {
        vec!["id", "name", "tracks_api", "track_count"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
}

#[derive(Clone, Debug)]
pub struct AlbumData {
    pub id: String,
    pub name: String,
    pub track_count: u64,
    pub tracks: Vec<TrackData>,
    pub popularity: u8, // value between 0 and 100
    pub album_type: String,
    pub release_date: String,
    pub artists: Vec<String>,
    pub saved_at: String,
}

impl KeyAccess for AlbumData {
    fn access<T>(&self, key: T) -> Result<DValue, String>
    where
        T: AsRef<str> + Display,
    {
        match key.as_ref() {
            "id" => Ok(DValue::Str(self.id.clone())),
            "name" => Ok(DValue::Str(self.name.clone())),
            "track_count" => Ok(DValue::Int(self.track_count.cast_signed())),
            "popularity" => Ok(DValue::Int(self.popularity.cast_signed().into())),
            "album_type" => Ok(DValue::Str(self.album_type.clone())),
            "release_date" => Ok(DValue::Str(self.release_date.clone())),
            "artists" => Ok(DValue::List(
                self.artists
                    .clone()
                    .into_iter()
                    .map(|x| DValue::Str(x))
                    .collect(),
            )),
            "saved_at" => Ok(DValue::Str(self.saved_at.clone())),
            _ => Err(format!(
                "SYNTAX ERROR: {} is not a valid attribute for album data.",
                key
            )),
        }
    }

    fn attributes() -> Vec<String> {
        vec![
            "id",
            "name",
            "track_count",
            "popularity",
            "album_type",
            "release_date",
            "artists",
            "saved_at",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
    }
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

impl Default for Data {
    fn default() -> Self {
        Self {
            playlist_data_ct: 0,
            playlist_data: None,
            saved_album_data_ct: 0,
            saved_album_data: None,
        }
    }
}

pub fn load_data_source(cx: &mut AppContext, source: DataSource) -> Result<(), String> {
    //check playlist data
    match source {
        DataSource::Playlist(_) | DataSource::Playlists => {
            let mut load = false;

            if cx.data.playlist_data.is_some() {
                if cx.data.playlist_data_ct + DATA_TTL < secs_now() {
                    load = true;
                }
            } else {
                load = true;
            }

            if load {
                cx.data.playlist_data = Some(APIQuery::get_playlists(cx)?);
                cx.data.playlist_data_ct = secs_now();
            }
        }
        DataSource::SavedAlbum(_) | DataSource::SavedAlbums => {
            let mut load = false;

            if cx.data.saved_album_data.is_some() {
                if cx.data.saved_album_data_ct + DATA_TTL < secs_now() {
                    load = true;
                }
            } else {
                load = true;
            }

            if load {
                cx.data.saved_album_data = Some(APIQuery::get_saved_albums(cx)?);
                cx.data.saved_album_data_ct = secs_now();
            }
        }
    }

    Ok(())
}

/// Extract data with complete error handling
pub mod result_parser {
    use crate::query::data::{AlbumData, PlaylistData, TrackData};
    use serde_json::Value;

    pub fn parse_playlists(str_data: String) -> Result<Vec<PlaylistData>, String> {
        let mut playlists: Vec<PlaylistData> = Vec::new();
        let val: Value = serde_json::from_str(str_data.as_str()).map_err(|x| x.to_string())?;

        let raw_playlists: Vec<Value>;

        if let Value::Array(pl) = &val["items"] {
            raw_playlists = pl.clone();
        } else {
            return Err("'items' field in response data is an unexpected type. (1)".to_string());
        }

        for i in raw_playlists {
            match i {
                Value::Object(obj) => {
                    let id = match &obj["id"] {
                        Value::String(res) => res.clone(),
                        _ => return Err(
                            "Value 'id' in field 'items' in response data is an unexpected type."
                                .to_string(),
                        ),
                    };
                    let name = match &obj["name"] {
                        Value::String(res) => res.clone(),
                        _ => return Err(
                            "Value 'name' in field 'items' in response data is an unexpected type."
                                .to_string(),
                        ),
                    };
                    let track_data = match &obj["tracks"] {
                        Value::Object(tracks_obj) => {
                            let api = match &tracks_obj["href"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!(
                                        "Value 'href' in field 'tracks' of playlist {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            let total = match &tracks_obj["total"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        res.as_u64().expect("You shouldn't see this error message")
                                    } else {
                                        return Err(format!(
                                            "Value 'total' in field 'tracks' of playlist {} in response data is not a positive integer.",
                                            name
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(format!(
                                        "Value 'total' in field 'tracks' of playlist {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            (api, total)
                        }
                        _ => {
                            return Err(format!(
                                "Value of field 'tracks' of playlist {} in response data is an unexpected type.",
                                name
                            ));
                        }
                    };

                    playlists.push(PlaylistData {
                        id,
                        name,
                        tracks: Vec::new(),
                        tracks_api: track_data.0,
                        track_count: track_data.1,
                    })
                }
                _ => {
                    return Err(
                        "Value in field 'items' in response data is an unexpected type."
                            .to_string(),
                    );
                }
            }
        }
        Ok(playlists)
    }

    pub fn parse_albums(str_data: String) -> Result<Vec<AlbumData>, String> {
        let mut albums: Vec<AlbumData> = Vec::new();
        let val: Value = serde_json::from_str(str_data.as_str()).map_err(|x| x.to_string())?;

        let raw_albums: Vec<Value>;

        if let Value::Array(pl) = &val["items"] {
            raw_albums = pl.clone();
        } else {
            return Err("'items' field in response data is an unexpected type. (2)".to_string());
        }

        for i in raw_albums {
            match i {
                Value::Object(obj) => {
                    let added_at = match &obj["added_at"] {
                        Value::String(res) => res.clone(),
                        _ => return Err(
                            "Value 'added_at' in field 'items' in response data is an unexpected type."
                                .to_string(),
                        ),
                    };
                    let album_data = match &obj["album"] {
                        Value::Object(album_obj) => {
                            let name = match &album_obj["name"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(
                                        "Value 'name' in field 'tracks' of album response data is an unexpected type.".to_string()
                                    );
                                }
                            };
                            let id = match &album_obj["id"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!(
                                        "Value 'id' in field 'album' of album {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };

                            let track_count = match &album_obj["total_tracks"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        res.as_u64().expect("You shouldn't see this error message")
                                    } else {
                                        return Err(format!(
                                            "Value 'total' in field 'tracks' of playlist {} in response data is not a positive integer.",
                                            name
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(format!(
                                        "Value 'total_tracks' in field 'album' of album {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            let popularity = match &album_obj["popularity"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        let temp = res
                                            .as_u64()
                                            .expect("You shouldn't see this error message");

                                        temp as u8 // this should be fine as popularity must be between 0 and 100
                                    } else {
                                        return Err(format!(
                                            "Value 'popularity' in field 'album' of album {} in response data is not a positive integer.",
                                            name
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(format!(
                                        "Value 'popularity' in field 'album' of album {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            let album_type = match &album_obj["album_type"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!(
                                        "Value 'album_type' in field 'album' of album {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };

                            let artists = match &album_obj["artists"] {
                                Value::Array(res) => {
                                    let mut map: Vec<String> = Vec::new();

                                    for artist in res {
                                        let name = match &artist["name"] {
                                            Value::String(res) => res.clone(),
                                            _ => {
                                                return Err(format!(
                                                    "Value 'name' in field 'artists' of track {} in response data is an unexpected type.",
                                                    name
                                                ));
                                            }
                                        };

                                        map.push(name);
                                    }

                                    map
                                }
                                _ => {
                                    return Err(format!(
                                        "Value of field 'artists' of track {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            let release_date = match &album_obj["release_date"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!(
                                        "Value 'release_date' in field 'album' of album {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            (
                                id,
                                name,
                                track_count,
                                popularity,
                                album_type,
                                release_date,
                                artists,
                            )
                        }
                        _ => {
                            return Err(
                                "Value of field 'tracks' of album response data is an unexpected type.".to_string(),
                                
                            );
                        }
                    };

                    albums.push(AlbumData {
                        id: album_data.0,
                        name: album_data.1,
                        track_count: album_data.2,
                        tracks: Vec::new(),
                        popularity: album_data.3,
                        album_type: album_data.4,
                        release_date: album_data.5,
                        artists: album_data.6,
                        saved_at: added_at,
                    })
                }
                _ => {
                    return Err(
                        "Value in field 'items' in response data is an unexpected type."
                            .to_string(),
                    );
                }
            }
        }
        Ok(albums)
    }

    pub fn parse_tracks(
        str_data: String,
        _debug_parent_id: &String,
    ) -> Result<Vec<TrackData>, String> {
        let mut tracks: Vec<TrackData> = Vec::new();
        let val: Value = serde_json::from_str(str_data.as_str()).map_err(|x| x.to_string())?;

        let raw_tracks: Vec<Value>;

        if let Value::Array(pl) = &val["items"] {
            raw_tracks = pl.clone();
        } else {
            return Err("'items' field in response data is an unexpected type. (3)".to_string());
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
                                    return Err(format!(
                                        "Value 'id' in field 'track' of tracks '{}' in response data is an unexpected type.",
                                        _debug_parent_id
                                    ));
                                }
                            };
                            let name = match &track["name"] {
                                Value::String(res) => res.clone(),
                                _ => {
                                    return Err(format!(
                                        "Value 'name' in field 'track' of tracks '{}' in response data is an unexpected type.",
                                        _debug_parent_id
                                    ));
                                }
                            };
                            let duration = match &track["duration_ms"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        res.as_u64().expect("You shouldn't see this error message")
                                    } else {
                                        return Err(format!(
                                            "Value 'duration_ms' in field 'track' of tracks '{}' in response data is not a positive integer.",
                                            _debug_parent_id
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(format!(
                                        "Value 'duration_ms' in field 'track' of playlist {} in response data is an unexpected type.",
                                        _debug_parent_id
                                    ));
                                }
                            };
                            let popularity = match &track["popularity"] {
                                Value::Number(res) => {
                                    if res.is_u64() {
                                        let temp = res
                                            .as_u64()
                                            .expect("You shouldn't see this error message");

                                        temp as u8 // this should be fine as popularity must be between 0 and 100
                                    } else {
                                        return Err(format!(
                                            "Value 'popularity' in field 'track' of tracks '{}' in response data is not a positive integer.",
                                            _debug_parent_id
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(format!(
                                        "Value 'popularity' in field 'track' of tracks '{}' in response data is an unexpected type.",
                                        _debug_parent_id
                                    ));
                                }
                            };
                            let album_data = match &track["album"] {
                                Value::Object(album) => {
                                    let id = match &album["id"] {
                                        Value::String(res) => res.clone(),
                                        _ => {
                                            return Err(
                                                "Value 'id' in field 'album' of field 'track' is an unexpected type.".to_string(),
                                            );
                                        }
                                    };

                                    let name = match &album["name"] {
                                        Value::String(res) => res.clone(),
                                        _ => {
                                            return Err("Value 'name' in field 'album' of field 'track' is an unexpected type.".to_string());
                                        }
                                    };

                                    (id, name)
                                }
                                _ => {
                                    return Err(format!(
                                        "Value of field 'album' of track {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };
                            let artists = match &track["artists"] {
                                Value::Array(res) => {
                                    let mut map: Vec<String> = Vec::new();

                                    for artist in res {
                                        let name = match &artist["name"] {
                                            Value::String(res) => res.clone(),
                                            _ => {
                                                return Err(format!(
                                                    "Value 'name' in field 'artists' of track {} in response data is an unexpected type.",
                                                    name
                                                ));
                                            }
                                        };

                                        map.push(name);
                                    }

                                    map
                                }
                                _ => {
                                    return Err(format!(
                                        "Value of field 'artists' of track {} in response data is an unexpected type.",
                                        name
                                    ));
                                }
                            };

                            (
                                id,
                                name,
                                duration,
                                album_data.0,
                                album_data.1,
                                artists,
                                popularity,
                            )
                        }
                        _ => {
                            return Err(
                                "Value 'track' in response data is an unexpected type.".to_string()
                            );
                        }
                    };

                    tracks.push(TrackData {
                        id: track_data.0,
                        name: track_data.1,
                        duration: track_data.2,
                        album_id: track_data.3,
                        album_name: track_data.4,
                        artists: track_data.5,
                        added_at,
                        popularity: track_data.6,
                    })
                }
                _ => {
                    return Err(
                        "Value in field 'items' in response data is an unexpected type. (1)"
                            .to_string(),
                    );
                }
            }
        }
        Ok(tracks)
    }
}
