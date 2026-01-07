use crate::app_context::AppContext;
use crate::query::data::load_data_source;
use crate::query::parse::parse;
use crate::query::tokenise::tokenise;
use crate::utils::logger::{info_nnl, success};
use std::io;
use std::io::Write;

pub fn run_query(cx: &mut AppContext, query: String) -> Result<(), String> {
    info_nnl!("Tokenising");
    let tokens = tokenise(query)?;
    success!("Processed Tokens");

    info_nnl!("Parsing Tokens");
    let statement = parse(tokens)?;
    success!("Parsed Tokens");

    info_nnl!("Loading Data");
    io::stdout().flush().unwrap();
    load_data_source(cx, statement.source.clone())?;

    statement.run();
    success!("Loaded Data");

    Ok(())
}
