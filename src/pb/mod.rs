pub mod sign;

pub use sign::*;

impl LoginReq {
    pub fn verify(&self) -> bool {
        true
    }
}

impl SignUpReq {
    pub fn register(&self) -> bool {
        true
    }
}