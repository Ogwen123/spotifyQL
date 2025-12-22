use crate::config::app_config::AppContext;
use crate::query::tokenise::tokenise;

pub fn run_query(cx: &mut AppContext, query: String) -> Result<(), String> {
    let tokens = tokenise(query)?;


    Ok(())
}
