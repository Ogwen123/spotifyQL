use regex::Regex;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use std::mem::discriminant;
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
                Operator::NotIn => "NotIn"
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

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Value::Int(res) => res.to_string(),
            Value::Float(res) => res.to_string(),
            Value::Bool(res) => res.to_string(),
            Value::Str(res) => res.to_string(),
            Value::List(res) => res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
        })
    }
}

impl Value {
    pub fn compare(&self, value: Value, operator: Operator) -> Result<bool, String> {
        match operator {
            Operator::Equals => self.equals(value),
            Operator::NotEquals => Ok(!self.equals(value)?),
            Operator::Like => self.like(value),
            Operator::In => self.in_list(value),
            Operator::Less => self.less_than(value),
            Operator::LessEqual => self.less_than_or_equal(value),
            Operator::Greater => self.greater_than(value),
            Operator::GreaterEqual => self.greater_than_or_equal(value),
            Operator::NotIn => Ok(!self.in_list(value)?)
        }
    }

    // TODO: actually write comparison code
    fn equals(&self, value: Value) -> Result<bool, String> {
        if self == &value {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn like(&self, value: Value) -> Result<bool, String> {
        if let Value::Str(first) = self && let Value::Str(second) = &value {
            if first.to_lowercase().contains(second) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err("You can only use LIKE operator on strings".to_string())
        }
    }
    
    fn inner_in_list(list: Vec<Value>, val: Value) -> Result<bool, String> {
        if list.len() == 0 {
            return Ok(false)
        }
        if discriminant(&list[0]) == discriminant(&val) {
            if list.contains(&val) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err("Mismatched types in 'IN' condition!".to_string())
        }
    }
    
    fn in_list(&self, value: Value) -> Result<bool, String> {
        if let Value::List(res) = self {
            Self::inner_in_list(res.clone(), value)
        } else if let Value::List(res) = value {
            Self::inner_in_list(res, self.clone())
        } else {
            return Err("".to_string())
        }
    }

    fn extract_numerics(&self) -> Result<f64, ()> {
        match self {
            Value::Int(res) => Ok(*res as f64),
            Value::Float(res) => Ok(*res),
            _ => {
                Err(())
            }
        }
    }

    fn less_than(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self.extract_numerics().map_err(|_| "Left and right hand sides of < operation must be numeric.".to_string())?;
        let rhs: f64 = value.extract_numerics().map_err(|_| "Left and right hand sides of < operation must be numeric.".to_string())?;

        if lhs < rhs {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn less_than_or_equal(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self.extract_numerics().map_err(|_| "Left and right hand sides of <= operation must be numeric.".to_string())?;
        let rhs: f64 = value.extract_numerics().map_err(|_| "Left and right hand sides of <= operation must be numeric.".to_string())?;

        if lhs < rhs || self.equals(value)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn greater_than(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self.extract_numerics().map_err(|_| "Left and right hand sides of > operation must be numeric.".to_string())?;
        let rhs: f64 = value.extract_numerics().map_err(|_| "Left and right hand sides of > operation must be numeric.".to_string())?;

        if lhs > rhs {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn greater_than_or_equal(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self.extract_numerics().map_err(|_| "Left and right hand sides of >= operation must be numeric.".to_string())?;
        let rhs: f64 = value.extract_numerics().map_err(|_| "Left and right hand sides of >= operation must be numeric.".to_string())?;

        if lhs > rhs || self.equals(value)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    SELECT,
    AttributeWildcard,
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
                Token::AttributeWildcard => "AllAttributes".to_string(),
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
                let str_regex = Regex::new(r"^[\w\s]+$").map_err(|x| x.to_string())?;
                let str_list_regex = Regex::new(r#"^("[\w]+", [ ]?)*("[\w]+")$"#).map_err(|x| x.to_string())?;
                let int_list_regex = Regex::new(r#"^([\d]+, [ ]?)*([\d]+)$"#).map_err(|x| x.to_string())?;
                let float_list_regex = Regex::new(r#"^([\d]+.[\d]+, [ ]?)*([\d]+.[\d]+)$"#).map_err(|x| x.to_string())?;

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

                // if the identifier is a string then it is an attribute string, otherwise it is a value string
                if str_regex.is_match(self.identifier.as_str()) {
                    return Ok(Token::Attribute(self.identifier)); // remove the quotes from the string
                }

                if self.content.is_some() && self.identifier.len() == 0 {
                    let cont = self.clone().content.unwrap();

                    if str_list_regex.is_match(cont.as_str()) {
                        let items: Vec<Value> = cont.split(",").map(|x| {
                            let mut buf = String::new();

                            let mut add = false;

                            for char in x.chars() {
                                if !add {
                                    if char == '"' {
                                        add = true;
                                    }
                                } else {
                                    if char == '"' {
                                        return Value::Str(buf)
                                    } else {
                                        buf.push(char);
                                    }
                                }
                            }

                            return Value::Str(buf)
                        }).collect::<Vec<Value>>();

                        return Ok(Token::Value(Value::List(items)))
                    }

                    if int_list_regex.is_match(cont.as_str()) {
                        let parsed_items = cont.replace(" ", "");
                        let str_items = parsed_items.split(",");
                        let mut items: Vec<Value> = Vec::new();

                        for item in str_items {
                            match i64::from_str(item) {
                                Ok(res) => {
                                    items.push(Value::Int(res))
                                },
                                Err(err) => {
                                    return Err(format!("Could not parse {} into an int. ({})", item, err))
                                }
                            }
                        }

                        return Ok(Token::Value(Value::List(items)))
                    }

                    if float_list_regex.is_match(cont.as_str()) {
                        let parsed_items = cont.replace(" ", "");
                        let str_items = parsed_items.split(",");
                        let mut items: Vec<Value> = Vec::new();

                        for item in str_items {
                            match f64::from_str(item) {
                                Ok(res) => {
                                    items.push(Value::Float(res))
                                },
                                Err(err) => {
                                    return Err(format!("Could not parse {} into an int. ({})", item, err))
                                }
                            }
                        }

                        return Ok(Token::Value(Value::List(items)))
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
            end_on = *split_on.get(&letter).ok_or("You should not see this error".to_string())?;

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
