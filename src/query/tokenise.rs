use crate::query::value::Value;
use crate::utils::date::{Date, DateSource};
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// TOKEN ENUMS
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
pub enum Order {
    Ascending,
    Descending,
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Order::Ascending => "Order(Ascending)",
                Order::Descending => "Order(Descending)",
            }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
    Equals,
    Like,
    NotEquals,
    In,
    NotIn,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
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
                Operator::In => "In",
                Operator::Less => "LessThan",
                Operator::LessEqual => "LessThanOrEqual",
                Operator::Greater => "GreaterThan",
                Operator::GreaterEqual => "GreaterThanOrEqual",
                Operator::NotIn => "NotIn",
            }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Logical {
    And,
    Or,
}

impl Logical {
    pub fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Logical::Or => a || b,
            Logical::And => a && b,
        }
    }
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

#[derive(Clone, PartialEq)]
pub enum Token {
    SELECT,
    AttributeWildcard,
    COUNT(String),
    AVERAGE(String),
    FROM,
    ORDER,
    /// Only ever part of the ORDER BY compound keyword
    BY,
    OrderDirection(Order),
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
                Token::AttributeWildcard => "AllAttributes".to_string(),
                Token::COUNT(res) => format!("COUNT({})", res),
                Token::AVERAGE(res) => format!("AVERAGE({})", res),
                Token::FROM => "FROM".to_string(),
                Token::ORDER => "ORDER".to_string(),
                Token::BY => "BY".to_string(),
                Token::OrderDirection(res) => format!("{}", res),
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
            "*" => return Ok(Token::AttributeWildcard),
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
            "<" => return Ok(Token::Operator(Operator::Less)),
            "<=" => return Ok(Token::Operator(Operator::LessEqual)),
            ">" => return Ok(Token::Operator(Operator::Greater)),
            ">=" => return Ok(Token::Operator(Operator::GreaterEqual)),
            "LIKE" => return Ok(Token::Operator(Operator::Like)),
            "IN" => return Ok(Token::Operator(Operator::In)),
            "NOT" => return Ok(Token::Operator(Operator::NotIn)), // not can only be used before IN so it must be a NotIn operation
            "ORDER" => return Ok(Token::ORDER),
            "BY" => return Ok(Token::BY),
            "ASC" => return Ok(Token::OrderDirection(Order::Ascending)),
            "DESC" => return Ok(Token::OrderDirection(Order::Descending)),
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
            }
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
                let date_regex = Regex::new(r"^(\d?\d([-/]))?(\d?\d([-/]))?(\d{2}|\d{4})$")
                    .map_err(|x| x.to_string())?;
                let str_regex = Regex::new(r"^[\w\s]+$").map_err(|x| x.to_string())?;
                let str_list_regex =
                    Regex::new(r#"^("\w+",  ?)*("\w+")$"#).map_err(|x| x.to_string())?;
                let int_list_regex =
                    Regex::new(r#"^(\d+,  ?)*(\d+)$"#).map_err(|x| x.to_string())?;
                let float_list_regex =
                    Regex::new(r#"^(\d+.\d+,  ?)*(\d+.\d+)$"#).map_err(|x| x.to_string())?;

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
                        f64::from_str(self.identifier.as_str())
                            .map_err(|x| format!("FLOAT ERROR: {}", x.to_string()))?,
                    )));
                }

                if date_regex.is_match(self.identifier.as_str()) {
                    return Ok(Token::Value(Value::Date(Date::new(
                        self.identifier,
                        DateSource::User,
                    )?)));
                }

                // if the identifier is a string then it is an attribute string, otherwise it is a value string
                if str_regex.is_match(self.identifier.as_str()) {
                    return Ok(Token::Attribute(self.identifier)); // remove the quotes from the string
                }

                if self.content.is_some() && self.identifier.len() == 0 {
                    let cont = self.clone().content.unwrap();

                    if str_list_regex.is_match(cont.as_str()) {
                        let items: Vec<Value> = cont
                            .split(",")
                            .map(|x| {
                                let mut buf = String::new();

                                let mut add = false;

                                for char in x.chars() {
                                    if !add {
                                        if char == '"' {
                                            add = true;
                                        }
                                    } else {
                                        if char == '"' {
                                            return Value::Str(buf);
                                        } else {
                                            buf.push(char);
                                        }
                                    }
                                }

                                return Value::Str(buf);
                            })
                            .collect::<Vec<Value>>();

                        return Ok(Token::Value(Value::List(items)));
                    }

                    if int_list_regex.is_match(cont.as_str()) {
                        let parsed_items = cont.replace(" ", "");
                        let str_items = parsed_items.split(",");
                        let mut items: Vec<Value> = Vec::new();

                        for item in str_items {
                            match i64::from_str(item) {
                                Ok(res) => items.push(Value::Int(res)),
                                Err(err) => {
                                    return Err(format!(
                                        "Could not parse {} into an int. ({})",
                                        item, err
                                    ));
                                }
                            }
                        }

                        return Ok(Token::Value(Value::List(items)));
                    }

                    if float_list_regex.is_match(cont.as_str()) {
                        let parsed_items = cont.replace(" ", "");
                        let str_items = parsed_items.split(",");
                        let mut items: Vec<Value> = Vec::new();

                        for item in str_items {
                            match f64::from_str(item) {
                                Ok(res) => items.push(Value::Float(res)),
                                Err(err) => {
                                    return Err(format!(
                                        "Could not parse {} into an int. ({})",
                                        item, err
                                    ));
                                }
                            }
                        }

                        return Ok(Token::Value(Value::List(items)));
                    }

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
    let split_on = HashMap::from([('(', ')'), ('"', '"'), ('[', ']')]);
    let mut end_on: char = '.';

    let mut split = false;

    let mut rt = RawToken::new();

    for i in s.chars() {
        if split == false {
            if split_on.keys().collect::<Vec<&char>>().contains(&&i) {
                end_on = *split_on.get(&i).unwrap(); // letter must be a key to enter this condition so unwrapping here should be safe

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
    let split_on = HashMap::from([('(', ')'), ('"', '"'), ('[', ']')]);

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
        } else if group == false && (split_on.keys().collect::<Vec<&char>>().contains(&&letter)) {
            group = true;
            end_on = *split_on
                .get(&letter)
                .ok_or("You should not see this error".to_string())?;

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
