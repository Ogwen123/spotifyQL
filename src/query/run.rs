use crate::config::app_config::AppContext;
use crate::query::data::build_queries;
use crate::query::tokenise::tokenise;

pub fn run_query(cx: &mut AppContext, query: String) -> Result<(), String> {
    let tokens = tokenise(query)?;

    // a users entire playlist data get downloaded in one go to avoid repeated fetches
    let api_queries = build_queries(cx, tokens);

    for q in api_queries {
        println!("{:?}", q)
    }

    Ok(())
}
