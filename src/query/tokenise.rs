use regex::Regex;
use std::fmt::{Display, Formatter, write};
use std::ops::Add;

#[derive(Clone, PartialEq)]
pub enum Attribute {
    Id,
    Name,
    Artist,
}

impl Attribute {
    fn _match(s: &String) -> Result<Self, ()> {
        match s.as_str() {
            "id" => Ok(Attribute::Id),
            "name" => Ok(Attribute::Name),
            "artist" => Ok(Attribute::Artist),
            _ => Err(()),
        }
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Attribute::Id => "Attribute(Id)",
                Attribute::Name => "Attribute(Name)",
                Attribute::Artist => "Attribute(Artist)",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum DataSource {
    Playlist(String),
    SavedAlbums(String),
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataSource::Playlist(res) => format!("Playlist({})", res),
                DataSource::SavedAlbums(res) => format!("Playlist({})", res),
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum Operator {
    Equals,
    Like,
    NotEquals,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Equals => "Equals",
                Operator::Like => "Like",
                Operator::NotEquals => "NotEquals",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    SELECT,
    COUNT(Attribute),
    FROM,
    WHERE,
    Attribute(Attribute),
    Operator(Operator),
    Source(DataSource),
    Str(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::SELECT => "SELECT".to_string(),
                Token::COUNT(res) => format!("COUNT({})", res),
                Token::FROM => "FROM".to_string(),
                Token::WHERE => "WHERE".to_string(),
                Token::Attribute(res) => format!("Attribute({})", res),
                Token::Operator(res) => format!("Operator({})", res),
                Token::Source(res) => format!("Source({})", res),
                Token::Str(res) => format!("Str({})", res),
            }
        )
    }
}

#[derive(Clone, Debug)]
struct RawToken {
    identifier: String,
    content: Option<String>,
}

impl RawToken {
    fn new() -> Self {
        Self {
            identifier: String::new(),
            content: None,
        }
    }

    fn build_token(self) -> Result<Token, String> {
        match self.identifier.as_str().to_uppercase().as_str() {
            "SELECT" => return Ok(Token::SELECT),
            "COUNT" => {
                let attr = match Attribute::_match(&self.content.unwrap_or("".to_string())) {
                    Ok(res) => res,
                    Err(_) => return Err("Invalid attribute in COUNT.".to_string()),
                };

                return Ok(Token::COUNT(attr));
            }
            "FROM" => return Ok(Token::FROM),
            "WHERE" => return Ok(Token::WHERE),
            "==" => return Ok(Token::Operator(Operator::Equals)),
            "!=" => return Ok(Token::Operator(Operator::NotEquals)),
            "LIKE" => return Ok(Token::Operator(Operator::Like)),
            "PLAYLIST" => {
                return Ok(Token::Source(DataSource::Playlist(
                    self.content.unwrap_or("".to_string()),
                )));
            }
            "ALBUM" => {
                return Ok(Token::Source(DataSource::SavedAlbums(
                    self.content.unwrap_or("".to_string()),
                )));
            }
            _ => {
                // check if it matches an attribute
                match Attribute::_match(&self.identifier) {
                    Ok(res) => return Ok(Token::Attribute(res)),
                    Err(_) => {}
                }
                if self.content.is_some() && self.identifier.len() == 0 {
                    let cont = self.clone().content.unwrap();

                    // check if content is a valid string
                    let str_regex = Regex::new(r"[\w\s]*").map_err(|x| x.to_string())?;

                    if str_regex.is_match(cont.as_str()) {
                        return Ok(Token::Str(cont)); // remove the quotes from the string
                    }
                }
            }
        };

        println!("{:?}", self);
        Err("Found unknown token in input.".to_string())
    }

    fn add_content(&mut self, c: char) {
        if self.content.is_none() {
            return;
        } else {
            self.content = Some(self.content.clone().unwrap() + c.to_string().as_str());
        }
    }
}

fn split_token(s: &String) -> RawToken {
    let split_on = vec!['(', '"'];
    let mut end_on: char = '.';

    let mut split = false;

    let mut rt = RawToken::new();

    for i in s.chars() {
        if split == false {
            if split_on.contains(&i) {
                match i {
                    '(' => end_on = ')',
                    '"' => end_on = '"',
                    _ => {} // this case should never be hit
                }

                split = true;
                rt.content = Some(String::new())
            } else {
                rt.identifier.push(i)
            }
        } else {
            if i == end_on {
                break;
            } else {
                rt.add_content(i);
            }
        }
    }

    rt
}

pub fn tokenise(input: String) -> Result<Vec<Token>, String> {
    let mut letters = input.chars();

    // split
    let mut group: bool = false; // n = nothing, b = bracket, q = quote
    let mut split: Vec<String> = Vec::new();
    let mut buffer: String = String::new();

    while let Some(letter) = letters.next() {
        if letter == ' ' {
            if group == false {
                split.push(buffer);
                buffer = String::new();
            } else {
                buffer.push(letter)
            }
        } else if letter == '(' || letter == '"' {
            group = true;
            buffer.push(letter);
        } else if group == true && (letter == ')' || letter == '"') {
            group = false;
            buffer.push(letter);
        } else {
            buffer.push(letter);
        }
    }
    // clean up buffer contents
    split.push(buffer);

    println!("{:?}", split);
    let mut split_iter = split.iter();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(elem) = split_iter.next() {
        let token: Token = split_token(elem).build_token()?;

        tokens.push(token);
    }

    tokens.clone().iter().for_each(|x| println!("{}", x));

    Ok(tokens)
}
