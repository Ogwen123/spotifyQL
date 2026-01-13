use crate::app_context::AppContext;
use crate::query::data::{AlbumData, DataValue, KeyAccess, PlaylistData, TrackData};
use crate::query::tokenise::{DataSource, Logical, Operator, Value};

#[derive(Debug)]
pub enum Aggregation {
    Count,
    Average,
    None,
}

pub type NextCondition = (Logical, Box<Condition>);

#[derive(Debug, Clone)]
pub struct Condition {
    pub attribute: String,
    pub operation: Operator,
    pub value: Value,
    pub next: Option<NextCondition>,
}

impl Condition {
    pub fn add_next_condition(&mut self, logical: Logical, condition: Condition) {
        let mut next: Box<Condition>;

        if self.next.is_none() {
            self.next = Some((logical, Box::new(condition)));
            return;
        } else {
            next = self.next.clone().unwrap().1;
            loop {
                if next.next.is_none() {
                    next.next = Some((logical, Box::new(condition)));
                    break;
                } else {
                    next = next.next.unwrap().1;
                }
            }
        }
    }
}

pub type NextConditionResult = (Logical, Box<ConditionResult>);

#[derive(Clone)]
pub struct ConditionResult {
    pub val: bool,
    pub next: Option<NextConditionResult>,
}

impl ConditionResult {
    pub fn add_next_condition(&mut self, logical: Logical, val: bool) {
        let mut next: Box<ConditionResult>;

        let res = Box::new(ConditionResult { val, next: None });

        if self.next.is_none() {
            self.next = Some((logical, res));
            return;
        } else {
            next = self.next.clone().unwrap().1;
            loop {
                if next.next.is_none() {
                    next.next = Some((logical, res));
                    break;
                } else {
                    next = next.next.unwrap().1;
                }
            }
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
            let is_valid = self.conditions.is_none();

            let mut result_tree: ConditionResult;

            if !is_valid {
                let mut current_condition = self.conditions.clone().unwrap();
                let mut current_op: Logical = Logical::Or;

                // do the first condition outside loop to set up the tree
                let res = i
                    .access(current_condition.attribute)?
                    .compare(current_condition.value, current_condition.operation)?;

                result_tree = ConditionResult {
                    val: res,
                    next: None,
                };

                loop {
                    if let Some(cc) = current_condition.next {
                        current_condition = *(cc.1);
                        current_op = cc.0;
                    } else {
                        break;
                    }

                    let res = i
                        .access(current_condition.attribute)?
                        .compare(current_condition.value, current_condition.operation)?;

                    result_tree.add_next_condition(current_op, res)
                }

                // TODO: write code to collapse result tree and update is_valid
            }

            if is_valid {
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
