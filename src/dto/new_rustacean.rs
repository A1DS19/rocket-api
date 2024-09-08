use diesel::prelude::Insertable;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::rustacean)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
