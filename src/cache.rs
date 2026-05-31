use crate::app_context::AppContext;
use crate::query::data::{AlbumData, DATA_TTL, PlaylistData, TrackData};
use crate::utils::date::{Date, DateSource};
use crate::utils::file::File as _File;
use crate::utils::utils::secs_now;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use urlencoding::{decode, encode};

pub trait ToCSV {
    fn csv(&self) -> String;
}

impl ToCSV for TrackData {
    fn csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{}",
            encode(self.id.as_str()),
            encode(self.name.as_str()),
            encode(self.duration.to_string().as_str()),
            encode(self.release_date.format().as_str()),
            encode(self.album_name.as_str()),
            encode(self.album_id.as_str()),
            encode(self.artists.join("|").as_str()), // connected with pipes to not interfere with over CSV
            encode(self.added_at.format().as_str()),
            encode(self.popularity.to_string().as_str())
        )
    }
}

impl ToCSV for PlaylistData {
    fn csv(&self) -> String {
        format!(
            "{},{},{},{}",
            encode(self.id.as_str()), encode(self.name.as_str()), encode(self.tracks_api.as_str()), encode(self.track_count.to_string().as_str())
        )
    }
}

impl ToCSV for AlbumData {
    fn csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            encode(self.id.as_str()),
            encode(self.name.as_str()),
            encode(self.track_count.to_string().as_str()),
            encode(self.popularity.to_string().as_str()),
            encode(self.album_type.as_str()),
            encode(self.release_date.format().as_str()),
            encode(self.artists.join("|").as_str()),
            encode(self.saved_at.format().as_str())
        )
    }
}

trait FromCSV {
    fn deserialise(lines: Vec<String>) -> Result<Self, String>
    where
        Self: Sized;
}

macro_rules! decode {
    ($s:expr) => {
        decode($s).map_err(|x| x.to_string())?.to_string()
    };
}

impl FromCSV for Vec<TrackData> {
    fn deserialise(lines: Vec<String>) -> Result<Self, String>
    where
        Self: Sized
    {
        let mut tracks: Vec<TrackData> = Vec::new();
        for line in lines {
            let mut data = TrackData::default();

            let split = line
                .split(",")
                .collect::<Vec<&str>>();

            data.id = decode!(split[0]);
            data.name = decode!(split[1]);
            data.duration = decode!(split[2])
                .parse()
                .map_err(|_| format!("Could not parse track duration into u64 ({})", split[2]))?;
            data.release_date = Date::from_iso8601(decode!(split[3]))?;
            data.album_name = decode!(split[4]);
            data.album_id = decode!(split[5]);
            data.artists = decode!(split[6]).split("|").map(|x| x.to_string()).collect();
            data.added_at = Date::from_iso8601(decode!(split[7]))?;
            data.popularity = decode!(split[8])
                .parse()
                .map_err(|_| "Cloud not parse track popularity into u8")?;

            tracks.push(data)
        }

        Ok(tracks)
    }
}

impl FromCSV for PlaylistData {
    fn deserialise(lines: Vec<String>) -> Result<Self, String> {
        if lines.len() < 1 {
            return Err(
                "Must provide at least the playlist data line when deserialising".to_string(),
            );
        }

        let mut data = PlaylistData::default();

        let split = lines[0]
            .split(",")
            .collect::<Vec<&str>>();

        if split.len() != 4 {
            return Err("Playlist data CSV line does not contain 4 values.".to_string());
        }

        data.id = decode!(split[0]);
        data.name = decode!(split[1]);
        data.tracks_api = decode!(split[2]);
        data.track_count = decode!(split[3])
            .parse()
            .map_err(|_| "Could not parse track count into a u64.".to_string())?;
        data.tracks = <Vec<TrackData> as FromCSV>::deserialise(lines[1..].to_vec())?;

        Ok(data)
    }
}

impl FromCSV for AlbumData {
    fn deserialise(lines: Vec<String>) -> Result<Self, String> {
        if lines.len() < 1 {
            return Err("Must provide at least the album data line when deserialising".to_string());
        }

        let mut data = AlbumData::default();

        let split = lines[0]
            .split(",")
            .collect::<Vec<&str>>();

        if split.len() != 8 {
            return Err("Album data CSV line does not contain 8 values.".to_string());
        }

        data.id = decode!(split[0]);
        data.name = decode!(split[1]);
        data.track_count = decode!(split[2])
            .parse()
            .map_err(|_| "Could not parse track count into a u64.".to_string())?;
        data.popularity = decode!(split[3])
            .parse()
            .map_err(|_| "Could not parse popularity into a u64.".to_string())?;
        data.album_type = decode!(split[4]);
        data.release_date = Date::from_iso8601(decode!(split[5]))?;
        data.artists = split[6].clone().split("|").map(|x| x.to_string()).collect();
        data.saved_at = Date::from_iso8601(decode!(split[7]))?;
        data.tracks = <Vec<TrackData> as FromCSV>::deserialise(lines[1..].to_vec())?;

        Ok(data)
    }
}

pub fn load_cache() -> Result<Option<(impl Iterator<Item = String>, u64)>, String> {
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

    Ok(Some((cache_iter, epoch)))
}

#[derive(Default)]
pub struct DeserialisedCache {
    pub playlists: Vec<PlaylistData>,
    pub albums: Vec<AlbumData>,
}

#[derive(PartialEq)]
enum DataType {
    Playlist,
    Album,
}

/// Doesn't do that much error checking, relies on the format being correct
pub fn deserialise_cache(data: impl Iterator<Item = String>) -> Result<DeserialisedCache, String> {
    let mut data_iter = data.peekable();

    let mut playlists: Vec<PlaylistData> = Vec::new();
    let mut albums: Vec<AlbumData> = Vec::new();

    let mut currently_reading: DataType;

    loop {
        let line = match data_iter.next() {
            Some(res) => res,
            None => {
                break
            },
        };

        let bi = line.split(" ").next().unwrap();
        match bi {
            // there must be at least one item in the iter
            "ALBUM" => currently_reading = DataType::Album,
            "PLAYLIST" => currently_reading = DataType::Playlist,
            _ => return Err(format!("Unknown block identifier reached ({})", bi)),
        };

        let mut lines = Vec::new();
        loop {
            let p = data_iter.peek();
            if p.is_some() && vec!["ALBUM", "PLAYLIST"].contains(&p.clone().unwrap().as_str()) {
                break;
            }

            let data_line = data_iter.next();
            if data_line.is_none() {
                break;
            }

            lines.push(data_line.unwrap().to_string())
        }

        if currently_reading == DataType::Playlist {
            playlists.push(PlaylistData::deserialise(lines)?)
        } else {
            albums.push(AlbumData::deserialise(lines)?)
        }
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
