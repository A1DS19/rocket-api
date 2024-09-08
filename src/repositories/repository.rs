use diesel::{QueryResult, SqliteConnection};

pub trait Repository {
    type Model;
    type NewModel;

    fn find_many(c: &mut SqliteConnection) -> QueryResult<Vec<Self::Model>>;
    fn create(c: &mut SqliteConnection, new_model: Self::NewModel) -> QueryResult<usize>;
}
