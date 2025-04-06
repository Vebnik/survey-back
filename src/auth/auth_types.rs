use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyClaims {
    pub id: String,
    pub iat: u64,
    pub aud: String,
    pub exp: u64,
}
