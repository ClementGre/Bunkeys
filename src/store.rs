use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::encrypt::{decrypt_string, encrypt_string};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
    /// Section name -> Entry name -> Entry value
    #[serde(flatten)]
    store: HashMap<String, HashMap<String, String>>,
}

impl Store {
    // Getters
    pub fn list_sections(&self) -> Vec<&String> {
        self.store.keys().collect()
    }
    pub fn list_entries(&self, section: &str) -> Vec<&String> {
        self.store
            .get(section)
            .map(|h| h.keys().collect())
            .unwrap_or(Vec::default())
    }
    pub fn get(&self, section: &String, key: &String) -> Option<&String> {
        self.store.get(section)?.get(key)
    }
    // Setters
    pub fn set(&mut self, section: &str, key: &str, value: String) {
        self.store
            .entry(section.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value);
    }
    pub fn add_section(&mut self, section: String) {
        self.store.insert(section, HashMap::new());
    }
    pub fn remove_section(&mut self, section: &str) {
        self.store.remove(section);
    }
    pub fn remove_entry(&mut self, section: &str, entry: &str) {
        self.store
            .entry(section.to_string())
            .or_insert_with(HashMap::new)
            .remove(entry);
    }

    pub fn load(encryption_key: Option<Vec<u8>>, path: PathBuf) -> Result<Store, String> {
        // Read the encrypted data from the file
       let encrypted_data = match fs::read(path) {
            Ok(encrypted_data) => encrypted_data,
            Err(e) => return Err(format!("Failed to read file: {}", e)),
        };

        // Decrypt the YAML data
        let yaml_data = if let Some(key) = encryption_key {
            match decrypt_string(key, encrypted_data) {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to decrypt store data: {}", e))
            }
        }else {
            encrypted_data
        };

        // Deserialize the YAML data into a Store object
        serde_yaml::from_slice::<Store>(&yaml_data)
            .map_err(|e| format!("Failed to parse store data: {}", e))
    }

    // Save
    pub fn save(&self, encryption_key: Option<Vec<u8>>, path: PathBuf) -> Result<String, String> {
        // Serialize the store data into YAML
        let yaml_data = match serde_yaml::to_string(self) {
            Ok(data) => data.into_bytes(),
            Err(e) => return Err(format!("Failed to serialize store data: {}", e)),
        };

        // Encrypt the YAML data
        let encrypted_data = if let Some(key) = encryption_key {
            match encrypt_string(key, yaml_data) {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to encrypt store data: {}", e))
            }
        }else {
            yaml_data
        };

        // Write the encrypted data to the file
        match fs::write(&path, encrypted_data) {
            Ok(_) => Ok("Store saved successfully!".to_string()),
            Err(e) => Err(format!("Failed to write file: {}", e)),
        }
    }
}
