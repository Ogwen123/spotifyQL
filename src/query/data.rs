use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::tokenise::{DataSource, Token};
use crate::utils::utils::secs_now;

pub const DATA_TTL: u64 = 60*30;

#[derive(Clone, Debug)]
pub struct TrackData {
    pub name: String,
    pub artist_id: String,
    pub artist_name: String
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
