pub mod DataDisplay {
    use std::collections::HashMap;
    use crate::query::data::KeyAccess;
    use crate::query::statements::{Aggregation, AggregationResult};

    pub fn table<T>(data: Vec<T>, attributes: Vec<String>) -> Result<(), String> where T: KeyAccess {
        let mut head_buffer: Vec<String> = Vec::new();
        let mut body_buffer: Vec<Vec<String>> = Vec::new();
        let mut max_cols: Vec<usize> = Vec::new();

        for (index, i) in attributes.iter().enumerate() {
            max_cols.push(i.len());
            head_buffer.push(i.clone());
        }

        for row in data {
            let mut buf: Vec<String> = Vec::new();
            for (colindex, col) in attributes.iter().enumerate() {
                let data = row.access(col)?.to_string();
                let length = data.len();
                buf.push(data);

                if max_cols[colindex] < length {
                    max_cols[colindex] = length
                }
            }

            body_buffer.push(buf)
        }

        // add padding

        head_buffer = head_buffer.iter().enumerate().map(|(index, col)| {
            return format!("{:^width$}", col, width = max_cols[index]);

        }).collect::<Vec<String>>();

        body_buffer = body_buffer.iter().enumerate().map(|(rowindex, row)| {
            let padded = row.iter().enumerate().map(|(colindex, col)| {
                return format!("{:^width$}", col, width = max_cols[colindex]);
            }).collect::<Vec<String>>();

            return padded
        }).collect::<Vec<Vec<String>>>();

        let sep_line = max_cols.iter().map(|len| {
            return format!("{:-<w$}", "", w = len)
        }).collect::<Vec<String>>().join("|");

        println!("|{}|\n|{}|\n{}", head_buffer.join("|"), sep_line, body_buffer.iter().map(|x| format!("|{}|", x.join("|"))).collect::<Vec<String>>().join("\n"));

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