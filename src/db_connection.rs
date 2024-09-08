use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};

#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);
