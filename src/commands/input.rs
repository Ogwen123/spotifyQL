use crate::api::APIQuery;
use crate::app_context::AppContext;
use crate::query::parse::parse;
use crate::query::run::run_query;
use crate::query::tokenise::tokenise;
use crate::utils::date::{Date, DateSource};
use crate::utils::logger::{error, info, success};
use std::io;
use std::io::Write;

fn exit() {
    success!("Exiting");
    std::process::exit(0);
}

fn input_inner(cx: &mut AppContext, parsed_input: &str) -> Result<(), String> {
    if cx.user_config.debug {
        match parsed_input {
            "exit" | "/exit" | "quit" | "/quit" => {
                exit();
                return Ok(());
            }
            "/test" => {
                info!("testing date parsing");
                let dates = vec!["12/12/25", "12/2025", "2025", "12-12/25", "12-2025", "2025"];
                for i in dates {
                    println!("{} parsed as {:?}", i, Date::new(i.to_string(), DateSource::User))
                }

                info!("testing api querying");
                APIQuery::get_playlists(cx)?;

                info!("testing tokeniser");
                let tokens = tokenise(
                    "SELECT COUNT(name) FROM playlist(all) WHERE artist == \"Arctic Monkeys\";"
                        .to_string(),
                )?;

                info!("testing token parsing");
                println!("{:?}", parse(tokens)?);
            }
            "/testf" => run_query(
                // test full run through
                "SELECT COUNT(name) FROM playlist(All) WHERE \"Arctic Monkeys\" IN artists;".to_string(),
                cx,
                None
            )?,
            "/testd" => run_query(
                // test double attributes
                "SELECT id, name FROM playlist(All);".to_string(),
                cx,
                None
            )?,
            "/testl" => run_query(
                // test list operators
                "SELECT id, name FROM playlist(All) WHERE name in [\"Holiday\", \"Shout\"];".to_string(),
                cx,
                None
            )?,
            "/testlr" => run_query(
                // test list operators reversed
                "SELECT name FROM playlist(All) WHERE \"Arctic Monkeys\" in artists;".to_string(),
                cx,
                None
            )?,
            "/tests" => run_query(
                // test simple query
                "SELECT name FROM playlist(All);".to_string(),
                cx,
                None
            )?,
            "/testa" => println!(
                // test fetching album data
                "{:?}",
                parse(tokenise("SELECT name FROM ALBUMS;".to_string())?)?
            ),
            "/testp" => run_query(
                // test playlist data
                "SELECT name FROM PLAYLIST(All) WHERE name LIKE \"dancefloor\";".to_string(),
                cx,
                None
            )?,
            "/testb" => println!(
                "{:?}",
                parse(tokenise(
                    "SELECT name FROM album WHERE name == true;".to_string()
                )?)?
            ), // test booleans in conditions,
            "/testc" => run_query(
                "SELECT name FROM PLAYLIST(test) WHERE name == \"Shout\" AND id LIKE \"test\" OR id LIKE \"test\" AND id LIKE \"test\";".to_string(),
                cx,
                None
            )?,
            "/testni" => run_query(
                // test NOT IN operator
                "SELECT COUNT(name) FROM PLAYLIST(All) WHERE \"Arctic Monkeys\" NOT IN artists;".to_string(),
                cx,
                None
            )?,
            "/testda" => run_query(
                // test date
                "SELECT name, release_date FROM PLAYLIST(All) WHERE release_date > 7-6-2006;".to_string(),
                cx,
                None
            )?,
            _ => {
                run_query(parsed_input.to_string(), cx, None)?
            }
        }
    } else {
        match parsed_input {
            "exit" | "/exit" | "quit" | "/quit" => {
                exit();
                return Ok(());
            }
            _ => run_query(parsed_input.to_string(), cx, None)?,
        }
    }

    Ok(())
}

pub fn input_loop(cx: &mut AppContext) -> Result<(), String> {
    loop {
        // take input
        let mut input: String = String::new();

        print!(":: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input = input.trim();

        match input_inner(cx, parsed_input) {
            Ok(_) => {}
            Err(err) => error!("{}", err),
        };
    }
}
