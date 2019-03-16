use super::author::*;
use crate::db::*;

#[derive(Default)]
pub struct Like {}

graphql_object!(Like: Context |&self| {
    field author(&executor) -> Option<Author> {
        let database = executor.context();
        let author = Author::default();
        Some(author)
    }
});
