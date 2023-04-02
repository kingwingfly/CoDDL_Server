use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::env;
use tokio;

pub struct PgPool {
    pool: Pool<Postgres>,
}

impl PgPool {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL in env");
        let pool = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .unwrap();
                check_database(&pool).await; // Ensure table we need exist
                pool
            })
        })
        .join()
        .unwrap();

        Self { pool }
    }

    pub async fn query(&self, sql: &str) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
        sqlx::query(sql).fetch_one(&self.pool).await
    }

    pub async fn execute(&self, sql: &str) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query(sql).execute(&self.pool).await
    }
}

impl Drop for PgPool {
    fn drop(&mut self) {}
}

async fn check_database(pool: &Pool<Postgres>) {
    let sql = "SELECT count(*) FROM information_schema.TABLES WHERE table_name ='userinfo'";
    match sqlx::query(sql)
        .fetch_one(pool)
        .await
        .unwrap()
        .try_get::<i64, _>("count")
    {
        Ok(1) => {
            println!("Database is ready.")
        }
        Err(e) => {
            println!("Something goes run. Can't check the existence of table\n{e}")
        }
        _ => {
            println!("No table. Trying creating.");
            let sql = "
CREATE TABLE userinfo (
    id serial PRIMARY KEY NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255)
)
            ";
            sqlx::query(sql)
                .execute(pool)
                .await
                .expect("failed to create table");
            println!("succeed in creating table")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modify_query_test() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let pool = PgPool::new();
            let sql = "INSERT INTO userinfo (username, password, email) VALUES ('Jake', '123', '111@gmail.com')";
            let result = pool.execute(sql).await;
            assert!(result.is_ok());
            let sql = "SELECT * FROM userinfo WHERE username='Jake'";
            let result = pool.query(sql).await.unwrap()
                .try_get::<String, _>("username")
                .unwrap();
            assert_eq!(result, "Jake");
            let sql = "DELETE from userinfo where username='Jake'";
            let result = pool.execute(sql).await;
            assert!(result.is_ok());
        })
    }
}
