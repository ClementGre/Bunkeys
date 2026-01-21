use std::env;
use crate::app::data::AppData;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::app::state::{AppState, AppStateEvents};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct SaveStoreState {
    encrypted: bool,
    path: String,
}

impl SaveStoreState {
    pub fn new(encrypted: bool, path: Option<String>) -> Self {
        let path = path.unwrap_or_else(|| {
            let current_path = env::current_dir().unwrap().to_string_lossy().to_string();
            if encrypted {
                current_path + "/store.enc"
            }else {
                current_path + "/store.yaml"
            }
        });
        Self { encrypted, path }
    }
}

impl AppStateEvents for SaveStoreState {
    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match key.code {
            KeyCode::Char(c) => {
                let mut new_path = self.path.clone();
                new_path.push(c);
                SaveStoreState::new(self.encrypted, Some(new_path)).into()
            }
            KeyCode::Backspace => {
                let mut new_path = self.path.clone();
                new_path.pop();
                SaveStoreState::new(self.encrypted, Some(new_path)).into()
            }
            KeyCode::Enter => {
                if !self.path.is_empty() {
                    self.clone().try_save_store(data)
                } else {
                    data.error = Some("Path cannot be empty".to_string());
                    self.clone().into()
                }
            }
            KeyCode::Esc => MainMenuState::new(MainMenuAction::SaveStore).into(),
            _ => self.clone().into(),
        }
    }

    fn render(&self, _data: &AppData, frame: &mut Frame, area: Rect) {
        let mut text = vec![
            Line::from("Enter store file path:"),
            Line::from(""),
            Line::from(Span::styled(&self.path, Style::default().fg(Color::Yellow))),
        ];

        if !self.encrypted {
            text.push(Line::from(""));
            text.push(Line::from(Span::styled(
                "⚠ WARNING: Your store will not be stored encrypted with this function.",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
        }

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Save Store")
                .title_bottom("[Esc: Cancel] [Enter: Continue]"),
        );

        frame.render_widget(paragraph, area);
    }
}

impl SaveStoreState {
    fn try_save_store(self, data: &mut AppData) -> AppState {
        let path = PathBuf::from(&self.path);

        let key = if self.encrypted {
            Some(data.store_key.clone().unwrap())
        }else {
            None
        };

        match data.store_data.save(key, path.clone()) {
            Ok(msg) => {
                data.store_path = Some(path);
                data.message = Some(msg);
                MainMenuState::new(MainMenuAction::SaveStore).into()
            }
            Err(e) => {
                data.error = Some(e);
                self.into()
            }
        }
    }
}
