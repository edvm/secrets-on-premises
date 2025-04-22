use loco_rs::model::ModelError;
use loco_rs::model::ModelResult;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use sea_orm::EntityTrait;
use super::_entities::secrets::Column;
pub use super::_entities::secrets::{self, ActiveModel, Model, Entity};
pub type Secrets = Entity;
use crate::cryptography::encryption::{AES256Cypher, CryptoError, Cypher};


#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {

    pub fn decrypt(
        &self,
        passphrase: &[u8],
    ) -> Result<String, CryptoError> {
        let plaintext = Cypher::new(&AES256Cypher)
            .with_passphrase(passphrase.into())
            .decrypt(&self.ciphertext)?;
        let secret = String::from_utf8(plaintext).map_err(|_| CryptoError::InvalidData)?;
        Ok(secret)
    }

    pub async fn find_by_uuid(uuid: Uuid, db: &DatabaseConnection) -> ModelResult<Model>{
        let secret = secrets::Entity::find()
            .filter(Column::Uuid.eq(uuid))
            .filter(Column::Maxviews.gt(0))
            .one(db)
            .await?;

        if let Some(ref s) = secret {
            if s.has_expired() {
                return Err(ModelError::EntityNotFound);
            }
            if s.has_reached_maxviews() {
                return Err(ModelError::EntityNotFound);
            }
        }

        secret.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub fn has_reached_maxviews(&self) -> bool {
        if let Some(maxviews) = self.maxviews {
            maxviews == 0
        } else {
            false
        }
    }

    pub fn has_expired(&self) -> bool {
        if let Some(exp_days) = self.exp {
            let created_at = self.created_at;
            let expiration_date = created_at + chrono::Duration::days(exp_days as i64);
            let now = chrono::Utc::now();
            now > expiration_date
        } else {
            false
        }
    }

}


impl From<CryptoError> for ModelError {
    fn from(err: CryptoError) -> ModelError {
        ModelError::Message(err.to_string())
    }
}

impl ActiveModel {

    pub fn new(ciphertext: Vec<u8>, maxviews: Option<u16>, expiration_days: Option<u8>) -> Self {
        let maxviews = maxviews.unwrap_or(1);
        let expiration_days = expiration_days.unwrap_or(1);
        Self {
            uuid: Set(Uuid::new_v4()),
            ciphertext: Set(ciphertext),
            maxviews: Set(Some(maxviews)),
            exp: Set(Some(expiration_days)),
            ..Default::default()
        }
    }
    pub fn encrypt(
        text: &[u8],
        passphrase: &[u8],
        maxviews: Option<u16>,
        expiration_days: Option<u8>,
    ) -> Result<Self, CryptoError> {
        let cipher = Cypher::new(&AES256Cypher)
            .with_data(text.to_vec())
            .with_passphrase(passphrase.to_vec());
        let ciphertext = cipher.encrypt()?;
        return Ok(Self::new(ciphertext, maxviews, expiration_days));
    }

    pub async fn decrement_maxviews_by_uuid(
        uuid: Uuid,
        db: &DatabaseConnection,
    ) -> Result<(), ModelError> {
        let secret = Model::find_by_uuid(uuid, db).await?;
        let active_model = ActiveModel {
            id: Set(secret.id),
            maxviews: Set(secret.maxviews.map(|v| v - 1)),
            ..Default::default()
        };
        active_model.update(db).await?;
        Ok(())
    }

}

impl Entity {}
