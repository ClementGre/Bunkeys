use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use clap::builder::Str;

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

    // Load
    pub fn load_encrypted(path: PathBuf) -> Result<Store, String> {
       let encrypted_data = match fs::read(path) {
            Ok(encrypted_data) => encrypted_data,
            Err(e) => return Err(format!("Failed to read file: {}", e)),
        };

        // TODO: implement encryption
        let yaml_data = encrypted_data;

        serde_yaml::from_slice::<Store>(&yaml_data)
            .map_err(|e| format!("Failed to parse store data: {}", e))
    }

    // Save
    pub fn save_encrypted(&self, path: PathBuf) -> Result<String, String> {
        // Serialize the store data to YAML
        let yaml_data = match serde_yaml::to_string(self) {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed to serialize store data: {}", e)),
        };

        // TODO: implement encryption
        let encrypted_data = yaml_data;

        // Write to file
        match fs::write(&path, encrypted_data) {
            Ok(_) => Ok("Store saved successfully!".to_string()),
            Err(e) => Err(format!("Failed to write file: {}", e)),
        }
    }
}
