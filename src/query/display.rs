use crate::query::data::KeyAccess;

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
}