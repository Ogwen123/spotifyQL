use std::io;
use std::io::Write;
use crate::query::tokenise::{tokenise, Tokens};

fn exit() {
    std::process::exit(0);
}

pub fn input_loop() {
    loop {
        // take input
        let mut input: String = String::new();

        print!(":: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input = input.trim();
        let tokens: Vec<Tokens> = match parsed_input {
            "/exit" => {
                exit();
                return
            },
            "/test" => tokenise("SELECT COUNT(name) FROM playlist WHERE artist == \"Arctic Monkeys\"".to_string()),
            _ => {
                println!("{}", parsed_input);
                tokenise(parsed_input.to_string())
            }
        };
        // process input
        // perform api request
        // process data from api
        // format and print
    }
}