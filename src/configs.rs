use std::env;
use std::time::Duration;
use log::info;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn get_db_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    info!("Database connected successfully!");

    pool
}