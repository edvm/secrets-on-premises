use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::models::secrets::Secret;
use crate::state::AppState;

#[derive(Serialize)]
pub struct SecretResponse {
    pub uuid: String,
    pub created_at: String,
    pub length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[axum::debug_handler]
pub async fn encrypt(
    State(state): State<AppState>,
    Json(params): Json<EncryptParams>,
) -> Response {
    let passphrase = params.passphrase.as_bytes();
    let text = params.text.as_bytes();

    let secret = match Secret::create(
        text,
        passphrase,
        params.maxviews,
        params.expiration_days,
        &state.db,
    )
    .await {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };

    let response = SecretResponse {
        uuid: secret.uuid.clone(),
        created_at: secret.created_at.clone(),
        length: text.len(),
        secret: None,
        url: Some(format!("/api/secrets/decrypt/{}", secret.uuid)),
    };

    Json(response).into_response()
}

pub async fn decrypt(
    Path(uuid): Path<String>,
    State(state): State<AppState>,
    Json(params): Json<DecryptParams>,
) -> Response {
    let secret = match Secret::find_by_uuid(&uuid, &state.db).await {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };

    let passphrase = params.passphrase.as_bytes();
    let plaintext = match secret.decrypt(passphrase) {
        Ok(p) => p,
        Err(e) => return e.into_response(),
    };

    if let Err(e) = secret.decrement_maxviews(&state.db).await {
        return e.into_response();
    }

    let response = SecretResponse {
        uuid: secret.uuid.clone(),
        created_at: secret.created_at.clone(),
        length: plaintext.len(),
        secret: Some(plaintext),
        url: None,
    };

    Json(response).into_response()
}
