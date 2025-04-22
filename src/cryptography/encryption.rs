use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use rand::RngCore;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub trait Encryptable {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid input data")]
    InvalidData,
    #[error("Incorrect passphrase")]
    IncorrectPassphrase,
    #[error("Data tampering detected")]
    DataTampered,
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub struct AES256Cypher;

impl Encryptable for AES256Cypher {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if data.is_empty() {
            return Err(CryptoError::InvalidData);
        }

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| CryptoError::InternalError("Failed to create cipher".into()))?;

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|_| CryptoError::InternalError("Encryption failed".into()))?;

        let mut result: Vec<u8> = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if data.len() <= 12 {
            return Err(CryptoError::InvalidData); // AES-GCM requires a 12 bytes nonce
        }
        let (nonce_slice, ciphertext) = data.split_at(12);
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| CryptoError::InternalError("Failed to create cipher".into()))?;

        let nonce = aes_gcm::Nonce::from_slice(nonce_slice);
        let decrypted_data = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::DataTampered)?;
        if decrypted_data.is_empty() {
            return Err(CryptoError::DataTampered);
        }
        Ok(decrypted_data)
    }
}

pub struct Cypher<'a> {
    cipher: &'a dyn Encryptable,
    data: Option<Vec<u8>>,
    passphrase: Option<Vec<u8>>,
    iterations: Option<u32>,
}

impl<'a> Cypher<'a> {
    pub fn new(cipher: &'a dyn Encryptable) -> Self {
        Cypher {
            cipher,
            data: None,
            passphrase: None,
            iterations: Some(500_000),
        }
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_passphrase(mut self, passphrase: Vec<u8>) -> Self {
        self.passphrase = Some(passphrase);
        self
    }

    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = Some(iterations);
        self
    }

    fn derive_key(&self, salt: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let iterations = self.iterations.unwrap_or(100_000);
        let passphrase = self.passphrase.as_ref().unwrap();

        let mut key = vec![0u8; 32];
        pbkdf2_hmac::<Sha256>(passphrase, salt, iterations, &mut key);

        Ok(key)
    }

    fn generate_random_salt(&self) -> Vec<u8> {
        let mut salt = vec![0u8; 16];
        rand::rng().fill_bytes(&mut salt);
        salt
    }

    fn prepare_result(salt: &[u8], encrypted_data: Vec<u8>) -> Vec<u8> {
        let mut result = Vec::with_capacity(encrypted_data.len() + salt.len());
        result.extend_from_slice(salt);
        result.extend_from_slice(&encrypted_data);
        result
    }

    pub fn encrypt(&self) -> Result<Vec<u8>, CryptoError> {
        if self.data.is_none() || self.passphrase.is_none() {
            return Err(CryptoError::InvalidData);
        }

        let salt = self.generate_random_salt();
        let key = self.derive_key(&salt)?;
        let data = self.data.as_ref().unwrap();

        let encrypted_data = self.cipher.encrypt(data, &key)?;
        let result = Self::prepare_result(&salt, encrypted_data);

        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if self.passphrase.is_none() || data.len() < 16 {
            return Err(CryptoError::InvalidData);
        }

        let salt = &data[..16];
        let key = self.derive_key(&salt)?;
        let encrypted_data = &data[salt.len()..];
        self.cipher.decrypt(encrypted_data, &key)
    }
}

#[cfg(test)]
mod symmetric_cipher_tests {
    use super::*;

    #[test]
    fn test_empty_data_encryption() {
        let cipher = AES256Cypher;
        let result = cipher.encrypt(&[], "key".as_bytes());
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::InvalidData)));
    }
}

#[cfg(test)]
mod cypher_builder_tests {

    use super::*;

    #[test]
    fn test_with_data() {
        let builder = Cypher::new(&AES256Cypher).with_data(vec![7, 8, 9]);
        assert_eq!(builder.data, Some(vec![7, 8, 9]));
    }

    #[test]
    fn test_with_passphrase() {
        let builder = Cypher::new(&AES256Cypher).with_passphrase("secret".into());
        assert_eq!(builder.passphrase, Some("secret".into()));
    }

    #[test]
    fn test_with_iterations() {
        let builder = Cypher::new(&AES256Cypher).with_iterations(1000);
        assert_eq!(builder.iterations, Some(1000));
    }
    #[test]
    fn test_derive_key() {
        let builder = Cypher::new(&AES256Cypher)
            .with_passphrase("secret".into())
            .with_iterations(1000);

        let salt = b"salt";
        let key = builder.derive_key(salt).unwrap();
        assert_eq!(key.len(), 32);
    }
    #[test]
    fn test_prepare_result() {
        let salt = b"salt";
        let encrypted_data = vec![1, 2, 3];
        let result = Cypher::prepare_result(salt, encrypted_data.clone());
        assert_eq!(result.len(), salt.len() + encrypted_data.len());
        assert_eq!(&result[..salt.len()], salt);
        assert_eq!(&result[salt.len()..], &encrypted_data[..]);
    }
    #[test]
    fn test_decrypt() {
        // Seems like some pirates are trying to steal our secrets
        let secret_data = "grok, grok, grok!";
        let data = secret_data.as_bytes(); 

        // Encrypt the data
        let result = Cypher::new(&AES256Cypher) 
            .with_data(data.into())
            .with_passphrase("secret".into())
            .encrypt();

        // Decrypt the data
        let decoded_bytes = Cypher::new(&AES256Cypher) 
            .with_passphrase("secret".into())
            .decrypt(&result.unwrap()).unwrap();
        let plaintext = String::from_utf8(decoded_bytes).unwrap_or_else(|e| {
            String::from_utf8_lossy(&e.into_bytes()).into_owned()
        });

        assert_eq!(plaintext, secret_data); 
        println!("deciphered: {}", plaintext);
    }
}
