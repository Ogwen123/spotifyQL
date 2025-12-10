use std::io;
use std::io::Write;

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
        match parsed_input {
            "/exit" => exit(),
            _ => println!("{}", parsed_input)
        }
        // process input
        // perform api request
        // process data from api
        // format and print
    }
}