use crate::app_context::AppContext;
use crate::query::data::load_data_source;
use crate::query::parse::parse;
use crate::query::tokenise::tokenise;
use crate::utils::logger::{info, info_nnl, log, success};
use std::io;
use std::io::Write;
use crate::ui::tui::{Log, Severity, TUI};

pub fn run_query(query: String, cx: &mut AppContext, mut window: Option<&mut TUI>) -> Result<(), String> {

    log!(info_nnl!("Tokenising"), Log {severity: Severity::Log, content: "Tokenising".to_string()}, window);
    let tokens = tokenise(query)?;
    log!(success!("Processed Tokens"), Log {severity: Severity::Success, content: "Processed Tokens".to_string()}, window);


    if cx.user_config.debug && !cx.user_config.tui {
        info!("Tokens");
        for i in &tokens {
            print!("{}   ", i)
        }
    }

    log!(info_nnl!("Parsing Tokens"), Log {severity: Severity::Log, content: "Parsing Tokens".to_string()}, window);
    let statement = parse(tokens)?;
    log!(success!("Parsed Tokens"), Log {severity: Severity::Success, content: "Parsed Tokens".to_string()}, window);

    if cx.user_config.debug && !cx.user_config.tui {
        info!("Parsed Statement");
        println!("{:?}", statement)
    }

    log!(info_nnl!("Loading Data"), Log {severity: Severity::Log, content: "Loading Data".to_string()}, window);
    io::stdout().flush().unwrap();
    load_data_source(cx, statement.source.clone())?;
    log!(success!("Loaded Data"), Log {severity: Severity::Success, content: "Loaded Data".to_string()}, window);

    let _ = statement.run(cx, window)?;

    Ok(())
}
