use crate::app::data::AppData;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::app::state::AppStateEvents;
use crate::app::AppState;
use crate::bip39;
use crate::store::Store;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AppLoadStoreStep {
    EnterPath(String),
    EnterKey(String),
}
impl Default for AppLoadStoreStep {
    fn default() -> Self {
        let current_path = env::current_dir().unwrap().to_string_lossy().to_string();
        AppLoadStoreStep::EnterPath(current_path + "/store.enc")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LoadStoreState {
    encrypted: bool,
    step: AppLoadStoreStep,
}
impl LoadStoreState {
    pub fn new_path(encrypted: bool, path: Option<String>) -> Self {
        let path = path.unwrap_or_else(|| {
            let current_path = env::current_dir().unwrap().to_string_lossy().to_string();
            if encrypted {
                current_path + "/store.enc"
            } else {
                current_path + "/store.yaml"
            }
        });
        Self {
            encrypted,
            step: AppLoadStoreStep::EnterPath(path),
        }
    }
    fn new_key(encrypted: bool, key: String) -> Self {
        Self {
            encrypted,
            step: AppLoadStoreStep::EnterKey(key),
        }
    }
}

impl AppStateEvents for LoadStoreState {
    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match self.step.clone() {
            AppLoadStoreStep::EnterPath(mut path) => match key.code {
                KeyCode::Char(c) => {
                    path.push(c);
                    LoadStoreState::new_path(self.encrypted, Some(path)).into()
                }
                KeyCode::Backspace => {
                    path.pop();
                    LoadStoreState::new_path(self.encrypted, Some(path)).into()
                }
                KeyCode::Enter => {
                    if !path.is_empty() {
                        data.store_path = Some(PathBuf::from(path));
                        if !self.encrypted {
                            return self.try_load_store(data, None);
                        }
                        return LoadStoreState::new_key(self.encrypted, String::default()).into();
                    }
                    LoadStoreState::new_path(self.encrypted, Some(path)).into()
                }
                KeyCode::Esc => MainMenuState::new(MainMenuAction::LoadStore).into(),
                _ => self.clone().into(),
            },
            AppLoadStoreStep::EnterKey(mut raw_key) => match key.code {
                KeyCode::Char(c) => {
                    raw_key.push(c);
                    LoadStoreState::new_key(self.encrypted, raw_key).into()
                }
                KeyCode::Backspace => {
                    raw_key.pop();
                    LoadStoreState::new_key(self.encrypted, raw_key).into()
                }
                KeyCode::Enter => {
                    if !raw_key.is_empty() {
                        return match Self::parse_raw_key(raw_key.clone()) {
                            Ok(key) => self.try_load_store(data, Some(key)),
                            Err(e) => {
                                data.error = Some(e);
                                self.clone().into()
                            }
                        };
                    }
                    LoadStoreState::new_key(self.encrypted, raw_key).into()
                }
                KeyCode::Esc => MainMenuState::new(MainMenuAction::LoadStore).into(),
                _ => self.clone().into(),
            },
        }
    }

    fn render(&self, _data: &AppData, frame: &mut Frame, area: Rect) {
        let (title, prompt, input) = match &self.step {
            AppLoadStoreStep::EnterPath(input) => {
                ("Load Store - Enter Path", "Enter store file path:", input)
            }
            AppLoadStoreStep::EnterKey(input) => (
                "Load Store - Enter Key",
                "Enter key (hex or BIP39 mnemonic):",
                input,
            ),
        };

        let text = vec![
            Line::from(prompt),
            Line::from(""),
            Line::from(Span::styled(input, Style::default().fg(Color::Yellow))),
        ];

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_bottom("[Esc: Cancel] [Enter: Continue]"),
        );

        frame.render_widget(paragraph, area);
    }
}

impl LoadStoreState {
    fn parse_raw_key(raw_key: String) -> Result<Vec<u8>, String> {
        Ok(if raw_key.contains(' ') {
            // It's a mnemonic
            match bip39::Bip39::new() {
                Ok(bip39) => match bip39.decode(&raw_key) {
                    Ok(bigint) => {
                        let bytes = bigint.to_bytes_be();
                        if bytes.len() <= 32 {
                            let mut key = vec![0u8; 32 - bytes.len()];
                            key.extend_from_slice(&bytes);
                            key
                        } else {
                            return Err("Key too long".to_string());
                        }
                    }
                    Err(e) => {
                        return Err(format!("Failed to decode mnemonic: {}", e));
                    }
                },
                Err(e) => {
                    return Err(format!("Failed to initialize BIP39: {}", e));
                }
            }
        } else {
            // Try to parse as hex
            match hex::decode(&raw_key) {
                Ok(bytes) if bytes.len() == 32 => bytes,
                _ => {
                    return Err(
                        "Invalid key format. Use hex (64 chars) or BIP39 mnemonic".to_string()
                    );
                }
            }
        })
    }

    fn try_load_store(&self, data: &mut AppData, key: Option<Vec<u8>>) -> AppState {
        let path = match &data.store_path {
            Some(p) => p,
            None => {
                data.error = Some("No path specified".to_string());
                return self.clone().into();
            }
        };

        if self.encrypted && key.is_none() {
            data.error = Some("Encryption key is undefined".to_string());
            return self.clone().into()
        }

        // Try to load and decrypt the file
        match Store::load(key.clone(), path.clone()) {
            Ok(store) => {
                data.store_data = store;
                if key.is_some() {
                    data.store_key = key;
                }
                data.message = Some("Store loaded successfully!".to_string());
                MainMenuState::new(MainMenuAction::EditStore).into()
            }
            Err(e) => {
                data.error = Some(e);
                MainMenuState::new(MainMenuAction::LoadStore).into()
            }
        }
    }
}
