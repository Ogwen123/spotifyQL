use std::collections::HashMap;
use crate::query::data::KeyAccess;
use crate::query::statements::{Aggregation, AggregationResult};

pub struct DataDisplay;

impl DataDisplay {
    pub fn table<T>(data: Vec<T>, attributes: Vec<String>) -> Result<(), String> where T: KeyAccess {
        for i in data {
            for attr in &attributes {
                print!("{:?} ", i.access(attr)?)
            }
            println!()
        }

        Ok(())
    }

    pub fn aggregation_table(aggregation: Aggregation, data: HashMap<String, AggregationResult>) {
        let mut head_buffer: Vec<String> = Vec::new();
        let mut body_buffer: Vec<String> = Vec::new();
        let mut max_cols: Vec<usize> = Vec::new();

        data.iter().for_each(|(k, v)| {
            let mut str = aggregation.format(k);

            let mut info_line = match v {
                AggregationResult::Int(res) => {
                    format!("{}", res)
                },
                AggregationResult::Float(res) => {
                    format!("{:.2}", res)
                }
            };

            if info_line.len() > str.len() {
                max_cols.push(info_line.len());
                str = format!("{:<width$}", str, width = info_line.len());
            } else {
                max_cols.push(str.len());
                info_line = format!("{:<width$}", info_line, width = str.len());
            }

            head_buffer.push(str);
            body_buffer.push(info_line)
        });

        let sep_line = max_cols.iter().map(|len| {
            return format!("{:-<w$}", "", w = len)
        }).collect::<Vec<String>>().join("|");

        println!("|{}|\n|{}|\n|{}|", head_buffer.join("|"), sep_line, body_buffer.join("|"))
    }
}