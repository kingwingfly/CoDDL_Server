pub mod sign;

use std::sync::Arc;

pub use sign::*;
use sqlx::{Pool, Postgres, FromRow};



#[derive(Debug, FromRow)]
struct User {
    id: i32,
    username: String,
    password: String,
}

impl LoginReq {
    pub async fn verify(&self, pgpool: Arc<Pool<Postgres>>) -> bool {
        let sql = format!(
            "SELECT id, username, password FROM userinfo WHERE username='{}'",
            self.username
        );
        if let Ok(result) = sqlx::query_as::<_, User>(sql.as_str()).fetch_one(pgpool.as_ref()).await {
            match result.password.cmp(&self.password) {
                std::cmp::Ordering::Equal => return true,
                _ => return false,
            }
        };
        false
    }
}

impl SignUpReq {
    pub async fn register(&self, pgpool: Arc<Pool<Postgres>>) -> bool {
        let sql = format!(
            "INSERT INTO userinfo (username, password) VALUES ('{}', '{}')",
            self.username, self.password
        );
        match sqlx::query(sql.as_str()).execute(pgpool.as_ref()).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
