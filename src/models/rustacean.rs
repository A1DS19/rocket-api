use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::rustacean)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Rustacean {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: Option<String>,
}
