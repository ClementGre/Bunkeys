#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextInput {
    text: String,
    cursor_pos: usize,
}

impl TextInput {
    pub fn new(text: String) -> Self {
        Self {
            cursor_pos: text.len(),
            text,
        }
    }
    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn cursor_char_pos(&self) -> usize {
        self.text[..self.cursor_pos].chars().count()
    }

    pub fn with_insert_char(&self, c: char) -> Self {
        let mut ti = self.clone();
        ti.text.insert(self.cursor_pos, c);
        ti.cursor_pos += c.len_utf8();
        ti
    }

    pub fn with_delete_char(&self) -> Self {
        let mut ti = self.clone();
        if ti.cursor_pos > 0 {
            // Find the previous char boundary
            let prev_pos = ti.text[..ti.cursor_pos]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
            ti.text.remove(prev_pos);
            ti.cursor_pos = prev_pos;
        }
        ti
    }

    pub fn with_move_left(&self) -> Self {
        let mut ti = self.clone();
        if ti.cursor_pos > 0 {
            // Move to previous char boundary
            ti.cursor_pos = ti.text[..ti.cursor_pos]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
        ti
    }

    pub fn with_move_right(&self) -> Self {
        let mut ti = self.clone();
        if ti.cursor_pos < ti.text.len() {
            // Move to next char boundary
            if let Some((_, c)) = ti.text[ti.cursor_pos..].char_indices().next() {
                ti.cursor_pos += c.len_utf8();
            }
        }
        ti
    }
}
