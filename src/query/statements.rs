use crate::query::tokenise::{DataSource, Logical, Operator, Value};

struct GeneralData {}

#[derive(Debug)]
pub enum Aggregation {
    Count,
    Average,
    None,
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

#[derive(Debug)]
pub struct SelectStatement {
    pub aggregation: Aggregation,
    pub targets: Vec<String>,
    pub source: DataSource,
    pub conditions: Option<Condition>,
}

impl SelectStatement {
    pub fn run(&self) -> Result<(), String> {
        // gather targets
        //let targets: HashMap<String, >

        // apply conditions
        // apply aggregations

        Ok(())
    }
}
