use std::path::PathBuf;
use crate::store::Store;

pub struct AppData {
    // Status
    pub message: Option<String>,
    pub error: Option<String>,

    // Store data
    pub store_path: Option<PathBuf>,
    pub store_key: Option<Vec<u8>>,
    pub store_data: Store,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            message: None,
            error: None,
            store_path: None,
            store_key: None,
            store_data: Store::default(),
        }
    }
}
