use crossterm::event::KeyEvent;
use enum_dispatch::enum_dispatch;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::data::AppData;
use crate::app::state::edit_store::EditStoreState;
use crate::app::state::init_store::InitStoreState;
use crate::app::state::load_store::LoadStoreState;
use crate::app::state::main_menu::MainMenuState;
use crate::app::state::save_store::SaveStoreState;

pub mod init_store;
pub mod load_store;
pub mod edit_store;
pub mod main_menu;
pub mod save_store;


#[enum_dispatch(AppState)]
pub trait AppStateEvents {
    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState;
    fn render(&self, data: &AppData, frame: &mut Frame, area: Rect);
}

#[enum_dispatch]
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    MainMenu(MainMenuState),
    InitStore(InitStoreState),
    LoadStore(LoadStoreState),
    EditStore(EditStoreState),
    SaveStore(SaveStoreState),
}

impl Default for AppState {
    fn default() -> Self {
        MainMenuState::default().into()
    }
}
