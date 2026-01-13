use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// TOKEN ENUMS
// #[derive(Clone, PartialEq, Debug)]
// pub enum Attribute {
//     Id,
//     Name,
//     Artist,
// }
//
// impl Attribute {
//     fn _match(s: &String) -> Result<Self, ()> {
//         match s.as_str() {
//             "id" => Ok(Attribute::Id),
//             "name" => Ok(Attribute::Name),
//             "artist" => Ok(Attribute::Artist),
//             _ => Err(()),
//         }
//     }
// }
//
// impl Display for Attribute {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Attribute::Id => "Attribute(Id)",
//                 Attribute::Name => "Attribute(Name)",
//                 Attribute::Artist => "Attribute(Artist)",
//             }
//         )
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub enum DataSource {
    Playlist(String),
    Playlists, // all playlists
    SavedAlbum(String),
    SavedAlbums, // all saved albums
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataSource::Playlist(res) => format!("Playlist({})", res),
                DataSource::Playlists => "Playlists".to_string(),
                DataSource::SavedAlbum(res) => format!("SavedAlbum({})", res),
                DataSource::SavedAlbums => "SavedAlbums".to_string(),
            }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub enum Logical {
    And,
    Or,
}

impl Display for Logical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Logical::And => "And",
                Logical::Or => "Or",
            }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f32),
    Bool(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Str(res) => format!("Str({})", res),
                Value::Int(res) => format!("Int({})", res),
                Value::Float(res) => format!("Float({})", res),
                Value::Bool(res) => format!("Bool({})", res),
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    SELECT,
    COUNT(String),
    AVERAGE(String),
    FROM,
    WHERE,
    Attribute(String),
    Operator(Operator),
    Logical(Logical),
    Source(DataSource),
    Value(Value),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::SELECT => "SELECT".to_string(),
                Token::COUNT(res) => format!("COUNT({})", res),
                Token::AVERAGE(res) => format!("AVERAGE({})", res),
                Token::FROM => "FROM".to_string(),
                Token::WHERE => "WHERE".to_string(),
                Token::Attribute(res) => format!("Attribute({})", res),
                Token::Operator(res) => format!("Operator({})", res),
                Token::Logical(res) => format!("Logical({})", res),
                Token::Source(res) => format!("Source({})", res),
                Token::Value(res) => format!("Value({})", res),
            }
        )
    }
}

// TOKEN PROCESSING
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
                let attr = self.content.unwrap_or("".to_string());

                return Ok(Token::COUNT(attr));
            }
            "AVERAGE" => {
                let attr = self.content.unwrap_or("".to_string());

                return Ok(Token::AVERAGE(attr));
            }
            "FROM" => return Ok(Token::FROM),
            "WHERE" => return Ok(Token::WHERE),
            "==" => return Ok(Token::Operator(Operator::Equals)),
            "!=" => return Ok(Token::Operator(Operator::NotEquals)),
            "LIKE" => return Ok(Token::Operator(Operator::Like)),
            "AND" => return Ok(Token::Logical(Logical::And)),
            "OR" => return Ok(Token::Logical(Logical::Or)),
            "PLAYLIST" => {
                return Ok(Token::Source(DataSource::Playlist(
                    self.content.unwrap_or("".to_string()),
                )));
            }
            "ALBUM" => {
                return Ok(Token::Source(DataSource::SavedAlbum(
                    self.content.unwrap_or("".to_string()),
                )));
            },
            "PLAYLISTS" => {
                return Ok(Token::Source(DataSource::Playlists));
            }
            "ALBUMS" => {
                return Ok(Token::Source(DataSource::SavedAlbums));
            }
            _ => {
                let int_regex = Regex::new(r"^-?\d+$").map_err(|x| x.to_string())?;
                let float_regex = Regex::new(r"^-?\d+.\d+$").map_err(|x| x.to_string())?;
                let bool_regex = Regex::new(r"^true|false$").map_err(|x| x.to_string())?;
                let str_regex = Regex::new(r"^[\w\s]+$").map_err(|x| x.to_string())?;

                if bool_regex.is_match(&self.identifier.as_str()) {
                    return Ok(Token::Value(Value::Bool(self.identifier == "true")));
                }

                if int_regex.is_match(self.identifier.as_str()) {
                    return Ok(Token::Value(Value::Int(
                        i64::from_str(self.identifier.as_str())
                            .map_err(|x| format!("INT ERROR: {}", x.to_string()))?,
                    )));
                }

                if float_regex.is_match(self.identifier.as_str()) {
                    return Ok(Token::Value(Value::Float(
                        f32::from_str(self.identifier.as_str())
                            .map_err(|x| format!("FLOAT ERROR: {}", x.to_string()))?,
                    )));
                }

                // if the identifier is a string then it is an attribute string, otherwise it is a value string
                if str_regex.is_match(self.identifier.as_str()) {
                    return Ok(Token::Attribute(self.identifier)); // remove the quotes from the string
                }

                if self.content.is_some() && self.identifier.len() == 0 {
                    let cont = self.clone().content.unwrap();

                    if str_regex.is_match(cont.as_str()) {
                        return Ok(Token::Value(Value::Str(cont))); // remove the quotes from the string
                    }
                }
            }
        };

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
    let mut group: bool = false;
    let mut split: Vec<String> = Vec::new();
    let mut buffer: String = String::new();
    let mut terminated: bool = false;

    let mut end_on = '.';

    while let Some(letter) = letters.next() {
        if letter == ';' {
            terminated = true;
            // clean up buffer contents
            split.push(buffer);
            break;
        }

        if letter == ' ' {
            if group == false {
                split.push(buffer);
                buffer = String::new();
            } else {
                buffer.push(letter)
            }
        } else if letter == ',' {
            // only add a comma if it is part of a string
            if group {
                buffer.push(letter)
            }
        } else if group == false && (letter == '(' || letter == '"') {
            group = true;
            match letter {
                '(' => end_on = ')',
                '"' => end_on = '"',
                _ => {}
            }
            buffer.push(letter);
        } else if group == true && letter == end_on {
            group = false;
            buffer.push(letter);
        } else {
            buffer.push(letter);
        }
    }

    if terminated == false {
        return Err("Input must be terminated with ';'.".to_string());
    }

    let mut split_iter = split.iter();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(elem) = split_iter.next() {
        let temp = split_token(elem);
        let token: Token = temp.build_token()?;

        tokens.push(token);
    }

    Ok(tokens)
}
