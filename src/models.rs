use diesel::prelude::*;
use crate::schema::ideas;
use chrono;

#[derive(Queryable)]
pub struct Idea {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = ideas)]
pub struct NewIdea<'a> {
    pub title: &'a str,
    pub body: &'a str,
}