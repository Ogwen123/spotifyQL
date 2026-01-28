use std::collections::HashMap;
use crate::app_context::AppContext;
use crate::query::condition::{Condition, compute_conditions};
use crate::query::data::{AlbumData, KeyAccess, PlaylistData, TrackData};
use crate::query::display::data_display;
use crate::query::tokenise::{DataSource, Value};

#[derive(Debug)]
pub enum Aggregation {
    Count,
    Average,
    None,
}

#[derive(Clone)]
pub enum AggregationResult {
    Int(i64),
    Float(f64)
}

impl Aggregation {
    pub fn format(&self, attribute: &String) -> String{

        match self {
            Aggregation::Count => format!("COUNT({})", attribute),
            Aggregation::Average => format!("AVERAGE({})", attribute),
            Aggregation::None => "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct SelectStatement {
    pub aggregation: Aggregation,
    pub targets: Vec<String>, // list of attribute names
    pub source: DataSource,
    pub conditions: Option<Condition>,
}

impl SelectStatement {
    pub fn run(self, cx: &AppContext) -> Result<(), String> {
        // gather targets
        match &self.source {
            DataSource::Playlists => {
                let valid = self.playlists(match &cx.data.playlist_data {
                    Some(playlists) => playlists.clone(),
                    None => return Err("Playlist data not fetched.".to_string()),
                })?;

                self.handle_aggregation(valid)?
            },
            DataSource::SavedAlbums => {
                let valid = self.albums(match &cx.data.saved_album_data {
                    Some(albums) => albums.clone(),
                    None => return Err("Playlist data not fetched.".to_string()),
                })?;

                self.handle_aggregation(valid)?
            },
            DataSource::Playlist(res) => {
                let mut data: Option<&Vec<TrackData>> = None;

                match &cx.data.playlist_data {
                    Some(playlists) => {
                        for playlist in playlists {
                            if playlist.name == *res {
                                data = Some(&playlist.tracks);
                                break;
                            }
                        }
                    }
                    None => return Err("Playlist data not fetched.".to_string()),
                };

                if data.is_none() {
                    return Err(format!("No playlist with the name {}.", res));
                }

                let valid = self.tracks(data.unwrap().clone())?;

                self.handle_aggregation(valid)?
            },
            DataSource::SavedAlbum(res) => {
                let mut data: Option<&Vec<TrackData>> = None;

                match &cx.data.saved_album_data {
                    Some(albums) => {
                        for album in albums {
                            if album.name == *res {
                                data = Some(&album.tracks)
                            }
                        }
                    }
                    None => return Err("Playlist data not fetched.".to_string()),
                };

                if data.is_none() {
                    return Err(format!("No saved album with the name {}.", res));
                }

                let valid = self.tracks(data.unwrap().clone())?;

                self.handle_aggregation(valid)?
            }
        };

        // apply aggregations

        Ok(())
    }

    fn handle_aggregation<T>(self, data: Vec<T>) -> Result<(), String> where T: KeyAccess {
        match self.aggregation {
            Aggregation::Count => {
                let mut count_data: HashMap<String, AggregationResult> = HashMap::new();
                let count = AggregationResult::Int(data.len() as i64);

                for i in self.targets {
                    count_data.insert(i, count.clone());
                }

                data_display::aggregation_table(self.aggregation, count_data)
            },
            Aggregation::Average => {
                let mut average_data: HashMap<String, AggregationResult> = HashMap::new();
                let count = data.len() as f64;

                for i in self.targets {
                    let mut total: f64 = 0f64;

                    for j in &data {
                        match j.access(&i)? {
                            Value::Int(res) => {total += res as f64},
                            Value::Float(res) => {total += res},
                            _ => return Err(format!("Cannot average field {} as it is a non-numeric type.", i))
                        };
                    }

                    average_data.insert(i, AggregationResult::Float(total/count));
                }

                data_display::aggregation_table(self.aggregation, average_data)
            },
            Aggregation::None => data_display::table(data, self.targets.clone())?
        }

        Ok(())
    }

    fn tracks(&self, data: Vec<TrackData>) -> Result<Vec<TrackData>, String> {
        let mut valid: Vec<TrackData> = Vec::new();

        for i in data {
            if self.conditions.is_none()
                || compute_conditions(&i, self.conditions.clone().unwrap())?
            {
                valid.push(i);
            }
        }

        Ok(valid)
    }

    fn playlists(&self, data: Vec<PlaylistData>) -> Result<Vec<PlaylistData>, String> {
        let mut valid: Vec<PlaylistData> = Vec::new();

        for i in data {
            if self.conditions.is_none()
                || compute_conditions(&i, self.conditions.clone().unwrap())?
            {
                valid.push(i);
            }
        }

        Ok(valid)
    }

    fn albums(&self, data: Vec<AlbumData>) -> Result<Vec<AlbumData>, String> {
        let mut valid: Vec<AlbumData> = Vec::new();

        for i in data {
            if self.conditions.is_none()
                || compute_conditions(&i, self.conditions.clone().unwrap())?
            {
                valid.push(i);
            }
        }

        Ok(valid)
    }
}
