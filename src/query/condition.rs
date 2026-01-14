use crate::query::data::KeyAccess;
use crate::query::tokenise::{Logical, Operator, Value};
use std::fmt::Debug;

pub type NextCondition = (Logical, Box<Condition>);

pub fn compute_conditions<T: KeyAccess + Debug>(
    data: &T,
    conditions: Condition,
) -> Result<bool, String> {
    let is_valid = false;

    let mut current_condition = conditions.clone();
    let mut current_op: Logical = Logical::Or;

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
    println!("{:?}     {:?}", result_tree, data);

    // make sure the result tree actually needs collapsing
    if result_tree.next.is_none() {
        return Ok(result_tree.val);
    }

    // TODO: write code to collapse result tree and update is_valid
    let operators = vec![Logical::Or, Logical::And]; // order of precedence, all Or operations are collapsed, then all And operations, etc

    for op in operators {
        let mut prev_op = Logical::Or;
        let mut current_res = Box::new(result_tree.clone());

        let next: NextConditionResult;

        let mut new_tree: Option<ConditionResult> = None;

        loop {
            let next = current_res.next.clone().unwrap();
            // do first condition outside the loop to set up the tree
            if next.0 == op{
                let eval = op.eval(current_res.val, next.1.val);

                if new_tree.is_none() {
                    new_tree = Some(ConditionResult {
                        val: eval,
                        next: next.1.next
                    })
                } else {
                    new_tree.as_mut().unwrap().add_next_condition(prev_op.clone(), eval);
                }
            } else {
                current_res = next.1;
            }
        }

    }

    Ok(is_valid)
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub attribute: String,
    pub operation: Operator,
    pub value: Value,
    pub next: Option<NextCondition>,
}

impl Condition {
    pub fn add_next_condition(&mut self, logical: Logical, condition: Condition) {
        let mut next: Box<Condition>;

        if self.next.is_none() {
            self.next = Some((logical, Box::new(condition)));
            return;
        } else {
            next = self.next.clone().unwrap().1;
            loop {
                if next.next.is_none() {
                    next.next = Some((logical, Box::new(condition)));
                    break;
                } else {
                    next = next.next.unwrap().1;
                }
            }
        }
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
        let mut next: Box<ConditionResult>;

        let res = Box::new(ConditionResult { val, next: None });

        if self.next.is_none() {
            self.next = Some((logical, res));
            return;
        } else {
            next = self.next.clone().unwrap().1;
            loop {
                if next.next.is_none() {
                    next.next = Some((logical, res));
                    break;
                } else {
                    next = next.next.unwrap().1;
                }
            }
        }
    }
}
