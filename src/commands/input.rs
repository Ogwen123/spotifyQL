use crate::api::APIQuery;
use crate::config::app_config::AppContext;
use crate::query::run::run_query;
use crate::query::tokenise::{Token, tokenise};
use crate::utils::logger::info;
use std::io;
use std::io::Write;

fn exit() {
    std::process::exit(0);
}

pub fn input_loop(cx: &mut AppContext) -> Result<(), String> {
    loop {
        // take input
        let mut input: String = String::new();

        print!(":: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input = input.trim();
        match parsed_input {
            "/exit" => {
                exit();
                return Ok(());
            }
            "/test" => {
                info!("testing tokeniser");
                tokenise(
                    "SELECT COUNT(name) FROM playlist(\"all\") WHERE artist == \"Arctic Monkeys\""
                        .to_string(),
                )?;

                info!("testing api querying");
                APIQuery::get_playlists(cx, None, None);
            }
            "/testf" => run_query(
                cx,
                "SELECT COUNT(name) FROM playlist(\"all\") WHERE artist == \"Arctic Monkeys\""
                    .to_string(),
            )?,
            _ => {
                println!("{}", parsed_input);
                run_query(cx, parsed_input.to_string())?
            }
        };
    }
}
