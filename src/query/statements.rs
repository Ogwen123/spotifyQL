use crate::query::tokenise::{Attribute, DataSource, Operator};

pub enum Aggregation {
    Count,
    Average,
    None
}

pub struct SelectStatement {
    pub aggregation: Aggregation,
    pub targets: Vec<Attribute>,
    pub source: DataSource,
    pub condition: Option<(Attribute, Operator, String)>
}