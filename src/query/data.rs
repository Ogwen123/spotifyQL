use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::tokenise::{DataSource, Token};
use crate::utils::utils::secs_now;

pub const DATA_TTL: u64 = 60*30;

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
