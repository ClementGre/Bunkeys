use crate::app::data::AppData;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::app::state::AppStateEvents;
use crate::app::text_input::TextInput;
use crate::app::AppState;
use crate::bip39;
use crate::store::Store;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AppLoadStoreStep {
    EnterPath(TextInput),
    EnterKey(TextInput),
}
impl Default for AppLoadStoreStep {
    fn default() -> Self {
        let current_path = env::current_dir().unwrap().to_string_lossy().to_string();
        AppLoadStoreStep::EnterPath(TextInput::new(current_path + "/store.enc"))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LoadStoreState {
    encrypted: bool,
    step: AppLoadStoreStep,
}
impl LoadStoreState {
    pub fn new(encrypted: bool, path: Option<String>) -> Self {
        let path = path.unwrap_or_else(|| {
            let current_path = env::current_dir().unwrap().to_string_lossy().to_string();
            if encrypted {
                current_path + "/store.enc"
            } else {
                current_path + "/store.yaml"
            }
        });
        Self::new_path(encrypted, TextInput::new(path))
    }
    fn new_path(encrypted: bool, path: TextInput) -> Self {
        Self {
            encrypted,
            step: AppLoadStoreStep::EnterPath(path),
        }
    }
    fn new_key(encrypted: bool, raw_key: TextInput) -> Self {
        Self {
            encrypted,
            step: AppLoadStoreStep::EnterKey(raw_key),
        }
    }
}

impl AppStateEvents for LoadStoreState {
    fn get_title(&self, _data: &AppData) -> String {
        match &self.step {
            AppLoadStoreStep::EnterPath(_) => "Load Store - Enter Path".to_string(),
            AppLoadStoreStep::EnterKey(_) => "Load Store - Enter Key".to_string(),
        }
    }

    fn get_footer(&self, _data: &AppData) -> &'static str {
        "[Esc: Cancel] [⏎ Enter: Continue]"
    }

    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match self.step.clone() {
            AppLoadStoreStep::EnterPath(path) => match key.code {
                KeyCode::Char(c) => {
                    LoadStoreState::new_path(self.encrypted, path.with_insert_char(c)).into()
                }
                KeyCode::Backspace => {
                    LoadStoreState::new_path(self.encrypted, path.with_delete_char()).into()
                }
                KeyCode::Left => {
                    LoadStoreState::new_path(self.encrypted, path.with_move_left()).into()
                }
                KeyCode::Right => {
                    LoadStoreState::new_path(self.encrypted, path.with_move_right()).into()
                }
                KeyCode::Enter => {
                    if !path.text.is_empty() {
                        data.store_path = Some(PathBuf::from(path.text.clone()));
                        if !self.encrypted {
                            return self.try_load_store(data, None);
                        }
                        return LoadStoreState::new_key(self.encrypted, TextInput::default()).into();
                    }
                    LoadStoreState::new_path(self.encrypted, path.clone()).into()
                }
                KeyCode::Esc => MainMenuState::new(MainMenuAction::LoadStore).into(),
                _ => self.clone().into(),
            },
            AppLoadStoreStep::EnterKey(raw_key) => match key.code {
                KeyCode::Char(c) => {
                    LoadStoreState::new_key(self.encrypted, raw_key.with_insert_char(c)).into()
                }
                KeyCode::Backspace => {
                    LoadStoreState::new_key(self.encrypted, raw_key.with_delete_char()).into()
                }
                KeyCode::Left => {
                    LoadStoreState::new_key(self.encrypted, raw_key.with_move_left()).into()
                }
                KeyCode::Right => {
                    LoadStoreState::new_key(self.encrypted, raw_key.with_move_right()).into()
                }
                KeyCode::Enter => {
                    if !raw_key.text.is_empty() {
                        return match Self::parse_raw_key(raw_key.text.clone()) {
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
        let (prompt, input) = match &self.step {
            AppLoadStoreStep::EnterPath(input) => {
                ("Enter store file path:", input)
            }
            AppLoadStoreStep::EnterKey(input) => (
                "Enter key (hex or BIP39 mnemonic):",
                input,
            ),
        };

        let text = vec![
            Line::from(prompt),
            Line::from(""),
            Line::from(Span::styled(input.text.clone(), Style::default().fg(Color::Yellow))),
        ];
        frame.set_cursor_position(Position::new(
            area.x + input.cursor_pos as u16,
            area.y + 2,
        ));

        let paragraph = Paragraph::new(text);

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
            return self.clone().into();
        }

        // Try to load and decrypt the file
        match Store::load(key.clone(), path.clone()) {
            Ok(store) => {
                data.sections = store.to_sections();
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
