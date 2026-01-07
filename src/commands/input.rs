use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::parse::parse;
use crate::query::run::run_query;
use crate::query::tokenise::{Token, tokenise};
use crate::utils::logger::{info, success};
use std::io;
use std::io::Write;

fn exit() {
    success!("Exiting");
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
            "exit" | "/exit" | "quit" | "/quit" => {
                exit();
                return Ok(());
            }
            "/test" => {
                info!("testing api querying");
                APIQuery::get_playlists(cx, None, None)?;

                info!("testing tokeniser");
                let tokens = tokenise(
                    "SELECT COUNT(name) FROM playlist(\"all\") WHERE artist == \"Arctic Monkeys\";"
                        .to_string(),
                )?;

                info!("testing token parsing");
                println!("{:?}", parse(tokens)?);
            }
            "/testf" => run_query(
                cx,
                "SELECT COUNT(name) FROM playlist(\"all\") WHERE artist == \"Arctic Monkeys\";"
                    .to_string(),
            )?,
            "/testd" => run_query(
                // test double attributes
                cx,
                "SELECT id, name FROM playlist(\"all\");".to_string(),
            )?,
            "/testl" => run_query(
                // test logical operators
                cx,
                "SELECT id, name FROM playlist(\"all\") WHERE name == \"test\" AND id == 1;"
                    .to_string(),
            )?,
            "/tests" => run_query(
                // test double attributes
                cx,
                "SELECT name FROM playlist(\"all\");".to_string(),
            )?,
            _ => {
                println!("{}", parsed_input);
                run_query(cx, parsed_input.to_string())?
            }
        };
    }
}
