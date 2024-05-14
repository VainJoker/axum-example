use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, PgPool};

use crate::{
    library::error::InnerResult,
    models::types::{AccountStatus, Currency, Language},
};

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwAccount {
    pub account_id: i64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub failed_attempt: i32,
    pub status: AccountStatus,
    pub last_login: Option<NaiveDateTime>,

    pub local_currency: Currency,
    pub system_lang: Language,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email_or_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordSchema {
    pub account_id: i64,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBwAccountSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl BwAccount {
    pub async fn register_account(
        db: &PgPool,
        new_bw_account: &CreateBwAccountSchema,
    ) -> InnerResult<Self> {
        let map = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO bw_account (name, email, password) VALUES ($1, $2, $3)
            RETURNING account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            status as "status: _", failed_attempt, last_login,
            created_at,updated_at
            "#,
            new_bw_account.name,
            new_bw_account.email,
            new_bw_account.password
        );

        Ok(map.fetch_one(db).await?)
    }

    pub async fn check_user_exists_by_email(
        db: &PgPool,
        email: &str,
    ) -> InnerResult<Option<bool>> {
        let map = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM bw_account WHERE email = $1)",
            email.to_owned().to_ascii_lowercase()
        );
        Ok(map.fetch_one(db).await?)
    }

    pub async fn check_user_exists_by_account_id(
        db: &PgPool,
        account_id: &i64,
    ) -> InnerResult<Option<bool>> {
        let map = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM bw_account WHERE account_id = $1)",
            account_id
        );
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_user_by_email_or_name(
        db: &PgPool,
        email_or_name: &str,
    ) -> InnerResult<Vec<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"SELECT account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            status as "status: _", failed_attempt, last_login,
            created_at,updated_at
            FROM bw_account WHERE name = $1 or email = $1"#,
            email_or_name
        );
        Ok(map.fetch_all(db).await?)
    }

    pub async fn fetch_user_by_account_id(
        db: &PgPool,
        account_id: i64,
    ) -> InnerResult<Option<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"SELECT account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            status as "status: _", failed_attempt, last_login,
            created_at,updated_at
            FROM bw_account WHERE account_id = $1"#,
            account_id
        );
        Ok(map.fetch_optional(db).await?)
    }

    pub async fn fetch_user_by_email(
        db: &PgPool,
        email: &str,
    ) -> InnerResult<Option<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"SELECT account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            status as "status: _", failed_attempt, last_login,
            created_at,updated_at
            FROM bw_account WHERE email = $1"#,
            email
        );
        Ok(map.fetch_optional(db).await?)
    }

    pub async fn update_last_login(
        db: &PgPool,
        account_id: i64,
    ) -> InnerResult<u64> {
        let map = sqlx::query!(
            r#"UPDATE bw_account set last_login = now()
            WHERE account_id = $1"#,
            account_id
        );
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn update_email_verified_at(
        db: &PgPool,
        account_id: i64,
    ) -> InnerResult<u64> {
        let map = sqlx::query!(
            r#"UPDATE bw_account set email_verified_at = now(), status = 'active'
            WHERE account_id = $1"#,
            account_id
        );
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn update_password_by_account_id(
        db: &PgPool,
        reset_password: &ResetPasswordSchema,
    ) -> InnerResult<u64> {
        let map = sqlx::query!(
            r#"UPDATE bw_account set password = $1
            WHERE account_id = $2"#,
            reset_password.password,
            reset_password.account_id
        );
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn check_user_active_by_account_id(
        db: &PgPool,
        account_id: i64,
    ) -> InnerResult<Option<bool>> {
        let map = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM bw_account WHERE account_id = $1 and status = 'active')",
            account_id
        );
        Ok(map.fetch_one(db).await?)
    }
}
