use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::{tokenise::DataSource, value::Value as DValue};
use crate::utils::date::Date;
use crate::utils::file::{write_file, File as _File, WriteMode};
use crate::utils::logger::info;
use crate::utils::utils::secs_now;
use std::fmt::Display;
use std::fs::File;
use crate::utils::file::File as FileType;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub const DATA_TTL: u64 = 60 * 30;

pub trait KeyAccess {
    fn access<T>(&self, key: T) -> Result<DValue, String>
    where
        T: AsRef<str> + Display;

    fn attributes() -> Vec<String>;
}

trait ToCSV {
    fn csv(&self) -> String;
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

impl ToCSV for TrackData {
    fn csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{}",
            self.id,
            self.name,
            self.duration,
            self.release_date.format(),
            self.album_name,
            self.album_id,
            self.artists.join("|"), // connected with pipes to not interfere with over CSV
            self.added_at.format(),
            self.popularity
        )
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

impl ToCSV for PlaylistData {
    fn csv(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.id, self.name, self.tracks_api, self.track_count
        )
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

impl ToCSV for AlbumData {
    fn csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.id,
            self.name,
            self.track_count,
            self.popularity,
            self.album_type,
            self.release_date.format(),
            self.artists.join("|"),
            self.saved_at
        )
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
    fn count_cache_lines(&self) -> usize {
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

fn load_cache() -> Result<Option<String>, String> {
    let Ok(cache_file) = File::open(_File::Cache.path()?) else {
        return Ok(None);
    };
    let cache_file_reader = BufReader::new(cache_file);

    let mut cache_iter = cache_file_reader
        .lines()
        .map(|x| x.expect("Failed to read line."));

    let Some(epoch_line) = cache_iter.next() else {
        return Err("Could not read cache epoch line.".to_string());
    };

    let epoch = u64::from_str(epoch_line.as_str())
        .map_err(|x| format!("Could not parse cache epoch ({})", x))?;

    if epoch + DATA_TTL < secs_now() {
        return Ok(None);
    }

    Ok(Some(cache_iter.collect()))
}

#[derive(Default)]
struct DeserialisedCache {
    playlists: Vec<PlaylistData>,
    albums: Vec<AlbumData>,
}

enum DataType {
    Playlist,
    Album,
}

/// Doesn't do that much error checking, relies on the format being correct
fn deserialise_cache(data: String) -> Result<DeserialisedCache, String> {
    let mut data_iter = data.split("\n");

    let mut playlists: Vec<PlaylistData> = Vec::new();
    let mut albums: Vec<AlbumData> = Vec::new();

    let mut currently_reading: DataType = DataType::Playlist;

    loop {
        let line = match data_iter.next() {
            Some(res) => res,
            None => break,
        };
        match line.split(" ").next().unwrap() {
            // there must be at least one item in the iter
            "ALBUM" => currently_reading = DataType::Album,
            "PLAYLIST" => currently_reading = DataType::Playlist,
            _ => return Err("Unknown block identifier reached".to_string()),
        };
        break;
    }
    Ok(DeserialisedCache { playlists, albums })
}

/// Cache format
/// PLAYLIST
/// <playlist data as csv>
/// <track 1 data as csv>
/// <track 2 data as csv>
/// ...
fn serialise_cache(cx: &AppContext) -> Result<String, String> {
    let pd = cx.data.playlist_data.clone();
    let ad = cx.data.saved_album_data.clone();

    let count= cx.data.count_cache_lines();
    let mut write_buffer: Vec<String> = Vec::with_capacity(count);

    write_buffer.push(secs_now().to_string());

    if pd.is_some() {
        for i in pd.unwrap() {
            let mut buf: Vec<String> = vec!["PLAYLIST".to_string(), i.csv()];
            for j in i.tracks {
                buf.push(j.csv())
            }
            write_buffer.append(&mut buf);
        }
    }

    if ad.is_some() {
        for i in ad.unwrap() {
            let mut buf: Vec<String> = vec!["ALBUM".to_string(), i.csv()];
            for j in i.tracks {
                buf.push(j.csv())
            }
            write_buffer.append(&mut buf);
        }
    }

    Ok(write_buffer.join("\n"))
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
