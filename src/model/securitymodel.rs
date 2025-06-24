use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EncodeJWT{
    pub(crate) username: String,
    pub(crate) email: String
}

