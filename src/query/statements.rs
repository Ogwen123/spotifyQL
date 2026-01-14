use crate::app_context::AppContext;
use crate::query::condition::{Condition, compute_conditions};
use crate::query::data::{AlbumData, KeyAccess, PlaylistData, TrackData};
use crate::query::tokenise::DataSource;

#[derive(Debug)]
pub enum Aggregation {
    Count,
    Average,
    None,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub aggregation: Aggregation,
    pub targets: Vec<String>, // list of attribute names
    pub source: DataSource,
    pub conditions: Option<Condition>,
}

impl SelectStatement {
    pub fn run(&self, cx: &AppContext) -> Result<(), String> {
        // gather targets
        match &self.source {
            DataSource::Playlists => {
                self.playlists(match &cx.data.playlist_data {
                    Some(playlists) => playlists.clone(),
                    None => return Err("Playlist data not fetched.".to_string()),
                })?;

                // display results
            }
            DataSource::SavedAlbums => {
                self.albums(match &cx.data.saved_album_data {
                    Some(albums) => albums.clone(),
                    None => return Err("Playlist data not fetched.".to_string()),
                })?;

                // display results
            }
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

                for i in valid {
                    println!("{:?}", i.access(self.targets[0].clone()))
                }

                // display results
            }
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

                self.tracks(data.unwrap().clone())?;

                // display results
            }
        }

        // apply conditions
        // apply aggregations

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

    fn playlists(&self, data: Vec<PlaylistData>) -> Result<(), String> {
        for i in data {
            println!("{:?}", i.access(self.targets[0].clone())?)
        }

        Ok(())
    }

    fn albums(&self, data: Vec<AlbumData>) -> Result<(), String> {
        Ok(())
    }
}
