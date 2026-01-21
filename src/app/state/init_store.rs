use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use aead::OsRng;
use rand::RngCore;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Paragraph, Wrap};
use crate::app::AppState;
use crate::app::data::{AppData};
use crate::app::state::AppStateEvents;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::bip39;
use crate::store::Store;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InitStoreState {
    generated_key: Vec<u8>,
    generated_mnemonic: String,
}

impl InitStoreState {
    pub fn try_init(data: &mut AppData) -> AppState {
        let mut rng = OsRng::default();
        let mut key = [0u8; 32];
        rng.fill_bytes(&mut key);

        // Convert key to BIP39 mnemonic
        let key_bigint = num_bigint::BigUint::from_bytes_be(&key);
        match bip39::Bip39::new() {
            Ok(bip39) => {
                match bip39.encode(&key_bigint) {
                    Ok(mnemonic) => {
                        data.message = Some("Store initialized successfully!".to_string());
                        Self {
                            generated_key: key.to_vec(),
                            generated_mnemonic: mnemonic,
                        }.into()
                    }
                    Err(e) => {
                        data.error = Some(format!("Failed to encode mnemonic: {}", e));
                        MainMenuState::new(MainMenuAction::InitStore).into()
                    }
                }
            }
            Err(e) => {
                data.error = Some(format!("Failed to initialize BIP39: {}", e));
                MainMenuState::new(MainMenuAction::InitStore).into()
            }
        }
    }
}

impl AppStateEvents for InitStoreState {
    fn get_title(&self, _data: &AppData) -> String {
        "Initialize Store".to_string()
    }

    fn get_footer(&self, _data: &AppData) -> &'static str {
        "[Esc: Cancel] [⏎ Enter: Initialize Store]"
    }

    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match key.code {
            KeyCode::Enter => {
                data.store_key = Some(self.generated_key.clone());
                data.store_data = Store::default();
                data.store_data.set("example_section", "key", "value".to_string());
                data.store_data.set("section2", "entry_name", "val".to_string());
                data.store_data.set("section2", "Nom test", "Secret key".to_string());
                MainMenuState::new(MainMenuAction::EditStore).into()
            }
            KeyCode::Esc => {
                MainMenuState::new(MainMenuAction::InitStore).into()
            }
            _ => self.clone().into()
        }
    }

    fn render(&self, _data: &AppData, frame: &mut Frame, area: Rect) {
        let mut text = Vec::new();

        text.push(Line::from("New 256-bit key generated!").style(Style::default().fg(Color::Green)));
        text.push(Line::from(""));

        // Hex key
        text.push(Line::from(Span::styled(
            "Key (hex):",
            Style::default().fg(Color::Yellow),
        )));
        text.push(Line::from(hex::encode(self.generated_key.clone())));
        text.push(Line::from(""));

        // Mnemonic
        text.push(Line::from(Span::styled(
            "BIP39 Mnemonic:",
            Style::default().fg(Color::Yellow),
        )));
        text.push(Line::from(self.generated_mnemonic.as_str()));
        text.push(Line::from(""));

        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            "⚠ IMPORTANT: Save this key securely! You will need it to access your store.",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));

        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        frame.render_widget(paragraph, area);
    }
}
