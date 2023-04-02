pub mod sign;
pub use sign::*;

use pgpool::PgPool;
use sqlx::Row;

impl LoginReq {
    pub async fn verify(&self, pgpool: &PgPool) -> bool {
        let sql = format!(
            "SELECT password FROM userinfo WHERE username='{}'",
            self.username
        );
        if let Ok(result) = pgpool.query(&sql).await {
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
    pub async fn register(&self, pgpool: &PgPool) -> bool {
        let sql = format!(
            "INSERT INTO userinfo (username, password, email) VALUES ('{}', '{}', '{}')",
            self.username, self.password, self.email
        );
        // The sql colomn has been set to Unique, so it just reject if there's duplicate.
        match pgpool.execute(&sql).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
