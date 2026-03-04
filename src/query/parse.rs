use crate::query::condition::Condition;
use crate::query::data::{AlbumData, KeyAccess, PlaylistData, TrackData};
use crate::query::statements::{Aggregation, OrderBy, SelectStatement};
use crate::query::tokenise::{DataSource, Logical, Operator, Order, Token};
use crate::query::value::Value;

fn safe_next(iter: &mut dyn Iterator<Item = Token>) -> Result<Token, String> {
    match iter.next() {
        Some(res) => Ok(res),
        None => Err("SYNTAX ERROR: Incomplete statement".to_string()),
    }
}

fn split_aggregated_attributes(attributes: String) -> Vec<String> {
    attributes
        .replace(" ", "")
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn parse_where(tokens: &mut dyn Iterator<Item = Token>) -> Result<Option<Condition>, String> {
    // get conditions
    let mut tl_condition: Option<Condition> = None;
    let mut next_logical_op: Logical = Logical::Or; // should never get used, just to avoid uninitialised error below

    /*
    order of operation should go OR -> AND and is read left to right using associative law
    so true && false || false || true && false
    is evaluated as true && (false && (false || (false || true)))
    */

    loop {
        let attr_t = match tokens.next() {
            Some(res) => res,
            None => return Err(
                "SYNTAX ERROR: Conditions should consist of an attribute, an operator and a value"
                    .to_string(),
            ),
        };
        let attr: String;
        let op: Operator;
        let val: Value;

        if let Token::Attribute(res) = attr_t {
            // every condition should be made up of 3 tokens
            attr = res;

            op = match tokens.next() {
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

            if op == Operator::NotIn {
                // ignore the IN token that should be after a NOT token
                if let Some(Token::Operator(res)) = tokens.next() {
                    if res != Operator::In {
                        return Err(
                            "SYNTAX ERROR: NOT can only be used to negate an IN operation."
                                .to_string(),
                        );
                    }
                }
            }

            val = match tokens.next() {
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
        } else if let Token::Value(res) = attr_t {
            // this branch only happens when checking if a value is in a list e.g. WHERE "Arctic Monkeys" IN artists
            val = res;

            op = match tokens.next() {
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

            if op == Operator::NotIn {
                // ignore the IN token that should be after a NOT token
                if let Some(Token::Operator(res)) = tokens.next() {
                    if res != Operator::In {
                        return Err(
                            "SYNTAX ERROR: NOT can only be used to negate an IN operation."
                                .to_string(),
                        );
                    }
                }
            }

            attr = match tokens.next() {
                Some(res) => match res {
                    Token::Attribute(res) => res,
                    _ => {
                        return Err(format!("SYNTAX ERROR: Condition is missing value at {}", res))
                    }
                },
                None => {
                    return Err("SYNTAX ERROR: Conditions should consist of an attribute, an operator and a value".to_string())
                }
            };
        } else {
            return Err(format!(
                "SYNTAX ERROR: Condition is missing attribute at {}",
                attr_t
            ));
        }

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

    Ok(tl_condition)
}

fn parse_order(tokens: &mut dyn Iterator<Item = Token>) -> Result<Option<OrderBy>, String> {
    let mut attributes: Vec<String> = Vec::new();
    let mut order: Order = Order::Ascending;

    match tokens.next() {
        Some(res) => {
            match res {
                Token::BY => {},
                _ => return Err("SYNTAX ERROR: Incomplete compound keyword, ORDER must be followed with BY. (ORDER BY ... ASC)".to_string())
            }
        },
        None => {
            return Err("SYNTAX ERROR: Incorrect syntax at ORDER".to_string())
        }
    };

    loop {
        match tokens.next() {
            Some(res) => {
                match res {
                    Token::OrderDirection(ord) => {
                        order = ord;
                        break;
                    },
                    Token::Attribute(attr) => {
                        attributes.push(attr)
                    },
                    _ => return Err("SYNTAX ERROR: Invalid tokens in order statement, must be in the form 'ORDER BY a, b, c ASC'".to_string())

                }
            },
            None => {
                return Err("SYNTAX ERROR: Order statement not completed, must be in the form 'ORDER BY a, b, c ASC'".to_string())
            }
        }
    }

    Ok(Some((attributes, order)))
}

pub fn parse(_tokens: Vec<Token>) -> Result<SelectStatement, String> {
    // if the tokens contain a COUNT then it's a SelectCount, otherwise it's a Select
    if _tokens.len() < 4 {
        return Err(
            "Must have a minimum of 4 tokens. e.g. 'SELECT name FROM Playlist(\"pl1\")".to_string(),
        );
    }
    let mut tokens = _tokens.into_iter().peekable();

    // the first 4 tokens can be unwrapped safely because of the above check
    let statement_type = tokens.next().unwrap();
    if statement_type == Token::SELECT {
        let mut aggregation = Aggregation::None;
        let mut targets: Vec<String> = Vec::new();

        let mut attribute_wild_card = false;

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

                    targets = split_aggregated_attributes(res);
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

                    targets = split_aggregated_attributes(res);
                    aggregation = Aggregation::Average;

                    // cannot mix aggregation and normal attributes to this has to be the end of the attribute section
                    break;
                }
                Token::Attribute(res) => {
                    targets.push(res);
                }
                Token::AttributeWildcard => {
                    if targets.len() != 0 {
                        return Err(format!(
                            "SYNTAX ERROR: Cannot mix wildcard with specific attributes at {}",
                            attr
                        ));
                    }

                    attribute_wild_card = true; // need to wait to find the datasource token to get the attributes list
                    break;
                }
                Token::FROM => {
                    reached_from = true;
                    break;
                }
                _ => return Err(format!("SYNTAX ERROR: Invalid token at {}", attr)),
            }
        }

        if targets.len() == 0 && attribute_wild_card == false {
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

        if attribute_wild_card {
            targets = match source {
                DataSource::Playlist(_) | DataSource::SavedAlbum(_) => TrackData::attributes(),
                DataSource::Playlists => PlaylistData::attributes(),
                DataSource::SavedAlbums => AlbumData::attributes(),
            }
        }

        let mut order: Option<OrderBy> = None;
        let mut conditions: Option<Condition> = None;

        loop {
            match tokens.next() {
                Some(w) => match w {
                    Token::WHERE => conditions = parse_where(&mut tokens)?,
                    Token::ORDER => order = parse_order(&mut tokens)?,
                    _ => {
                        return Ok(SelectStatement {
                            aggregation,
                            targets,
                            source,
                            order,
                            conditions,
                        });
                    }
                },
                None => {
                    return Ok(SelectStatement {
                        aggregation,
                        targets,
                        source,
                        order,
                        conditions,
                    });
                }
            }
        }
    } else {
        Err(format!("SYNTAX ERROR: Invalid token at {}", statement_type))
    }
}
