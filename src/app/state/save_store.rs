use std::fs;
use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::data::AppData;
use crate::app::state::{AppState, AppStateEvents};
use crate::app::state::load_store::AppLoadStoreStep;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};

#[derive(Debug, Clone, PartialEq)]
pub struct SaveStoreState {
    path: String,
}

impl SaveStoreState {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl AppStateEvents for SaveStoreState {
    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match key.code {
            KeyCode::Char(c) => {
                let mut new_path = self.path.clone();
                new_path.push(c);
                SaveStoreState::new(new_path).into()
            }
            KeyCode::Backspace => {
                let mut new_path = self.path.clone();
                new_path.pop();
                SaveStoreState::new(new_path).into()
            }
            KeyCode::Enter => {
                if !self.path.is_empty() {
                    self.clone().try_save_store(data)
                } else {
                    data.error = Some("Path cannot be empty".to_string());
                    self.clone().into()
                }
            }
            KeyCode::Esc => {
                MainMenuState::new(MainMenuAction::SaveStore).into()
            }
            _ => self.clone().into()
        }
    }

    fn render(&self, _data: &AppData, frame: &mut Frame, area: Rect) {

        let text = vec![
            Line::from("Enter store file path:"),
            Line::from(""),
            Line::from(Span::styled(&self.path, Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from("Press ENTER to continue, ESC to cancel"),
        ];

        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Save Store"));

        frame.render_widget(paragraph, area);
    }
}

impl SaveStoreState {
    fn try_save_store(self, data: &mut AppData) -> AppState {
        let path = PathBuf::from(&self.path);
        match data.store_data.save_encrypted(path.clone()) {
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
