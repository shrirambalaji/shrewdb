use core::{error, panic};

use crate::db::Database;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("something went wrong")]
    Unknown,
}

pub type ExecutionResult = Result<Vec<Vec<String>>, ExecutionError>;

pub fn execute(db: &mut Database, sql: &str) -> ExecutionResult {
    let statement = parser::parse_sql(sql).unwrap();
    let results = Vec::new();

    match &statement[0] {
        parser::ast::Statement::Query(query) => {
            println!("query: {:?}", query);
            let table = db.tables.get(&query.body.to_string()).unwrap();
        }
        _ => return Err(ExecutionError::Unknown),
    }
    Ok(results)
}
