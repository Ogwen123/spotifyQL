use crate::app_context::AppContext;
use crate::query::data::load_data_source;
use crate::query::parse::parse;
use crate::query::tokenise::{tokenise, Token};
use crate::utils::logger::{info, info_nnl, log, success};
use std::io;
use std::io::Write;
use crate::query::statements::SelectStatement;
use crate::ui::tui::{Log, Severity, TUI};

#[derive(PartialEq, Clone)]
pub enum TUIQueryStage {
    NotRunning,
    Queued(String),
    Tokenised(Vec<Token>),
    Parsed(SelectStatement),
    ParsedWithData(SelectStatement),
}

/// This run function is now for CLI mode only, when using a TUI the run flow is integrated in the main loop
pub fn run_query(query: String, cx: &mut AppContext) -> Result<(), String> {

    info_nnl!("Tokenising");
    let tokens: Vec<Token> = tokenise(query)?;
    success!("Processed Tokens");


    if cx.user_config.debug && !cx.user_config.tui {
        info!("Tokens");
        for i in &tokens {
            print!("{}   ", i)
        }
    }

    info_nnl!("Parsing Tokens");
    let statement = parse(tokens)?;
    success!("Parsed Tokens");

    if cx.user_config.debug && !cx.user_config.tui {
        info!("Parsed Statement");
        println!("{:?}", statement)
    }

    info_nnl!("Loading Data");
    io::stdout().flush().unwrap();
    load_data_source(cx, statement.source.clone())?;
    success!("Loaded Data");

    let _ = statement.run(cx, None)?;

    Ok(())
}
