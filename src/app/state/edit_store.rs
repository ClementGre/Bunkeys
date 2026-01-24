use crate::app::data::AppData;
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::app::state::AppStateEvents;
use crate::app::text_input::TextInput;
use crate::app::AppState;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem};
use ratatui::Frame;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EditStoreState {
    // Position of the current item
    section_index: Option<usize>,
    entry_index: Option<usize>,
    is_content: bool,
    //
    is_editing: bool,
    input: TextInput,
}

impl EditStoreState {
    pub fn new(data: &mut AppData) -> Self {
        let section_index = if data.sections.len() > 0 {
            Some(0)
        } else {
            None
        };
        let entry_index = section_index.and_then(|si| {
            data.sections
                .get(si)
                .and_then(|s| if s.entries.len() > 0 { Some(0) } else { None })
        });

        Self {
            section_index,
            entry_index,
            is_content: true,
            ..Default::default()
        }
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
            KeyCode::Esc => MainMenuState::new(MainMenuAction::EditStore).into(),
            _ => {
                // TODO: Implement store editing (add/edit/delete sections and entries)
                self.clone().into()
            }
        }
    }

    fn render(&self, data: &AppData, frame: &mut Frame, area: Rect) {
        let mut items = Vec::new();

        for (si, section) in data.sections.iter().enumerate() {
            items.push(ListItem::new(format!("{}:", section.name)));

            for (ei, entry) in section.entries.iter().enumerate() {
                items.push(ListItem::new(format!("  {}: {}", entry.key, entry.value)))
            }
            items.push(ListItem::new("  Add Entry"));
        }
        items.push(ListItem::new("Add Section"));

        let list = List::new(items);
        frame.render_widget(list, area);
    }
}
