use crate::api::APIQuery;
use crate::config::app_config::AppContext;
use crate::query::tokenise::{DataSource, Token};

pub fn build_queries(cx: &AppContext, tokens: Vec<Token>) -> Vec<APIQuery> {
    let mut queries: Vec<APIQuery> = Vec::new();

    //check playlist data
    for i in tokens {
        if let Token::Source(src) = i {
            if let DataSource::Playlist(_) = src {
                if cx.data.playlist_data.is_none() {}
            }
        }
    }

    queries
}
