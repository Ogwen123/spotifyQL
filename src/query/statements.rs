use crate::query::tokenise::{Attribute, DataSource, Operator};

#[derive(Debug)]
pub enum Aggregation {
    Count,
    Average,
    None,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub aggregation: Aggregation,
    pub targets: Vec<Attribute>,
    pub source: DataSource,
    pub conditions: Vec<(Attribute, Operator, String)>,
}
