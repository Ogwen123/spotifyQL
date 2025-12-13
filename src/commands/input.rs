use crate::query::tokenise::{Token, tokenise};
use std::io;
use std::io::Write;

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
        let tokens: Vec<Token> = match parsed_input {
            "/exit" => {
                exit();
                return Ok(());
            }
            "/test" => tokenise(
                "SELECT COUNT(name) FROM playlist(\"all\") WHERE artist == \"Arctic Monkeys\""
                    .to_string(),
            )
            .map_err(|x| x)?,
            _ => {
                println!("{}", parsed_input);
                tokenise(parsed_input.to_string()).map_err(|x| x)?
            }
        };
        // process input
        // perform api request
        // process data from api
        // format and print
    }
}
