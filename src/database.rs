use sqlx::{Pool, Sqlite};
use sqlx::Executor;

pub async fn db_pool() -> Pool<Sqlite>{
    let opt = sqlx::sqlite::SqliteConnectOptions::new().filename("data/animals.db").create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(opt).await.unwrap();

    pool.execute("
      CREATE TABLE if not exists animals (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        image TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    ").await.unwrap();

    pool
}