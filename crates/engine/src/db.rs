use core::time;
use jiff::Timestamp;
use parser::keywords::FILE;
use std::collections::HashMap;
use std::io::Result;

use crate::constants::FILE_EXT;
pub type Row = Vec<String>;

/// A ShrewDB table.
/// A table is a collection of rows, where each row is a collection of columns.
pub struct Table {
    rows: Vec<Row>,
}

/// A ShrewDB database.
pub struct Database {
    pub name: String,
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(name: Option<String>) -> Database {
        Database {
            name: resolve_name(name),
            tables: HashMap::new(),
        }
    }
}

/// Load a database from a file.
/// The expected file format is a binary .shrewdb database file in binary format.
pub fn load(file_path: &str) -> Result<Database> {
    if (file_path.ends_with(FILE_EXT)) {
        let file = std::fs::File::open(file_path)?;
    }
    Ok(Database::new(None))
}

pub fn save() {
    todo!()
}

/// Resolve the name of the database.
fn resolve_name(name: Option<String>) -> String {
    let timestamp = Timestamp::now().to_string();
    let name = name.unwrap_or(timestamp);
    return name;
}
