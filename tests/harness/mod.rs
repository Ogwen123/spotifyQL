use std::io;
use std::io::{Read, Write};
use gag::{BufferRedirect, Gag};
use spotifyQL::app_context::AppContext;
use spotifyQL::query::data::load_data_source;
use spotifyQL::query::parse::parse;
use spotifyQL::query::tokenise::{tokenise, Token};

pub fn query_harness(query: String) -> Result<String, String> {
    let cx: &mut AppContext = &mut AppContext::load()?;

    cx.user_config.debug = false;
    cx.user_config.tui = false;

    let tokens: Vec<Token> = tokenise(query.clone())?;

    let statement = parse(tokens)?;

    io::stdout().flush().unwrap();
    load_data_source(cx, statement.source.clone(), None)?;

    let mut buf = BufferRedirect::stdout().unwrap();

    let _ = statement.run(cx, None, query)?;

    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();

    drop(buf);

    Ok(output)
}