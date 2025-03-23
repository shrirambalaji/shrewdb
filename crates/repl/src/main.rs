use engine::db::{self};
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result as ReplResult};
use std::fmt;

#[derive(Debug)]
enum ShrewReplError {
    ReedlineReplError(reedline_repl_rs::Error),
    EngineError(String),
}

/// Custom error type for the REPL
/// This error type wraps the reedline_repl_rs::Error and adds a custom message
impl From<reedline_repl_rs::Error> for ShrewReplError {
    fn from(e: reedline_repl_rs::Error) -> Self {
        ShrewReplError::ReedlineReplError(e)
    }
}

impl fmt::Display for ShrewReplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShrewReplError::ReedlineReplError(e) => write!(f, "REPL Error: {}", e),
            ShrewReplError::EngineError(s) => write!(f, "Engine Error: {}", s),
        }
    }
}

impl std::error::Error for ShrewReplError {}

/// Exit the REPL
fn exit<T>(_args: ArgMatches, _context: &mut T) -> Result<Option<String>, ShrewReplError> {
    println!("Goodbye!");
    std::process::exit(0);
}

/// Open a database connection
fn open<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>, ShrewReplError> {
    let db_name = args.get_one::<String>("db").unwrap();
    println!("Opening database: {}", db_name);
    let db = db::load(db_name);
    match db {
        Ok(_db) => Ok(Some(format!("Database {} opened successfully", db_name))),
        Err(e) => Err(ShrewReplError::EngineError(e.to_string())),
    }
}

fn main() -> ReplResult<()> {
    let mut repl = Repl::new(())
        .with_name("shrewdb")
        .with_version("v0.1.0")
        .with_description("a tiny sql database")
        .with_banner(
            r"
  /$$$$$S  /$$   /$$ /$$$$$$$  /$$$$$$$$ /$$      /$$ /$$$$$$$  /$$$$$$$ 
 /$$__  $$| $$  | $$| $$__  $$| $$_____/| $$  /$ | $$| $$__  $$| $$__  $$
| $$  \__/| $$  | $$| $$  \ $$| $$      | $$ /$$$| $$| $$  \ $$| $$  \ $$
|  $$$$$$ | $$$$$$$$| $$$$$$$/| $$$$$   | $$/$$ $$ $$| $$  | $$| $$$$$$$ 
 \____  $$| $$__  $$| $$__  $$| $$__/   | $$$$_  $$$$| $$  | $$| $$__  $$
 /$$  \ $$| $$  | $$| $$  \ $$| $$      | $$$/ \  $$$| $$  | $$| $$  \ $$
|  $$$$$$/| $$  | $$| $$  | $$| $$$$$$$$| $$/   \  $$| $$$$$$$/| $$$$$$$/
 \______/ |__/  |__/|__/  |__/|________/|__/     \__/|_______/ |_______/ .       
        ",
        )
        .with_command(
            Command::new(".open")
                .about("Open a database connection")
                .arg(
                    Arg::new("db")
                        .help("The name of database; creates a new database if none exists")
                        .required(false)
                        .index(1),
                ),
            open,
        )
        .with_command(Command::new(".exit"), exit);
    repl.run()
}
