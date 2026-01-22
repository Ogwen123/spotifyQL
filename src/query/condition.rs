use crate::query::data::KeyAccess;
use crate::query::tokenise::{Logical, Operator, Value};
use std::fmt::Debug;

pub fn compute_conditions<T: KeyAccess + Debug>(
    data: &T,
    conditions: Condition,
) -> Result<bool, String> {
    let is_valid;

    let mut current_condition = conditions.clone();
    let mut current_op: Logical;

    // do the first condition outside loop to set up the tree
    let res = data
        .access(current_condition.attribute)?
        .compare(current_condition.value, current_condition.operation)?;

    let mut result_tree = ConditionResult {
        val: res,
        next: None,
    };

    loop {
        if let Some(cc) = current_condition.next {
            current_condition = *(cc.1);
            current_op = cc.0;
        } else {
            break;
        }

        let res = data
            .access(current_condition.attribute)?
            .compare(current_condition.value, current_condition.operation)?;

        result_tree.add_next_condition(current_op, res)
    }

    // make sure the result tree actually needs collapsing
    if result_tree.next.is_none() {
        return Ok(result_tree.val);
    }

    let operators = vec![Logical::Or, Logical::And]; // order of precedence, all Or operations are collapsed, then all And operations, etc

    for op in operators {
        let mut prev_op = Logical::Or;
        let mut current_res = Box::new(result_tree.clone());

        let mut next: NextConditionResult;

        let mut new_tree: Option<ConditionResult> = None;

        loop {
            next = match current_res.next.clone() {
                Some(res) => res,
                None => {
                    if new_tree.is_none() {
                        new_tree = Some(ConditionResult {
                            val: current_res.val,
                            next: None,
                        })
                    } else {
                        new_tree
                            .as_mut()
                            .unwrap()
                            .add_next_condition(prev_op.clone(), current_res.val);
                    }

                    break;
                }
            };
            // do first condition outside the loop to set up the tree
            if next.0 == op {
                let eval = op.eval(current_res.val, next.1.val);

                if new_tree.is_none() {
                    new_tree = Some(ConditionResult {
                        val: eval,
                        next: None,
                    })
                } else {
                    new_tree
                        .as_mut()
                        .unwrap()
                        .add_next_condition(prev_op.clone(), eval);
                }

                let nn = next.1.next;
                if nn.is_some() {
                    let nnu = nn.unwrap();
                    prev_op = nnu.0.clone();
                    current_res = nnu.1;
                } else {
                    break;
                }
            } else {
                if new_tree.is_none() {
                    new_tree = Some(ConditionResult {
                        val: current_res.val,
                        next: None,
                    })
                } else {
                    new_tree
                        .as_mut()
                        .unwrap()
                        .add_next_condition(prev_op.clone(), current_res.val);
                }
                prev_op = next.0;
                current_res = next.1;
            }
        }
        result_tree = match new_tree {
            Some(res) => res,
            None => {
                println!("i don't think you should be able to see this");
                break;
            }
        }
    }

    // after the above code completes there could be two conditions left joined by the last operator in the list

    if result_tree.next.is_some() {
        let next = result_tree.next.unwrap();
        is_valid = next.0.eval(result_tree.val, next.1.val);
    } else {
        is_valid = result_tree.val;
    }
    println!("{}", is_valid);
    Ok(is_valid)
}

pub type NextCondition = (Logical, Box<Condition>);

#[derive(Debug, Clone)]
pub struct Condition {
    pub attribute: String,
    pub operation: Operator,
    pub value: Value,
    pub next: Option<NextCondition>,
}

impl Condition {
    pub fn add_next_condition(&mut self, logical: Logical, condition: Condition) {
        let mut current = self;

        while let Some((_, ref mut next)) = current.next {
            current = next;
        }

        current.next = Some((logical, Box::new(condition)))
    }
}

pub type NextConditionResult = (Logical, Box<ConditionResult>);

#[derive(Clone, Debug)]
pub struct ConditionResult {
    pub val: bool,
    pub next: Option<NextConditionResult>,
}

impl ConditionResult {
    pub fn add_next_condition(&mut self, logical: Logical, val: bool) {
        let res = Box::new(ConditionResult { val, next: None });

        let mut current = self;

        while let Some((_, ref mut next)) = current.next {
            current = next;
        }

        current.next = Some((logical, res));
    }
}
