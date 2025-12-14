use crate::query::tokenise::{Token, tokenise};
use std::io;
use std::io::Write;
use crate::query::run::run_query;

fn exit() {
    std::process::exit(0);
}

pub fn input_loop() -> Result<(), String> {
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
            "/test" => run_query(
                "SELECT COUNT(name) FROM playlist(\"all\") WHERE artist == \"Arctic Monkeys\""
                    .to_string(),
            )
            .map_err(|x| x)?,
            _ => {
                println!("{}", parsed_input);
                run_query(parsed_input.to_string()).map_err(|x| x)?
            }
        };
        
    }
}
