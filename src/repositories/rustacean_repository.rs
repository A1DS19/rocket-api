use crate::{dto::new_rustacean::NewRustacean, models::rustacean::Rustacean, schema::rustacean};
use diesel::{
    query_dsl::methods::{LimitDsl, OrderDsl},
    ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection,
};

use super::repository::Repository;

pub struct RustaceanRepository {}

impl Repository for RustaceanRepository {
    type Model = Rustacean;
    type NewModel = NewRustacean;

    fn find_many(c: &mut SqliteConnection) -> QueryResult<Vec<Rustacean>> {
        rustacean::table
            .order(rustacean::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
    }

    fn create(c: &mut SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<usize> {
        diesel::insert_into(rustacean::table)
            .values(new_rustacean)
            .execute(c)
    }
}
