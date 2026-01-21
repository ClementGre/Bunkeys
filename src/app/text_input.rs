#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextInput {
    pub text: String,
    pub cursor_pos: usize,
}

impl TextInput {
    pub fn new(text: String) -> Self {
        Self {
            cursor_pos: text.len(),
            text,
        }
    }
    pub fn with_insert_char(&self, c: char) -> Self {
        let mut ti = self.clone();
        ti.text.insert(self.cursor_pos, c);
        ti.cursor_pos += 1;
        ti
    }
    pub fn with_delete_char(&self) -> Self {
        let mut ti = self.clone();
        if ti.cursor_pos > 0 {
            ti.text.remove(ti.cursor_pos - 1);
            ti.cursor_pos -= 1;
        }
        ti
    }
    pub fn with_move_left(&self) -> Self {
        let mut ti = self.clone();
        if ti.cursor_pos > 0 {
            ti.cursor_pos -= 1;
        }
        ti
    }
    pub fn with_move_right(&self) -> Self {
        let mut ti = self.clone();
        if ti.cursor_pos < ti.text.len() {
            ti.cursor_pos += 1;
        }
        ti
    }
}