use std::ops::Add;

#[derive(Clone)]
pub enum Attribute {
    Name,
}

#[derive(Clone)]
pub enum Type {
    Playlist(String),
    Album(String),
}

#[derive(Clone)]
pub enum Operator {
    Equals,
    Like,
    NotEquals,
}

pub enum Tokens {
    SELECT,
    Attribute(Attribute),
    COUNT,
    FROM(Type),
    WHERE(Attribute, Operator, String),
}

pub fn tokenise(input: String) -> Vec<Tokens> {
    let mut split = input.split(" ");

    // regroup strings and brackets together

    let mut looking_for: char = 'n'; // n = nothing, b = bracket, q = quote
    let mut regrouped: Vec<String> = Vec::new();

    while let Some(elem) = split.next() {
        if looking_for == 'n' {
            // check if starts with quote or bracket


            regrouped.push(elem.to_string());
        } else if looking_for == 'q' {

        }
    }

    // convert to tokens
    let mut in_string: bool = false;

    while let Some(elem) = split.next() {
        println!("{}", elem);

        let token: Tokens;

        // determine if the current element is a token or part of an identifier
        if elem.starts_with("\"") {
            in_string = true;
        }

        if in_string {

        } else {

        }

        if elem.ends_with("\"") {
            in_string = false;
        }
    }

    Vec::new()
}
