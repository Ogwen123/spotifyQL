use crate::query::statements::{Aggregation, Condition, SelectStatement};
use crate::query::tokenise::{Attribute, DataSource, Operator, Token};

fn safe_next(iter: &mut dyn Iterator<Item = Token>) -> Result<Token, String> {
    match iter.next() {
        Some(res) => Ok(res),
        None => Err("SYNTAX ERROR: Incomplete statement".to_string()),
    }
}

pub fn parse(_tokens: Vec<Token>) -> Result<SelectStatement, String> {
    // if the tokens contain a COUNT then it's a SelectCount, otherwise it's a Select
    if _tokens.len() < 4 {
        return Err(
            "Must have a minimum of 4 tokens. e.g. 'SELECT name FROM Playlist(\"pl1\")".to_string(),
        );
    }
    let mut tokens = _tokens.into_iter();

    // the first 4 tokens can be unwrapped safely because of the above check
    let statement_type = tokens.next().unwrap();
    if statement_type == Token::SELECT {
        let mut aggregation = Aggregation::None;
        let mut targets: Vec<Attribute> = Vec::new();

        loop {
            // collect attributes
            let attr = match tokens.next() {
                Some(res) => res,
                None => break,
            };

            match attr.clone() {
                Token::COUNT(res) => {
                    if targets.len() != 0 {
                        return Err(format!(
                            "SYNTAX ERROR: Cannot mix aggregated attributes and non-aggregated attributes at {}",
                            attr
                        ));
                    }

                    targets.push(res);
                    aggregation = Aggregation::Count;

                    // cannot mix aggregation and normal attributes to this has to be the end of the attribute section
                    break;
                }
                Token::AVERAGE(res) => {
                    if targets.len() != 0 {
                        return Err(format!(
                            "SYNTAX ERROR: Cannot mix aggregated attributes and non-aggregated attributes at {}",
                            attr
                        ));
                    }

                    targets.push(res);
                    aggregation = Aggregation::Average;

                    // cannot mix aggregation and normal attributes to this has to be the end of the attribute section
                    break;
                }
                Token::Attribute(res) => {
                    targets.push(res);
                }
                _ => return Err(format!("SYNTAX ERROR: Invalid token at {}", attr)),
            }
        }

        if targets.len() == 0 {
            return Err("SYNTAX ERROR: No attributes defined after SELECT".to_string());
        }

        let fr = safe_next(&mut tokens)?;

        if fr != Token::FROM {
            return Err(format!("SYNTAX ERROR: Token at {} should be FROM.", fr));
        }

        let source: DataSource;

        let st = safe_next(&mut tokens)?;

        match st {
            Token::Source(res) => source = res,
            _ => {
                return Err(format!(
                    "SYNTAX ERROR: Token {} should be a data source. e.g. Playlist(\"name\")",
                    st
                ));
            }
        }

        match tokens.next() {
            Some(w) => match w {
                Token::WHERE => {}
                _ => {
                    return Ok(SelectStatement {
                        aggregation,
                        targets,
                        source,
                        conditions: Vec::new(),
                    });
                }
            },
            None => {
                return Ok(SelectStatement {
                    aggregation,
                    targets,
                    source,
                    conditions: Vec::new(),
                });
            }
        }

        // get conditions
        let mut conditions: Vec<Condition> = Vec::new();

        loop {
            // every condition should be made up of 3 tokens
            let attr = match tokens.next() {
                Some(res) => match res {
                    Token::Attribute(res) => res,
                    _ => {
                        return Err(format!("SYNTAX ERROR: Condition is missing attribute at {}", res))
                    }
                },
                None => {
                    return Err("SYNTAX ERROR: Conditions should consist of an attribute, an operator and a value".to_string())
                }
            };

            let op = match tokens.next() {
                Some(res) => match res {
                    Token::Operator(res) => res,
                    _ => {
                        return Err(format!("SYNTAX ERROR: Condition is missing operator at {}", res))
                    }
                },
                None => {
                    return Err("SYNTAX ERROR: Conditions should consist of an attribute, an operator and a value".to_string())
                }
            };

            let val = match tokens.next() {
                Some(res) => match res {
                    Token::Value(res) => res,
                    _ => {
                        return Err(format!("SYNTAX ERROR: Condition is missing value at {}", res))
                    }
                },
                None => {
                    return Err("SYNTAX ERROR: Conditions should consist of an attribute, an operator and a value".to_string())
                }
            };

            conditions.push((attr, op, val));
            
            match tokens.next() {
                Some(res) => {
                    match res {
                        Token::Bitwise(_) => {},
                        _ => {
                            return Err(format!("SYNTAX ERROR: Only a bitwise operator (AND, OR) can come after a condition, error at {}", res))
                        }
                    }
                },
                None => {
                    break
                }
            }
        }

        Ok(SelectStatement {
            aggregation,
            targets,
            source,
            conditions,
        })
    } else {
        Err(format!("SYNTAX ERROR: Invalid token at {}", statement_type))
    }
}
