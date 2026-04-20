use crate::app_context::AppContext;
use crate::query::data::{AlbumData, DATA_TTL, PlaylistData, TrackData};
use crate::utils::date::{Date, DateSource};
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
            self.saved_at.format()
        )
    }
}

trait FromCSV {
    fn deserialise(lines: Vec<String>) -> Result<Self, String>
    where
        Self: Sized;
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
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            data.id = split[0].to_string();
            data.name = split[1].to_string();
            data.duration = split[2]
                .parse()
                .map_err(|_| "Could not parse track duration into u64")?;
            data.release_date = Date::from_iso8601(split[3].clone())?;
            data.album_name = split[4].clone();
            data.album_id = split[5].clone();
            data.artists = split[6].clone().split("|").map(|x| x.to_string()).collect();
            data.added_at = Date::from_iso8601(split[7].clone())?;
            data.popularity = split[8]
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
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if split.len() != 8 {
            return Err("Album data CSV line does not contain 8 values.".to_string());
        }

        data.id = split[0].clone();
        data.name = split[1].clone();
        data.track_count = split[2]
            .parse()
            .map_err(|_| "Could not parse track count into a u64.".to_string())?;
        data.popularity = split[3]
            .parse()
            .map_err(|_| "Could not parse popularity into a u64.".to_string())?;
        data.album_type = split[4].clone();
        data.release_date = Date::new(split[5].clone(), DateSource::User)?;
        data.artists = split[6].clone().split("|").map(|x| x.to_string()).collect();
        data.saved_at = Date::new(split[7].clone(), DateSource::User)?;
        data.tracks = <Vec<TrackData> as FromCSV>::deserialise(lines[1..].to_vec())?;

        Ok(data)
    }
}

pub fn load_cache() -> Result<Option<impl Iterator<Item = String>>, String> {
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

    Ok(Some(cache_iter))
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
                println!("breaking");
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
                println!("breaking");
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
