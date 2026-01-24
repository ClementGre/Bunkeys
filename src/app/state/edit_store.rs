use crate::app::data::{AppData, Entry, Section};
use crate::app::state::main_menu::{MainMenuAction, MainMenuState};
use crate::app::state::AppStateEvents;
use crate::app::text_input::TextInput;
use crate::app::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::prelude::Position;
use ratatui::style::{Color, Style};
use ratatui::widgets::{List, ListItem};
use ratatui::Frame;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EditStoreState {
    /// even values -> entry key selected, odd values -> entry value selected
    flattened_index: usize,
    is_editing: bool,
    was_created: bool,
    input: TextInput,
}

enum EditStoreSelection {
    Section(usize),
    EntryKey(usize, usize),
    EntryValue(usize, usize),
    AddEntry(usize),
    AddSection,
}
impl EditStoreSelection {
    pub fn get_text(&self, data: &AppData) -> String {
        match self {
            EditStoreSelection::Section(si) => data.sections[*si].name.clone(),
            EditStoreSelection::EntryKey(si, ei) => data.sections[*si].entries[*ei].key.clone(),
            EditStoreSelection::EntryValue(si, ei) => data.sections[*si].entries[*ei].value.clone(),
            _ => "".to_string(),
        }
    }
}

impl EditStoreState {
    pub fn flattened_len(&self, data: &AppData) -> usize {
        data.sections
            .iter()
            .map(|s| 2 + 2 * s.entries.len() + 2)
            .sum::<usize>()
            + 2
    }
    fn get_selected_item(&self, data: &AppData) -> EditStoreSelection {
        let mut i = 0;
        for (si, section) in data.sections.iter().enumerate() {
            // Section header
            if self.flattened_index == i || self.flattened_index == i + 1 {
                return EditStoreSelection::Section(si);
            }
            i += 2;
            // Entries
            for (ei, _entry) in section.entries.iter().enumerate() {
                if self.flattened_index == i {
                    return EditStoreSelection::EntryKey(si, ei);
                } else if self.flattened_index == i + 1 {
                    return EditStoreSelection::EntryValue(si, ei);
                }
                i += 2;
            }
            // "Add Entry" option
            if self.flattened_index == i || self.flattened_index == i + 1 {
                return EditStoreSelection::AddEntry(si);
            }
            i += 2;
        }
        // "Add Section" option
        EditStoreSelection::AddSection
    }
}

impl AppStateEvents for EditStoreState {
    fn get_title(&self, _data: &AppData) -> String {
        "Edit Store".to_string()
    }

    fn get_footer(&self, data: &AppData) -> &'static str {
        if self.is_editing {
            return "[Esc: Cancel Edit] [⏎: Save Edit]";
        }
        match self.get_selected_item(data) {
            EditStoreSelection::Section(_si) => {
                "[Esc: Save And Exit] [↑↓←→: Navigate] [⏎: Rename] [⌫: Delete Section]"
            }
            EditStoreSelection::EntryKey(_si, _ei) => {
                "[Esc: Save And Exit] [↑↓←→: Navigate] [⏎: Rename] [⌫: Delete Entry]"
            }
            EditStoreSelection::EntryValue(si, ei) => {
                if data.sections[si].entries[ei].value.is_empty() {
                    "[Esc: Save And Exit] [↑↓←→: Navigate] [⏎: Edit] [⌫: Empty Value]"
                } else {
                    "[Esc: Save And Exit] [↑↓←→: Navigate] [⏎: Edit] [⌫: Delete Entry]"
                }
            }
            EditStoreSelection::AddEntry(_si) => {
                "[Esc: Save and Exit] [↑↓←→: Navigate] [⏎: Create Entry]"
            }
            EditStoreSelection::AddSection => {
                "[Esc: Save and Exit] [↑↓←→: Navigate] [⏎: Create Section]"
            }
        }
    }

    fn handle_key(&self, data: &mut AppData, key: KeyEvent) -> AppState {
        let mut new_state = self.clone();
        let max_index = new_state.flattened_len(data);
        let selected = self.get_selected_item(data);

        match key.code {
            KeyCode::Esc if { !self.is_editing } => {
                return MainMenuState::new(MainMenuAction::EditStore).into();
            }
            KeyCode::Up if { !self.is_editing } => {
                if new_state.flattened_index <= 1 {
                    new_state.flattened_index = max_index - 2 + new_state.flattened_index;
                } else {
                    new_state.flattened_index -= 2;
                }
            }
            KeyCode::Down if { !self.is_editing } => {
                new_state.flattened_index = (new_state.flattened_index + 2) % max_index;
            }
            KeyCode::Left if { !self.is_editing } => {
                if new_state.flattened_index % 2 == 1 {
                    new_state.flattened_index -= 1;
                }
            }
            KeyCode::Right if { !self.is_editing } => {
                if new_state.flattened_index % 2 == 0 {
                    new_state.flattened_index += 1;
                }
            }
            KeyCode::Backspace if { !self.is_editing } => match selected {
                EditStoreSelection::Section(si) => {
                    data.message = Some(format!("Deleted section '{}'.", data.sections[si].name));
                    data.sections.remove(si);
                    if self.flattened_index >= 2 {
                        new_state.flattened_index -= 2;
                    }
                }
                EditStoreSelection::EntryKey(si, ei) => {
                    data.message = Some(format!(
                        "Deleted entry '{}'.",
                        data.sections[si].entries[ei].key
                    ));
                    data.sections[si].entries.remove(ei);
                    if self.flattened_index >= 2 {
                        new_state.flattened_index -= 2;
                    }
                }
                EditStoreSelection::EntryValue(si, ei) => {
                    if data.sections[si].entries[ei].value.is_empty() {
                        data.message = Some(format!(
                            "Deleted entry '{}'.",
                            data.sections[si].entries[ei].key
                        ));
                        data.sections[si].entries.remove(ei);
                        if self.flattened_index >= 2 {
                            new_state.flattened_index -= 2;
                        }
                    } else {
                        data.message = Some(format!(
                            "Emptied entry '{}'.",
                            data.sections[si].entries[ei].key
                        ));
                        data.sections[si].entries[ei].value = String::new();
                    }
                }
                _ => {}
            },
            KeyCode::Enter if { !self.is_editing } => {
                match selected {
                    EditStoreSelection::Section(si) => {
                        data.message =
                            Some(format!("Renaming section '{}'.", data.sections[si].name));
                        new_state.was_created = false;
                    }
                    EditStoreSelection::EntryKey(si, ei) => {
                        data.message = Some(format!(
                            "Renaming entry '{}'.",
                            data.sections[si].entries[ei].key
                        ));
                        new_state.was_created = false;
                    }
                    EditStoreSelection::EntryValue(si, ei) => {
                        data.message = Some(format!(
                            "Editing entry '{}'.",
                            data.sections[si].entries[ei].key
                        ));
                        new_state.was_created = false;
                    }
                    EditStoreSelection::AddSection => {
                        data.sections.push(Section::default());
                        new_state.flattened_index =
                            self.flattened_len(data) - 6 + (self.flattened_index % 2);
                        new_state.was_created = true;
                        data.message = Some("Section created. Please type in its name".to_string());
                    }
                    EditStoreSelection::AddEntry(si) => {
                        data.sections[si].entries.push(Entry::default());
                        new_state.flattened_index =
                            new_state.flattened_index - (new_state.flattened_index % 2);
                        new_state.was_created = true;
                        data.message = Some("Entry created. Please type in its name".to_string());
                    }
                }
                new_state.is_editing = true;
                new_state.input = TextInput::new(selected.get_text(data));
            }

            KeyCode::Char(c) if { self.is_editing } => {
                new_state.input = self.input.with_insert_char(c);
            }
            KeyCode::Left if { self.is_editing } => {
                new_state.input = self.input.with_move_left();
            }
            KeyCode::Right if { self.is_editing } => {
                new_state.input = self.input.with_move_right();
            }
            KeyCode::Backspace if { self.is_editing } => {
                new_state.input = self.input.with_delete_char();
            }
            KeyCode::Enter if { self.is_editing } => {
                match selected {
                    EditStoreSelection::Section(si) => {
                        if data
                            .sections
                            .iter()
                            .enumerate()
                            .any(|(sj, s)| sj != si && s.name == *self.input.get_text())
                        {
                            data.error = Some(format!(
                                "Section with name '{}' already exists.",
                                self.input.get_text()
                            ));
                            return self.clone().into();
                        }
                        data.sections[si].name = self.input.get_text().clone();
                        data.message = Some(format!(
                            "Section '{}' successfully renamed.",
                            self.input.get_text()
                        ));
                    }
                    EditStoreSelection::EntryKey(si, ei) => {
                        if data.sections[si]
                            .entries
                            .iter()
                            .enumerate()
                            .any(|(ej, e)| ej != ei && e.key == *self.input.get_text())
                        {
                            data.error = Some(format!(
                                "Entry with name '{}' already exists.",
                                self.input.get_text()
                            ));
                            return self.clone().into();
                        }
                        data.sections[si].entries[ei].key = self.input.get_text().clone();
                        data.message = Some(format!(
                            "Entry '{}' successfully renamed.",
                            self.input.get_text()
                        ));
                    }
                    EditStoreSelection::EntryValue(si, ei) => {
                        data.sections[si].entries[ei].value = self.input.get_text().clone();
                        data.message = Some(format!(
                            "Entry '{}' successfully updated.",
                            data.sections[si].entries[ei].key
                        ));
                    }
                    _ => {}
                }
                new_state.is_editing = false;
            }
            KeyCode::Esc if { self.is_editing } => {
                new_state.is_editing = false;
                if self.was_created {
                    // Delete entry/section if it was created
                    return new_state
                        .handle_key(
                            data,
                            KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty()),
                        )
                        .into();
                }
            }
            _ => {}
        };
        new_state.into()
    }

    fn render(&self, data: &AppData, frame: &mut Frame, area: Rect) {
        let mut items = Vec::new();
        let selected = self.get_selected_item(data);
        let mut current_line = 0u16;
        for (si, section) in data.sections.iter().enumerate() {
            // Section header
            items.push(match selected {
                EditStoreSelection::Section(sj) if sj == si && !self.is_editing => {
                    ListItem::new(format!("📁 {}:", section.name))
                        .style(Style::default().bg(Color::DarkGray))
                }
                EditStoreSelection::Section(sj) if sj == si && self.is_editing => {
                    let prefix_len = "📁 ".chars().count() as u16;
                    let cursor_offset = self.input.cursor_char_pos() as u16 + 1;
                    frame.set_cursor_position(Position::new(
                        area.x + prefix_len + cursor_offset,
                        area.y + current_line,
                    ));

                    ListItem::new(format!("📁 {}:", self.input.get_text()))
                        .style(Style::default().bg(Color::DarkGray))
                }
                _ => ListItem::new(format!("📁 {}:", section.name)),
            });
            current_line += 1;

            // Entries
            for (ei, entry) in section.entries.iter().enumerate() {
                let mut is_selected = true;
                let mut item = match selected {
                    EditStoreSelection::EntryKey(sj, ej)
                        if { sj == si && ej == ei && !self.is_editing } =>
                    {
                        ListItem::new(format!("  📄 {}: {}", entry.key, entry.value))
                    }
                    EditStoreSelection::EntryKey(sj, ej)
                        if { sj == si && ej == ei && self.is_editing } =>
                    {
                        let prefix_len = "  📄 ".chars().count() as u16;
                        let cursor_offset = self.input.cursor_char_pos() as u16 + 1;
                        frame.set_cursor_position(Position::new(
                            area.x + prefix_len + cursor_offset,
                            area.y + current_line,
                        ));
                        ListItem::new(format!("  📄 {}: {}", self.input.get_text(), entry.value))
                    }
                    EditStoreSelection::EntryValue(sj, ej)
                        if { sj == si && ej == ei && !self.is_editing } =>
                    {
                        ListItem::new(format!("  📄 {}: {}", entry.key, entry.value))
                    }
                    EditStoreSelection::EntryValue(sj, ej)
                        if { sj == si && ej == ei && self.is_editing } =>
                    {
                        let prefix_len = format!("  📄 {}: ", entry.key).chars().count() as u16;
                        let cursor_offset = self.input.cursor_char_pos() as u16 + 1;
                        frame.set_cursor_position(Position::new(
                            area.x + prefix_len + cursor_offset,
                            area.y + current_line,
                        ));
                        ListItem::new(format!("  📄 {}: {}", entry.key, self.input.get_text()))
                    }
                    _ => {
                        is_selected = false;
                        ListItem::new(format!("  📄 {}: {}", entry.key, entry.value))
                    }
                };
                if is_selected {
                    item = item.style(Style::default().bg(Color::DarkGray))
                }
                items.push(item);
                current_line += 1;
            }

            // "Add Entry" option
            let style = match selected {
                EditStoreSelection::AddEntry(sj) if sj == si => {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                }
                _ => Style::default().fg(Color::Gray),
            };
            items.push(ListItem::new("  ➕ Add Entry").style(style));
            current_line += 1;
        }

        // "Add Section" option
        let style = match selected {
            EditStoreSelection::AddSection => Style::default().bg(Color::DarkGray).fg(Color::White),
            _ => Style::default().fg(Color::Gray),
        };
        items.push(ListItem::new("➕ Add Section").style(style));

        frame.render_widget(List::new(items), area);
    }
}
