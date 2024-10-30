use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

type DbPool = Pool<Postgres>;

pub async fn create_pool(db_url: &str) -> Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    Ok(pool)
}

pub mod queries {
    use sqlx::Pool;
    use crate::db::DbPool;
    use crate::models::Post;
}
