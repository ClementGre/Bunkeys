use std::{env, fs};
use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::AppState;
use crate::app::data::{AppData};
use crate::app::state::AppStateEvents;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::bip39;
use crate::store::Store;

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
    step: AppLoadStoreStep,
}
impl LoadStoreState {
    fn new_path(path: String) -> Self {
        Self { step: AppLoadStoreStep::EnterPath(path) }
    }
    fn new_key(key: String) -> Self {
        Self { step: AppLoadStoreStep::EnterKey(key) }
    }
}

impl AppStateEvents for LoadStoreState {
    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match self.step.clone() {
            AppLoadStoreStep::EnterPath(mut path) => {
                match key.code {
                    KeyCode::Char(c) => {
                        path.push(c);
                        LoadStoreState::new_path(path).into()
                    }
                    KeyCode::Backspace => {
                        path.pop();
                        LoadStoreState::new_path(path).into()
                    }
                    KeyCode::Enter => {
                        if !path.is_empty() {
                            data.store_path = Some(PathBuf::from(path));
                            return LoadStoreState::new_key(String::default()).into();
                        }
                        LoadStoreState::new_path(path).into()
                    }
                    KeyCode::Esc => {
                        MainMenuState::new(MainMenuAction::LoadStore).into()
                    }
                    _ => self.clone().into()
                }
            }
            AppLoadStoreStep::EnterKey(mut raw_key) => {
                match key.code {
                    KeyCode::Char(c) => {
                        raw_key.push(c);
                        LoadStoreState::new_key(raw_key).into()
                    }
                    KeyCode::Backspace => {
                        raw_key.pop();
                        LoadStoreState::new_key(raw_key).into()
                    }
                    KeyCode::Enter => {
                        if !raw_key.is_empty() {
                            return self.try_load_store(data, raw_key);
                        }
                        LoadStoreState::new_key(raw_key).into()
                    }
                    KeyCode::Esc => {
                        MainMenuState::new(MainMenuAction::LoadStore).into()
                    }
                    _ => self.clone().into()
                }
            }
        }
    }

    fn render(&self, _data: &AppData, frame: &mut Frame, area: Rect) {
        let (title, prompt, input) = match &self.step {
            AppLoadStoreStep::EnterPath(input) => (
                "Load Store - Enter Path",
                "Enter store file path:",
                input
            ),
            AppLoadStoreStep::EnterKey(input) => (
                "Load Store - Enter Key",
                "Enter key (hex or BIP39 mnemonic):",
                input
            ),
        };

        let text = vec![
            Line::from(prompt),
            Line::from(""),
            Line::from(Span::styled(input, Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from("Press ENTER to continue, ESC to cancel"),
        ];

        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(title));

        frame.render_widget(paragraph, area);
    }
}

impl LoadStoreState {
    fn try_load_store(&self, data: &mut AppData, raw_key: String) -> AppState {
        let path = match &data.store_path {
            Some(p) => p,
            None => {
                data.error = Some("No path specified".to_string());
                return self.clone().into();
            }
        };

        let key_bytes = if raw_key.contains(' ') {
            // It's a mnemonic
            match bip39::Bip39::new() {
                Ok(bip39) => {
                    match bip39.decode(&raw_key) {
                        Ok(bigint) => {
                            let bytes = bigint.to_bytes_be();
                            if bytes.len() <= 32 {
                                let mut key = vec![0u8; 32 - bytes.len()];
                                key.extend_from_slice(&bytes);
                                key
                            } else {
                                data.error = Some("Key too long".to_string());
                                return MainMenuState::new(MainMenuAction::LoadStore).into();
                            }
                        }
                        Err(e) => {
                            data.error = Some(format!("Failed to decode mnemonic: {}", e));
                            return MainMenuState::new(MainMenuAction::LoadStore).into();
                        }
                    }
                }
                Err(e) => {
                    data.error = Some(format!("Failed to initialize BIP39: {}", e));
                    return MainMenuState::new(MainMenuAction::LoadStore).into();
                }
            }
        } else {
            // Try to parse as hex
            match hex::decode(&raw_key) {
                Ok(bytes) if bytes.len() == 32 => bytes,
                _ => {
                    data.error = Some("Invalid key format. Use hex (64 chars) or BIP39 mnemonic".to_string());
                    return MainMenuState::new(MainMenuAction::LoadStore).into();
                }
            }
        };

        // Try to load and decrypt the file
        match Store::load_encrypted(path.clone()) {
            Ok(store) => {
                data.store_data = store;
                data.store_key = Some(key_bytes);
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
