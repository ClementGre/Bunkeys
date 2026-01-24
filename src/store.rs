use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use indexmap::IndexMap;
use crate::app::data::{Entry, Section};
use crate::encrypt::{decrypt_string, encrypt_string};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
    /// Section name -> Entry name -> Entry value
    #[serde(flatten)]
    sections: IndexMap<String, IndexMap<String, String>>,
}

impl Store {
    pub fn from_sections(sections: &Vec<Section>) -> Self {
        let sections = sections
            .iter()
            .map(|section| {
                let entries = section
                    .entries
                    .iter()
                    .map(|entry| (entry.key.clone(), entry.value.clone()))
                    .collect();
                (section.name.clone(), entries)
            })
            .collect();
        Self { sections }
    }
    pub fn to_sections(self) -> Vec<Section> {
        self.sections
            .into_iter()
            .map(|(name, entries_map)| Section {
                name,
                entries: entries_map
                    .into_iter()
                    .map(|(key, value)| Entry { key, value })
                    .collect(),
            })
            .collect()
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
                Err(e) => return Err(format!("Failed to decrypt store data: {}", e)),
            }
        } else {
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
                Err(e) => return Err(format!("Failed to encrypt store data: {}", e)),
            }
        } else {
            yaml_data
        };

        // Write the encrypted data to the file
        match fs::write(&path, encrypted_data) {
            Ok(_) => Ok("Store saved successfully!".to_string()),
            Err(e) => Err(format!("Failed to write file: {}", e)),
        }
    }
}
