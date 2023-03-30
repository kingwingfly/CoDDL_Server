pub mod sign;

use sign::*;
use sqlx::{FromRow, Pool, Postgres, Row};
use std::sync::Arc;

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    username: String,
    password: String,
}

impl LoginReq {
    pub async fn verify(&self, pgpool: Arc<Pool<Postgres>>) -> bool {
        let sql = format!(
            "SELECT password FROM userinfo WHERE username='{}'",
            self.username
        );
        if let Ok(result) = sqlx::query(sql.as_str()).fetch_one(pgpool.as_ref()).await {
            if let Ok(relpw) = result.try_get::<&str, _>("password") {
                match relpw.cmp(&self.password) {
                    std::cmp::Ordering::Equal => return true,
                    _ => return false,
                }
            }
        };
        false
    }
}

impl SignUpReq {
    pub async fn register(&self, pgpool: Arc<Pool<Postgres>>) -> bool {
        let sql = format!(
            "INSERT INTO userinfo (username, password, email) VALUES ('{}', '{}', '{}')",
            self.username, self.password, self.email
        );
        match sqlx::query(sql.as_str()).execute(pgpool.as_ref()).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
