use std::any::{Any, TypeId};
use crate::query::data::KeyAccess;
use crate::query::statements::{Aggregation, AggregationResult};

pub struct DataDisplay;

impl DataDisplay {
    pub fn table<T>(data: Vec<T>, attributes: Vec<String>) where T: KeyAccess {
        for i in data {
            for attr in &attributes {
                print!("{:?} ", i.access(attr))
            }
            println!()
        }
    }

    pub fn aggregation_table(aggregation: Aggregation, attributes: Vec<String>, value: AggregationResult) {
        let mut display_buffer: Vec<String> = Vec::new();
        let mut max_col: usize;

        display_buffer.push(format!("{}", aggregation.format(attributes)));
        max_col = display_buffer[0].len();
        let info_line = match value {
            AggregationResult::Int(res) => {
                format!("{}", res)
            },
            AggregationResult::Float(res) => {
                format!("{:2}", res)
            }
        };

        if info_line.len() > max_col {
            max_col = info_line.len()
        }
        display_buffer.push(format!("{:-<w$}", "", w = max_col));
        display_buffer.push(info_line);
        
        println!("{}", display_buffer.join("\n"))
    }
}