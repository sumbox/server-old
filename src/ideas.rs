// use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};

use crate::models::{Idea, NewIdea};
use crate::init::get_pool;


#[allow(dead_code)]
pub fn create_idea(title: &str, body: &str) -> Idea {
    use crate::schema::ideas;
    use diesel::RunQueryDsl;

    let new_idea = NewIdea { title, body };

    diesel::insert_into(ideas::table)
        .values(&new_idea)
        .get_result(&mut *get_pool().clone().get().expect("Failed to get connection from pool"))
        .expect("Error saving new Idea")
}