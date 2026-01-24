use std::path::PathBuf;

pub struct AppData {
    // Status
    pub message: Option<String>,
    pub error: Option<String>,

    // Store data
    pub store_path: Option<PathBuf>,
    pub store_key: Option<Vec<u8>>,
    pub sections: Vec<Section>,
}
#[derive(Default)]
pub struct Section {
    pub(crate) name: String,
    pub(crate) entries: Vec<Entry>,
}
#[derive(Default)]
pub struct Entry {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            message: None,
            error: None,
            store_path: None,
            store_key: None,
            sections: Vec::new(),
        }
    }
}

impl AppData {
    pub fn get_store_path_string_as_enc(&self) -> Option<String> {
        self.store_path.clone().map(|p| {
            let mut path = p.to_string_lossy().to_string();
            if path.ends_with(".yaml") {
                path = path[..path.len()-5].to_string();
                path.push_str(".enc");
            }
            path
        })
    }
    pub fn get_store_path_string_as_yaml(&self) -> Option<String> {
        self.store_path.clone().map(|p| {
            let mut path = p.to_string_lossy().to_string();
            if path.ends_with(".enc") {
                path = path[..path.len()-4].to_string();
                path.push_str(".yaml");
            }
            path
        })
    }
}
