use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum KeyStoreError {
    KeyNotFound,
    LockError,
}

impl fmt::Display for KeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyStoreError::KeyNotFound => write!(f, "Key not found"),
            KeyStoreError::LockError => write!(f, "Failed to acquire lock"),
        }
    }
}

impl std::error::Error for KeyStoreError {}

#[derive(Clone)]
pub struct KeyStore {
    keys: Arc<Mutex<HashMap<String, (String, bool)>>>, // Public address to (private key, in-use flag) mapping
    notify: Arc<Notify>,
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            keys: Arc::new(Mutex::new(HashMap::new())),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn add_key(&self, public_address: String, private_key: String) {
        let mut keys = self.keys.lock().await;
        keys.insert(public_address, (private_key, false));
    }

    pub async fn acquire_key(&self) -> Result<String, KeyStoreError> {
        loop {
            let mut keys = self.keys.lock().await;
            if let Some((public_address, (_, in_use))) =
                keys.iter_mut().find(|(_, (_, in_use))| !*in_use)
            {
                *in_use = true;
                return Ok(public_address.clone());
            }
            drop(keys); // Release the lock before waiting
            self.notify.notified().await;
        }
    }

    pub async fn release_key(&self, public_address: String) -> Result<(), KeyStoreError> {
        let mut keys = self.keys.lock().await;
        if let Some((_, in_use)) = keys.get_mut(&public_address) {
            *in_use = false;
            self.notify.notify_one();
            Ok(())
        } else {
            Err(KeyStoreError::KeyNotFound)
        }
    }

    pub async fn get_private_key(&self, public_address: &str) -> Result<String, KeyStoreError> {
        let keys = self.keys.lock().await;
        keys.get(public_address)
            .map(|(private_key, _)| private_key.clone())
            .ok_or(KeyStoreError::KeyNotFound)
    }

    pub async fn len(&self) -> usize {
        let keys = self.keys.lock().await;
        keys.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::sync::Notify;

    #[tokio::test]
    async fn test_get_private_key() {
        let mut keys = HashMap::new();
        keys.insert("address1".to_string(), ("private_key1".to_string(), true));
        let keystore = KeyStore {
            keys: Arc::new(Mutex::new(keys)),
            notify: Arc::new(Notify::new()),
        };

        // Test for valid key retrieval
        let private_key = keystore.get_private_key("address1").await;
        assert_eq!(private_key.unwrap(), "private_key1");

        // Test for key not found
        let private_key = keystore.get_private_key("address2").await;
        assert!(private_key.is_err());
        assert_eq!(private_key.unwrap_err(), KeyStoreError::KeyNotFound);
    }

    #[tokio::test]
    async fn test_len() {
        let mut keys = HashMap::new();
        keys.insert("address1".to_string(), ("private_key1".to_string(), true));
        keys.insert("address2".to_string(), ("private_key2".to_string(), true));
        let keystore = KeyStore {
            keys: Arc::new(Mutex::new(keys)),
            notify: Arc::new(Notify::new()),
        };

        // Test for correct length
        let len = keystore.len().await;
        assert_eq!(len, 2);
    }
}