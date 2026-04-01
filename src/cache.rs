use crate::app_context::AppContext;
use crate::query::data::{AlbumData, DATA_TTL, PlaylistData, TrackData};
use crate::utils::file::File as _File;
use crate::utils::utils::secs_now;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub trait ToCSV {
    fn csv(&self) -> String;
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

impl ToCSV for PlaylistData {
    fn csv(&self) -> String {
        format!(
            "{},{},{},{}",
            self.id, self.name, self.tracks_api, self.track_count
        )
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

fn deserialise_track_data(line: String) -> TrackData {
    let data = TrackData::default();

    data
}

trait DeserialiseCache {
    fn deserialise(lines: Vec<String>) -> Result<Self, String>
    where
        Self: Sized;
}

impl DeserialiseCache for PlaylistData {
    fn deserialise(lines: Vec<String>) -> Result<Self, String> {
        if lines.len() < 1 {
            return Err(
                "Must provide at least the playlist data line when deserialising".to_string(),
            );
        }

        let mut data = PlaylistData::default();

        let split = lines[0]
            .split(",")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if split.len() != 4 {
            return Err("Playlist data CSV line does not contain 4 values.".to_string());
        }

        data.id = split[0].clone();
        data.name = split[1].clone();
        data.tracks_api = split[2].clone();
        data.track_count = split[3]
            .parse()
            .map_err(|_| "Could not parse track count into a u64.".to_string())?;

        data.tracks = lines[1..]
            .to_vec()
            .into_iter()
            .map(|x| deserialise_track_data(x))
            .collect::<Vec<TrackData>>();

        Ok(data)
    }
}

impl DeserialiseCache for AlbumData {
    fn deserialise(lines: Vec<String>) -> Result<Self, String> {
        if lines.len() < 1 {
            return Err("Must provide at least the album data line when deserialising".to_string());
        }

        let data = AlbumData::default();

        Ok(data)
    }
}

pub fn load_cache() -> Result<Option<String>, String> {
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

#[derive(PartialEq)]
enum DataType {
    Playlist,
    Album,
}

/// Doesn't do that much error checking, relies on the format being correct
pub fn deserialise_cache(data: String) -> Result<DeserialisedCache, String> {
    let mut data_iter = data.split("\n").peekable();

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

        let mut lines = Vec::new();
        loop {
            let p = data_iter.peek();
            if p.is_some() && vec!["ALBUM", "PLAYLIST"].contains(p.clone().unwrap()) {
                break;
            }

            let data_line = data_iter.next();
            if data_line.is_none() {
                break;
            }

            lines.push(data_line.unwrap())
        }

        if currently_reading == DataType::Playlist {
        } else {
        }
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
pub fn serialise_cache(cx: &AppContext) -> Result<String, String> {
    let pd = cx.data.playlist_data.clone();
    let ad = cx.data.saved_album_data.clone();

    let count = cx.data.count_cache_lines();
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
