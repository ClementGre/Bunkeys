use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::prelude::{Color, Rect, Style, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem};
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
    pub fn to_string(&self) -> &'static str {
        match self {
            MainMenuAction::InitStore => "Init Store - Generate new 256-bit key and create empty store",
            MainMenuAction::LoadStore => "Load Store - Load existing store from file",
            MainMenuAction::LoadUnencryptedStore => "Load Store Data From Unencrypted File - Load current store data from unencrypted file",
            MainMenuAction::EditStore => "Edit Store - View and modify store contents",
            MainMenuAction::SaveStore => "Save Store - Save store to file",
            MainMenuAction::SaveUnencryptedStore => "Save Unencrypted Store - Save store to file without encryption (NOT RECOMMENDED!)",
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
                        LoadStoreState::new_path(true, None).into()
                    },
                    MainMenuAction::LoadUnencryptedStore => {
                        LoadStoreState::new_path(false, None).into()
                    }
                    MainMenuAction::EditStore => {
                        EditStoreState::new().into()
                    }
                    MainMenuAction::SaveStore => {
                        let path = data.store_path.clone().map(|p| {
                            let mut path = p.to_string_lossy().to_string();
                            if path.ends_with(".yaml") {
                                path = path[..path.len()-5].to_string();
                                path.push_str(".enc");
                            }
                            path
                        });
                        SaveStoreState::new(true, path).into()
                    }
                    MainMenuAction::SaveUnencryptedStore => {
                        let path = data.store_path.clone().map(|p| {
                            let mut path = p.to_string_lossy().to_string();
                            if path.ends_with(".enc") {
                                path = path[..path.len()-4].to_string();
                                path.push_str(".yaml");
                            }
                            path
                        });
                        SaveStoreState::new(false, path).into()
                    }
                }
            }
            _ => self.clone().into()
        }
    }

    fn render(&self, data: &AppData, frame: &mut Frame, area: Rect) {
        let items = MainMenuAction::iter().map(|a| {
            let mut item = ListItem::new(a.to_string());
            if a == self.selected_action {
                item = item.clone().bg(Color::LightYellow).fg(Color::Black);
            }
            item
        }).collect::<Vec<_>>();

        let mut title = "Main Menu".to_string();
        if data.store_key.is_some() {
            title.push_str(" [Store Loaded]");
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_bottom("[q: Quit] [Enter: Select]"),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(list, area);
    }
}
