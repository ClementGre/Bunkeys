pub mod state;
pub mod data;

use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::data::AppData;
use crate::app::state::AppState;
use crate::app::state::AppStateEvents;

pub struct App {
    pub(crate) state: AppState,
    pub(crate) data: AppData,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            state: AppState::default(),
            data: AppData::default(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        self.data.message = None;
        self.data.error = None;
        self.state = (&self.state).handle_key(&mut self.data, key);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Status/Messages
            ])
            .split(area);

        // Render header
        frame.render_widget(
            Paragraph::new("Shamir Store manager")
                .block(Block::default().borders(Borders::ALL).style(Color::Cyan)),
            chunks[0],
        );

        // Render content
        (&self.state).render(&self.data, frame, chunks[1]);

        // Render status bar
        let text = if let Some(error) = &self.data.error {
            Line::from(Span::styled(
                format!("ERROR: {}", error),
                Style::default().fg(Color::Red),
            ))
        } else if let Some(message) = &self.data.message {
            Line::from(Span::styled(
                message.as_str(),
                Style::default().fg(Color::Green),
            ))
        } else {
            Line::from("Ready")
        };
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Status")
                    .border_style(Color::Gray),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, chunks[2]);
    }
}
