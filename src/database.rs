use std::sync::Arc;

use sqlx::{Executor, PgPool, migrate::MigrateDatabase};

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: Arc<sqlx::PgPool>,
}

impl Database {
    pub async fn connect(pg_url: &str) -> sqlx::Result<Self> {
        if !sqlx::Postgres::database_exists(pg_url).await? {
            sqlx::Postgres::create_database(pg_url).await?;
        }

        let pool = PgPool::connect(pg_url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}
