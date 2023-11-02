use clap::{App, Arg, SubCommand};
use rusqlite::Connection;
use std::error::Error;

// Ensure the db module is implemented in db.rs
mod db;

// Custom error type encompassing different kinds of errors our application might encounter
#[derive(Debug)]
enum CliError {
    DatabaseError(rusqlite::Error),
    ArgumentError(String),
}

impl From<rusqlite::Error> for CliError {
    fn from(e: rusqlite::Error) -> Self {
        CliError::DatabaseError(e)
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::DatabaseError(ref e) => e.fmt(f),
            CliError::ArgumentError(ref s) => write!(f, "Argument Error: {}", s),
        }
    }
}

impl Error for CliError {}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("My CLI App")
        .version("1.0")
        .author("Michael")
        .about("Rust CLI with SQLite Integration")
        // Define the subcommands and their arguments
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new record")
                .arg(
                    Arg::with_name("CONTENT")
                        .help("The data to be inserted")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("read").about("Reads all records"))
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates an existing record")
                .arg(
                    Arg::with_name("ID")
                        .help("The ID of the record to update")
                        .required(true),
                )
                .arg(
                    Arg::with_name("CONTENT")
                        .help("The new data for the record")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a record")
                .arg(
                    Arg::with_name("ID")
                        .help("The ID of the record to delete")
                        .required(true),
                ),
        )
        .get_matches();

    // Open a connection to the SQLite database
    let conn = Connection::open("mydb.db")?;

    // Call the create_table function here to ensure the table is created when the app starts
    db::create_table(&conn)?;

    match matches.subcommand() {
        ("create", Some(sub_m)) => {
            let data = sub_m
                .value_of("CONTENT")
                .ok_or(CliError::ArgumentError("Missing data for create".into()))?;
            db::create_entry(&conn, data)?;
        }
        ("read", Some(_)) => {
            db::read_entries(&conn)?;
        }
        ("update", Some(sub_m)) => {
            let id_str = sub_m
                .value_of("ID")
                .ok_or(CliError::ArgumentError("Missing ID for update".into()))?;
            let id: i32 = id_str.parse()?;
            let data = sub_m
                .value_of("CONTENT")
                .ok_or(CliError::ArgumentError("Missing data for update".into()))?;
            db::update_entry(&conn, id, data)?;
        }
        ("delete", Some(sub_m)) => {
            let id_str = sub_m
                .value_of("ID")
                .ok_or(CliError::ArgumentError("Missing ID for delete".into()))?;
            let id: i32 = id_str.parse()?;
            db::delete_entry(&conn, id)?;
        }
        _ => return Err(Box::new(CliError::ArgumentError("Invalid command".into()))),
    }

    Ok(())
}
