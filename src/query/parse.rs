use crate::query::statements::{Aggregation, Condition, SelectStatement};
use crate::query::tokenise::{DataSource, Logical, Token};

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
        let mut targets: Vec<String> = Vec::new();

        let mut reached_from = false;
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
                Token::FROM => {
                    reached_from = true;
                    break;
                }
                _ => return Err(format!("SYNTAX ERROR: Invalid token at {}", attr)),
            }
        }

        if targets.len() == 0 {
            return Err("SYNTAX ERROR: No attributes defined after SELECT".to_string());
        }

        if !reached_from {
            let fr = safe_next(&mut tokens)?;

            if fr != Token::FROM {
                return Err(format!("SYNTAX ERROR: Token at {} should be FROM.", fr));
            }
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
                        conditions: None,
                    });
                }
            },
            None => {
                return Ok(SelectStatement {
                    aggregation,
                    targets,
                    source,
                    conditions: None,
                });
            }
        }

        // get conditions
        let mut tl_condition: Option<Condition> = None;
        let mut next_logical_op: Logical = Logical::Or; // should never get used, just to avoid uninitialised error below

        /*
        order of operation should go OR -> AND and is read left to right using associative law
        so true && false || false || true && false
        is evaluated as true && (false && (false || (false || true)))
        */

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

            // validate conditions have correct types, e.g. name can not equal a number so should throw an error, types are string, int, float, bool

            // when it comes to evaluating the conditions on each row, convert each condition into a boolean value of true or false and simplify it down to a single boolean using boolean algebra rules

            let temp = Condition {
                attribute: attr,
                operation: op,
                value: val,
                next: None,
            };

            if tl_condition.is_some() {
                tl_condition
                    .as_mut()
                    .unwrap()
                    .add_next_condition(next_logical_op, temp);
            } else {
                tl_condition = Some(temp);
            }

            match tokens.next() {
                Some(res) => match res {
                    Token::Logical(res) => next_logical_op = res,
                    _ => {
                        return Err(format!(
                            "SYNTAX ERROR: Only a bitwise operator (AND, OR) can come after a condition, error at {}",
                            res
                        ));
                    }
                },
                None => break,
            }
        }

        Ok(SelectStatement {
            aggregation,
            targets,
            source,
            conditions: tl_condition,
        })
    } else {
        Err(format!("SYNTAX ERROR: Invalid token at {}", statement_type))
    }
}
