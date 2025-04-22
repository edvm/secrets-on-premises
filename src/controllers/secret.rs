#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::secrets::{self, ActiveModel};

#[derive(Serialize, Default)]
pub struct SecretResponse {
    pub uuid: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub length: usize,
    pub secret: Option<String>,
    pub url: Option<String>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptParams {
    pub text: String,
    pub passphrase: String,
    pub maxviews: Option<u16>,
    pub expiration_days: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptParams {
    pub passphrase: String,
}

#[debug_handler]
pub async fn encrypt(
    State(ctx): State<AppContext>,
    Json(params): Json<EncryptParams>,
) -> Result<Response> {
    let passphrase = params.passphrase.as_bytes();
    let text = params.text.as_bytes();
    let item = ActiveModel::encrypt(text, passphrase, params.maxviews, params.expiration_days).unwrap();
    let item = item.insert(&ctx.db).await?;
    let response = SecretResponse {
        uuid: item.uuid,
        created_at: item.created_at,
        length: text.len(),
        secret: None,
        url: Some(format!("/api/secrets/decrypt/{}", item.uuid)),
    };
    format::json(response)
}

#[debug_handler]
pub async fn decrypt(
    Path(id): Path<Uuid>,
    State(ctx): State<AppContext>,
    Json(params): Json<DecryptParams>,
) -> Result<Response> {
    let secret = secrets::Model::find_by_uuid(id, &ctx.db).await
        .map_err(|err| match err {
            ModelError::EntityNotFound => Error::BadRequest("Secret not found".to_string()),
            _ => Error::BadRequest("Internal server error".to_string()),
        })?;

    let passphrase = params.passphrase.as_bytes();
    let plaintext = secret.decrypt(passphrase)
        .or(Err(Error::BadRequest("Decryption failed".to_string())))?;

    ActiveModel::decrement_maxviews_by_uuid(id, &ctx.db).await?;

    let response = SecretResponse {
        uuid: secret.uuid,
        created_at: secret.created_at,
        length: plaintext.len(),
        secret: Some(plaintext),
        url: None,
    };

    format::json(response)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/secrets")
        .add("/encrypt/", post(encrypt))
        .add("/decrypt/{id}", post(decrypt))
}
