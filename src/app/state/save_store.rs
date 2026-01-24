use crate::app::data::AppData;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::app::state::{AppState, AppStateEvents};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Position, Span, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::env;
use std::path::PathBuf;
use crate::app::text_input::TextInput;
use crate::store::Store;

#[derive(Debug, Clone, PartialEq)]
pub struct SaveStoreState {
    encrypted: bool,
    path: TextInput,
}

impl SaveStoreState {
    pub fn new(encrypted: bool, path: Option<String>) -> Self {
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
            path: TextInput::new(path),
        }
    }
    fn from_text_input(encrypted: bool, path: TextInput) -> Self {
        Self { encrypted, path }
    }
}

impl AppStateEvents for SaveStoreState {
    fn get_title(&self, _data: &AppData) -> String {
        "Save Store".to_string()
    }

    fn get_footer(&self, _data: &AppData) -> &'static str {
        "[Esc: Cancel] [⏎ Enter: Continue]"
    }

    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match key.code {
            KeyCode::Char(c) => {
                SaveStoreState::from_text_input(self.encrypted, self.path.with_insert_char(c))
                    .into()
            }
            KeyCode::Left => {
                SaveStoreState::from_text_input(self.encrypted, self.path.with_move_left()).into()
            }
            KeyCode::Right => {
                SaveStoreState::from_text_input(self.encrypted, self.path.with_move_right()).into()
            }
            KeyCode::Backspace => {
                SaveStoreState::from_text_input(self.encrypted, self.path.with_delete_char()).into()
            }
            KeyCode::Enter => {
                if !self.path.get_text().is_empty() {
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
            Line::from(Span::styled(
                self.path.get_text(),
                Style::default().fg(Color::Yellow),
            )),
        ];
        frame.set_cursor_position(Position::new(
            area.x + self.path.cursor_char_pos() as u16,
            area.y + 2,
        ));

        if !self.encrypted {
            text.push(Line::from(""));
            text.push(Line::from(Span::styled(
                "⚠ WARNING: Your store will not be stored encrypted with this function.",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
        }

        let paragraph = Paragraph::new(text);

        frame.render_widget(paragraph, area);
    }
}

impl SaveStoreState {
    fn try_save_store(self, data: &mut AppData) -> AppState {
        let path = PathBuf::from(&self.path.get_text());

        let key = if self.encrypted {
            Some(data.store_key.clone().unwrap())
        } else {
            None
        };

        match Store::from_sections(&data.sections).save(key, path.clone()) {
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
