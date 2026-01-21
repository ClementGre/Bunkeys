use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem};
use crate::app::AppState;
use crate::app::data::AppData;
use crate::app::state::AppStateEvents;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EditStoreState;

impl EditStoreState {
    pub fn new() -> Self {
        Self
    }
}

impl AppStateEvents for EditStoreState {
    fn get_title(&self, _data: &AppData) -> String {
        "Edit Store".to_string()
    }

    fn get_footer(&self, _data: &AppData) -> &'static str {
        "[Esc: Cancel] [ ↑/↓: Navigate ] [ ⏎ Enter: Edit ]"
    }

    fn handle_key(&self, _data: &mut AppData, key: KeyEvent) -> AppState {
        match key.code {
            KeyCode::Esc => {
                MainMenuState::new(MainMenuAction::EditStore).into()
            }
            _ => {
                // TODO: Implement store editing (add/edit/delete sections and entries)
                self.clone().into()
            }
        }
    }

    fn render(&self, data: &AppData, frame: &mut Frame, area: Rect) {
        let mut items = Vec::new();

        if data.store_data.list_sections().len() == 0 {
            items.push(ListItem::new("Store is empty"));
        } else {
            for section in data.store_data.list_sections() {
                items.push(ListItem::new(format!("{}:", section)));
                for entry in data.store_data.list_entries(section) {
                    items.push(ListItem::new(format!("  {}: {}", entry, data.store_data.get(section, entry).unwrap())))
                }
            }
        }

        let list = List::new(items);
        frame.render_widget(list, area);
    }
}
