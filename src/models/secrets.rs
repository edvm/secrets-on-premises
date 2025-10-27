use chrono::{DateTime, Duration, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::cryptography::encryption::{AES256Cypher, CryptoError, Cypher};
use crate::error::{AppError, Result};

#[derive(Debug, sqlx::FromRow)]
pub struct Secret {
    pub id: i64,
    pub uuid: String,
    pub ciphertext: Vec<u8>,
    pub maxviews: Option<i64>,
    pub exp: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

impl Secret {
    pub async fn create(
        text: &[u8],
        passphrase: &[u8],
        maxviews: Option<u16>,
        expiration_days: Option<u8>,
        pool: &SqlitePool,
    ) -> Result<Self> {
        let cipher = Cypher::new(&AES256Cypher)
            .with_data(text.to_vec())
            .with_passphrase(passphrase.to_vec());

        let ciphertext = cipher.encrypt()?;
        let uuid = Uuid::new_v4().to_string();
        let maxviews = maxviews.unwrap_or(1) as i64;
        let exp = expiration_days.unwrap_or(1) as i64;

        let secret = sqlx::query_as::<_, Secret>(
            r#"
            INSERT INTO secrets (uuid, ciphertext, maxviews, exp)
            VALUES (?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&uuid)
        .bind(&ciphertext)
        .bind(maxviews)
        .bind(exp)
        .fetch_one(pool)
        .await?;

        Ok(secret)
    }

    pub async fn find_by_uuid(uuid: &str, pool: &SqlitePool) -> Result<Self> {
        let secret = sqlx::query_as::<_, Secret>(
            r#"
            SELECT * FROM secrets
            WHERE uuid = ? AND maxviews > 0
            "#,
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Secret not found or already consumed".to_string()))?;

        if secret.has_expired() {
            return Err(AppError::NotFound("Secret has expired".to_string()));
        }

        if secret.has_reached_maxviews() {
            return Err(AppError::NotFound("Secret has reached maximum views".to_string()));
        }

        Ok(secret)
    }

    pub async fn decrement_maxviews(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE secrets
            SET maxviews = maxviews - 1, updated_at = datetime('now')
            WHERE uuid = ?
            "#,
        )
        .bind(&self.uuid)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn decrypt(&self, passphrase: &[u8]) -> Result<String> {
        let plaintext = Cypher::new(&AES256Cypher)
            .with_passphrase(passphrase.into())
            .decrypt(&self.ciphertext)?;

        let secret = String::from_utf8(plaintext)
            .map_err(|_| AppError::Crypto(CryptoError::InvalidData))?;

        Ok(secret)
    }

    pub fn has_reached_maxviews(&self) -> bool {
        self.maxviews.map_or(false, |v| v == 0)
    }

    pub fn has_expired(&self) -> bool {
        if let Some(exp_days) = self.exp {
            if let Ok(created_at) = self.created_at.parse::<DateTime<Utc>>() {
                let expiration_date = created_at + Duration::days(exp_days);
                let now = Utc::now();
                return now > expiration_date;
            }
        }
        false
    }

    pub fn uuid_as_uuid(&self) -> std::result::Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.uuid)
    }

    pub fn created_at_as_datetime(&self) -> std::result::Result<DateTime<Utc>, chrono::ParseError> {
        self.created_at.parse::<DateTime<Utc>>()
    }
}
