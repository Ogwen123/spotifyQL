use crate::query::tokenise::Operator;
use crate::utils::date::Date;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::mem::discriminant;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Date(Date),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Int(res) => res.to_string(),
                Value::Float(res) => res.to_string(),
                Value::Bool(res) => res.to_string(),
                Value::Str(res) => res.to_string(),
                Value::Date(res) => res.format(),
                Value::List(res) => res
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            }
        )
    }
}

impl Value {
    pub fn compare(&self, value: Value, operator: Operator) -> Result<bool, String> {
        if let Value::Date(attr) = self {
            // standalone years get tokenised as ints so need toc convert to date if the attr is a date
            let target: &Date = match &value {
                Value::Int(res) => &Date::year(res.clone() as u32)?,
                Value::Date(res) => res,
                _ => return Err("Can only compare a Date to another Date".to_string()),
            };

            return match operator {
                Operator::Equals => self.equals(value),
                Operator::NotEquals => Ok(!self.equals(value)?),
                Operator::Less => Ok(attr < target),
                Operator::LessEqual => Ok(attr <= target),
                Operator::Greater => Ok(attr > target),
                Operator::GreaterEqual => Ok(attr >= target),
                _ => Err("Invalid operation on Date type.".to_string()),
            };
        }

        match operator {
            Operator::Equals => self.equals(value),
            Operator::NotEquals => Ok(!self.equals(value)?),
            Operator::Like => self.like(value),
            Operator::In => self.in_list(value),
            Operator::Less => self.less_than(value),
            Operator::LessEqual => self.less_than_or_equal(value),
            Operator::Greater => self.greater_than(value),
            Operator::GreaterEqual => self.greater_than_or_equal(value),
            Operator::NotIn => Ok(!self.in_list(value)?),
        }
    }

    fn equals(&self, value: Value) -> Result<bool, String> {
        if self == &value { Ok(true) } else { Ok(false) }
    }

    fn like(&self, value: Value) -> Result<bool, String> {
        if let Value::Str(first) = self
            && let Value::Str(second) = &value
        {
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
            return Ok(false);
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
            return Err(
                "SYNTAX ERROR: IN operator only valued between a list and a value.".to_string(),
            );
        }
    }

    fn extract_numerics(&self) -> Result<f64, ()> {
        match self {
            Value::Int(res) => Ok(*res as f64),
            Value::Float(res) => Ok(*res),
            _ => Err(()),
        }
    }

    fn less_than(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self
            .extract_numerics()
            .map_err(|_| "Left and right hand sides of < operation must be numeric.".to_string())?;
        let rhs: f64 = value
            .extract_numerics()
            .map_err(|_| "Left and right hand sides of < operation must be numeric.".to_string())?;

        if lhs < rhs { Ok(true) } else { Ok(false) }
    }

    fn less_than_or_equal(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self.extract_numerics().map_err(|_| {
            "Left and right hand sides of <= operation must be numeric.".to_string()
        })?;
        let rhs: f64 = value.extract_numerics().map_err(|_| {
            "Left and right hand sides of <= operation must be numeric.".to_string()
        })?;

        if lhs < rhs || self.equals(value)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn greater_than(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self
            .extract_numerics()
            .map_err(|_| "Left and right hand sides of > operation must be numeric.".to_string())?;
        let rhs: f64 = value
            .extract_numerics()
            .map_err(|_| "Left and right hand sides of > operation must be numeric.".to_string())?;

        if lhs > rhs { Ok(true) } else { Ok(false) }
    }

    fn greater_than_or_equal(&self, value: Value) -> Result<bool, String> {
        let lhs: f64 = self.extract_numerics().map_err(|_| {
            "Left and right hand sides of >= operation must be numeric.".to_string()
        })?;
        let rhs: f64 = value.extract_numerics().map_err(|_| {
            "Left and right hand sides of >= operation must be numeric.".to_string()
        })?;

        if lhs > rhs || self.equals(value)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn perform_comparison<T>(res1: T, res2: T) -> Option<Ordering>
where
    T: PartialOrd,
{
    Some(if res1 < res2 {
        Ordering::Less
    } else if res1 > res2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    })
}

impl PartialOrd for Value {
    /// If None is returned it is because the provided attribute is not orderable, e.g. list or bool
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if discriminant(self) != discriminant(other) {
            return None;
        }

        match self {
            Value::Int(res1) => {
                if let Value::Int(res2) = other {
                    return perform_comparison(res1, res2);
                }
                None
            }
            Value::Float(res1) => {
                if let Value::Float(res2) = other {
                    return perform_comparison(res1, res2);
                }
                None
            }
            Value::Str(res1) => {
                if let Value::Str(res2) = other {
                    return perform_comparison(res1, res2);
                }
                None
            }
            Value::Date(res1) => {
                if let Value::Date(res2) = other {
                    return perform_comparison(res1, res2);
                }
                None
            }
            _ => None,
        }
    }
}
