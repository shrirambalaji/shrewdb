use std::collections::HashMap;
use std::io::Result;

use jiff::Timestamp;
pub type Row = Vec<String>;

pub struct Table {
    rows: Vec<Row>,
}

pub struct Database {
    pub name: String,
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(name: Option<String>) -> Database {
        Database {
            name: name.unwrap_or(Timestamp::now().to_string()),
            tables: HashMap::new(),
        }
    }
}

pub fn load(file_path: &str) -> Result<Database> {
    Ok(Database::new(None))
}

pub fn save() {
    todo!()
}
