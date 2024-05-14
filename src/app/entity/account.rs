use serde::{Deserialize, Serialize};

use crate::{
    app::service::jwt::TokenSchema,
    models::{
        bw_account::BwAccount,
        types::{Currency, Language},
    },
};

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: TokenSchema,
    pub name: String,
    pub email: String,
    pub local_currency: Currency,
    pub system_lang: Language,
}

impl LoginResponse {
    pub fn new(token: TokenSchema, user: BwAccount) -> Self {
        Self {
            token,
            name: user.name,
            email: user.email,
            local_currency: user.local_currency,
            system_lang: user.system_lang,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeType {
    ActiveAccount,
    ResetPassword,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveAccountRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub code: String,
    pub password: String,
}
