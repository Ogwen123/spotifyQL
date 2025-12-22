use crate::query::tokenise::{Attribute, DataSource, Operator, Value};

#[derive(Debug)]
pub enum Aggregation {
    Count,
    Average,
    None,
}

pub type Condition = (Attribute, Operator, Value);

#[derive(Debug)]
pub struct SelectStatement {
    pub aggregation: Aggregation,
    pub targets: Vec<Attribute>,
    pub source: DataSource,
    pub conditions: Vec<Condition>,
}

impl SelectStatement {
    pub fn run() -> Result<(), String> {
        
        // check if we need to fetch the data source
        
        Ok(())
    }
}