pub mod models;
pub mod schema;

use std::path::PathBuf;

use diesel::prelude::*;

pub fn establish_connection(db: PathBuf) -> SqliteConnection {
    SqliteConnection::establish(&db.to_string_lossy().into_owned())
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db.to_string_lossy().into_owned()))
}