use x25519_dalek::{StaticSecret, PublicKey};
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, AeadCore};
use chacha20poly1305::aead::{Aead, OsRng};
use rand_core::RngCore;
use std::rc::Rc;
use std::cell::RefCell;
use base64::{Engine as _, engine::general_purpose};

pub struct CryptoService {
    secret: StaticSecret,
    public: PublicKey,
    shared_secret: Option<[u8; 32]>,
    cipher: Option<ChaCha20Poly1305>,
}

impl CryptoService {
    pub fn new() -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        Self {
            secret,
            public,
            shared_secret: None,
            cipher: None,
        }
    }

    pub fn get_public_key_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.public.as_bytes())
    }

    pub fn derive_secret(&mut self, remote_pubkey_base64: &str) -> Result<(), String> {
        let bytes = general_purpose::STANDARD.decode(remote_pubkey_base64)
            .map_err(|e| e.to_string())?;
        
        if bytes.len() != 32 {
            return Err("Invalid public key length".to_string());
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        let remote_public = PublicKey::from(arr);
        
        let shared_secret = self.secret.diffie_hellman(&remote_public);
        let key_bytes = shared_secret.to_bytes();
        self.shared_secret = Some(key_bytes);
        
        let key = Key::from_slice(&key_bytes);
        self.cipher = Some(ChaCha20Poly1305::new(key));
        
        Ok(())
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        if let Some(cipher) = &self.cipher {
            web_sys::console::log_1(&"Generating nonce...".into());
            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
            
            web_sys::console::log_1(&"Encrypting data...".into());
            let ciphertext = cipher.encrypt(&nonce, plaintext)
                .map_err(|e| e.to_string())?;
            
            web_sys::console::log_1(&"Encryption complete!".into());
            
            // Prepend nonce to ciphertext
            let mut result = nonce.to_vec();
            result.extend(ciphertext);
            Ok(result)
        } else {
            Err("Cipher not initialized".to_string())
        }
    }

    pub fn decrypt(&self, ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>, String> {
        if let Some(cipher) = &self.cipher {
            if ciphertext_with_nonce.len() < 12 {
                return Err("Ciphertext too short".to_string());
            }
            
            let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(12);
            let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);
            
            cipher.decrypt(nonce, ciphertext)
                .map_err(|e| e.to_string())
        } else {
            Err("Cipher not initialized".to_string())
        }
    }
    // --- Symmetric Encryption (No Diffie-Hellman) ---

    pub fn generate_key() -> String {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        general_purpose::STANDARD.encode(key)
    }

    pub fn encrypt_with_key(data: &[u8], key_base64: &str) -> Result<Vec<u8>, String> {
        let key_bytes = general_purpose::STANDARD.decode(key_base64)
            .map_err(|e| e.to_string())?;
        
        if key_bytes.len() != 32 {
            return Err("Invalid key length".to_string());
        }

        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        
        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|e| e.to_string())?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    pub fn decrypt_with_key(data: &[u8], key_base64: &str) -> Result<Vec<u8>, String> {
        let key_bytes = general_purpose::STANDARD.decode(key_base64)
            .map_err(|e| e.to_string())?;
        
        if key_bytes.len() != 32 {
            return Err("Invalid key length".to_string());
        }

        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        if data.len() < 12 {
            return Err("Ciphertext too short".to_string());
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| e.to_string())
    }
}
