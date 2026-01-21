use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

mod app;
mod bip39;
mod encrypt;
mod polynom;
mod shamir_secret_sharing;
mod store;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = app::App::new();

    loop {
        terminal.draw(|frame| app.render(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // Global quit
                if matches!(key.code, KeyCode::Char('q')) && matches!(app.state, app::state::AppState::MainMenu(_)) {
                    break Ok(());
                }
                app.handle_key(key);
            }
        }
    }
}
