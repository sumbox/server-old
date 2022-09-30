use diesel::PgConnection;
use crate::models::{Idea, NewIdea};

#[allow(dead_code)]
pub fn create_idea(conn: &mut PgConnection, title: &str, body: &str) -> Idea {
    use crate::schema::ideas;
    use diesel::RunQueryDsl;

    let new_idea = NewIdea { title, body };

    diesel::insert_into(ideas::table)
        .values(&new_idea)
        .get_result(conn)
        .expect("Error saving new Idea")
}