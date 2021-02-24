use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn create_connection_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
