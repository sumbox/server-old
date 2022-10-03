use std::{env};

use diesel::{
    r2d2::{self, ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn init() -> String {
    dotenv().ok();

    return String::from("");
}

pub fn get_pool() -> PostgresPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}
