use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::cache::serialise_cache;
use crate::query::{tokenise::DataSource, value::Value as DValue};
use crate::utils::date::Date;
use crate::utils::file::File as FileType;
use crate::utils::file::{WriteMode, write_file};
use crate::utils::logger::info;
use crate::utils::utils::secs_now;
use std::fmt::Display;

pub const DATA_TTL: u64 = 60 * 30;

pub trait KeyAccess {
    fn access<T>(&self, key: T) -> Result<DValue, String>
    where
        T: AsRef<str> + Display;

    fn attributes() -> Vec<String>;
}

#[derive(Clone, Debug, Default)]
pub struct TrackData {
    pub id: String,
    pub name: String,
    pub duration: u64,
    pub release_date: Date,
    pub album_name: String,
    pub album_id: String,
    pub artists: Vec<String>,
    pub added_at: Date,
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
            "release_date" => Ok(DValue::Date(self.release_date.clone())),
            "album_name" => Ok(DValue::Str(self.album_name.clone())),
            "album_id" => Ok(DValue::Str(self.album_id.clone())),
            "artists" => Ok(DValue::List(
                self.artists
                    .clone()
                    .into_iter()
                    .map(|x| DValue::Str(x))
                    .collect(),
            )),
            "added_at" => Ok(DValue::Date(self.added_at.clone())),
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
            "release_date",
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct AlbumData {
    pub id: String,
    pub name: String,
    pub track_count: u64,
    pub tracks: Vec<TrackData>,
    pub popularity: u8, // value between 0 and 100
    pub album_type: String,
    pub release_date: Date,
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
            "release_date" => Ok(DValue::Date(self.release_date.clone())),
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

impl Data {
    pub(crate) fn count_cache_lines(&self) -> usize {
        let mut count: usize = 1;
        if self.playlist_data.is_some() {
            for i in self.playlist_data.clone().clone().unwrap() {
                count += 2;
                count += i.track_count as usize;
            }
        }

        if self.saved_album_data.is_some() {
            for i in self.saved_album_data.clone().unwrap() {
                count += 2;
                count += i.track_count as usize;
            }
        }

        count
    }
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
    // load in cache
    // check for any missing data (e.g. there is only album data but the query needs playlist data
    // if there is missing data fetch correct data
    // overwrite cache with new data

    // if let Some(cache_text) = load_cache()? {
    //     let data = deserialise_cache(cache_text)?;
    // }

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
            if cx.user_config.debug && !cx.user_config.tui {
                info!("Loaded playlist data")
            }
            if cx.user_config.cache {
                let sd = serialise_cache(&cx)?;
                write_file(FileType::Cache, sd, WriteMode::Overwrite)?
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
            if cx.user_config.debug && !cx.user_config.tui {
                info!("Loaded album data")
            }
            if cx.user_config.cache {
                let sd = serialise_cache(&cx)?;
                write_file(FileType::Cache, sd, WriteMode::Overwrite)?
            }
        }
    }

    Ok(())
}
