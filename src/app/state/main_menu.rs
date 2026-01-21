use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::prelude::{Color, Rect, Style, Stylize, Constraint, Layout, Direction};
use ratatui::widgets::{Table, Row, Cell};
use ratatui::text::Text;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::app::AppState;
use crate::app::data::AppData;
use crate::app::state::AppStateEvents;
use crate::app::state::init_store::InitStoreState;
use crate::app::state::load_store::LoadStoreState;
use crate::app::state::edit_store::EditStoreState;
use crate::app::state::save_store::SaveStoreState;

#[derive(Debug, Clone, PartialEq, EnumIter, Default)]
pub enum MainMenuAction {
    InitStore,
    #[default]
    LoadStore,
    LoadUnencryptedStore,
    EditStore,
    SaveStore,
    SaveUnencryptedStore,
}

impl MainMenuAction {
    pub fn to_string(&self) -> (&'static str, &'static str) {
        match self {
            MainMenuAction::InitStore => ("Init Store", "Generate new 256-bit key and create empty store"),
            MainMenuAction::LoadStore => ("Load Store", "Load existing store from file"),
            MainMenuAction::LoadUnencryptedStore => ("Load Store Data From Unencrypted File", "Load current store data from unencrypted file"),
            MainMenuAction::EditStore => ("Edit Store", "View and modify store contents"),
            MainMenuAction::SaveStore => ("Save Store", "Save store to file"),
            MainMenuAction::SaveUnencryptedStore => ("Save Unencrypted Store", "Save store to file without encryption (NOT RECOMMENDED!)"),
        }
    }
    pub fn requires_store(&self) -> bool {
        match self {
            MainMenuAction::LoadUnencryptedStore | MainMenuAction::EditStore | MainMenuAction::SaveStore | MainMenuAction::SaveUnencryptedStore => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MainMenuState {
    selected_action: MainMenuAction,
}

impl MainMenuState {
    pub fn new(action: MainMenuAction) -> Self {
        Self { selected_action: action }
    }
}

impl AppStateEvents for MainMenuState {
    fn get_title(&self, data: &AppData) -> String {
        let mut title = "Main Menu".to_string();
        if data.store_key.is_some() {
            title.push_str(" - 🔓 Store Loaded");
        }
        title
    }

    fn get_footer(&self, _data: &AppData) -> &'static str {
        "[ q: Quit ] [ ↑/↓: Navigate ] [ ⏎ Enter: Select ]"
    }

    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        match key.code {
            KeyCode::Up => {
                let actions: Vec<_> = MainMenuAction::iter().collect();
                let current_idx = actions.iter().position(|a| a == &self.selected_action).unwrap();
                let prev_idx = if current_idx == 0 { actions.len() - 1 } else { current_idx - 1 };
                MainMenuState::new(actions[prev_idx].clone()).into()
            }
            KeyCode::Down => {
                let actions: Vec<_> = MainMenuAction::iter().collect();
                let current_idx = actions.iter().position(|a| a == &self.selected_action).unwrap();
                let next_idx = (current_idx + 1) % actions.len();
                MainMenuState::new(actions[next_idx].clone()).into()
            }
            KeyCode::Enter => {
                if self.selected_action.requires_store() && data.store_key.is_none() {
                    data.error = Some("No store loaded. Please load or init a store first.".to_string());
                    return self.clone().into();
                }
                match self.selected_action {
                    MainMenuAction::InitStore => {
                        InitStoreState::try_init(data)
                    },
                    MainMenuAction::LoadStore => {
                        LoadStoreState::new(true, data.get_store_path_string_as_enc()).into()
                    },
                    MainMenuAction::LoadUnencryptedStore => {
                        LoadStoreState::new(false, data.get_store_path_string_as_yaml()).into()
                    }
                    MainMenuAction::EditStore => {
                        EditStoreState::new().into()
                    }
                    MainMenuAction::SaveStore => {
                        SaveStoreState::new(true, data.get_store_path_string_as_enc()).into()
                    }
                    MainMenuAction::SaveUnencryptedStore => {
                        SaveStoreState::new(false, data.get_store_path_string_as_yaml()).into()
                    }
                }
            }
            _ => self.clone().into()
        }
    }

    fn render(&self, _data: &AppData, frame: &mut Frame, area: Rect) {
        // Create table rows with enhanced styling
        let mut rows = Vec::new();
        let actions: Vec<_> = MainMenuAction::iter().collect();

        for (idx, a) in actions.iter().enumerate() {
            let (name, description) = a.to_string();
            let is_selected = a == &self.selected_action;

            // Add icons for each action
            let icon = match a {
                MainMenuAction::InitStore => "🔑",
                MainMenuAction::LoadStore => "📂",
                MainMenuAction::LoadUnencryptedStore => "📄",
                MainMenuAction::EditStore => "✏️",
                MainMenuAction::SaveStore => "💾",
                MainMenuAction::SaveUnencryptedStore => "⚠️",
            };

            let icon_cell = Cell::from(Text::from(format!(" {} ", icon)));
            let name_cell = Cell::from(Text::from(name).style(
                if is_selected {
                    Style::default().bold()
                } else {
                    Style::default()
                }
            ));
            let desc_cell = Cell::from(Text::from(description).style(
                if is_selected {
                    Style::default()
                } else {
                    Style::default().fg(Color::Gray)
                }
            ));

            let row = Row::new(vec![icon_cell, name_cell, desc_cell]);
            if is_selected {
                rows.push(row.style(Style::default().bg(Color::Yellow).fg(Color::Black)));
            } else {
                rows.push(row);
            }

            // Add separator row (except after last item)
            if idx < actions.len() - 1 {
                let separator = Row::new(vec![
                    Cell::from(""),
                    Cell::from("─".repeat(40)),
                    Cell::from(""),
                ]).style(Style::default().fg(Color::DarkGray));
                rows.push(separator);
            }
        }

        let min_title_width = MainMenuAction::iter()
            .map(|a| a.to_string().0.len())
            .max()
            .unwrap_or(0);
        let min_desc_width = MainMenuAction::iter()
            .map(|a| a.to_string().1.len())
            .max()
            .unwrap_or(0);

        let table = Table::new(
            rows,
            [
                Constraint::Length(4),  // Icon column (fixed width)
                Constraint::Length(min_title_width as u16),
                Constraint::Length(min_desc_width as u16)
            ]
        )
            .column_spacing(2)
            .style(Style::default().fg(Color::White));

        let num_actions = MainMenuAction::iter().count();
        let table_height = (num_actions + (num_actions - 1)) as u16; // actions + separators
        let table_width = (4 + min_title_width + min_desc_width + 4) as u16; // columns + spacing

        // Center vertically within the area
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(table_height),
                Constraint::Fill(1),
            ])
            .split(area);

        // Center horizontally within the area
        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(table_width),
                Constraint::Fill(1),
            ])
            .split(vertical_layout[1]);

        frame.render_widget(table, horizontal_layout[1]);
    }
}
