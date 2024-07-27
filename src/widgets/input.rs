/// Input represents an input widget that only cares about getting inputs but not how it looks.
#[derive(Debug, Default)]
pub struct Input {
    /// The current input
    input: String,
    /// The current position of the cursor
    cursor_index: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_index: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn enter_character(&mut self, character: char) {
        self.input.insert(self.cursor_index, character);
        self.move_cursor_right();
    }

    pub fn delete_character(&mut self) {
        if self.input.chars().count() > 0 {
            // the cursor index is always one ahead of the input
            let current_index = self.cursor_index;
            let left_to_current_index = current_index - 1;
            let before_delete_chars = self.input.chars().take(left_to_current_index);
            let after_delete_chars = self.input.chars().skip(current_index);
            self.input = before_delete_chars.chain(after_delete_chars).collect();
            self.move_cursor_left();
        }
    }

    /// Gets the entire input and resets the states of the input widget.
    pub fn get_string(&self) -> String {
        self.input.clone()
    }

    pub fn get_cursor_index_u16(&self) -> u16 {
        match u16::try_from(self.cursor_index) {
            Ok(v) => v,
            Err(_) => 0,
        }
    }

    /// Reset the states of the input widget
    pub fn reset(&mut self) {
        self.input.clear();
        self.cursor_index = 0;
    }

    /// Moves the cursors by the right.
    fn move_cursor_right(&mut self) {
        let new_cursor_index = self.cursor_index.saturating_add(1);
        self.cursor_index = new_cursor_index.clamp(0, self.input.chars().count());
    }

    /// Moves the cursor by the left.
    fn move_cursor_left(&mut self) {
        let new_cursor_index = self.cursor_index.saturating_sub(1);
        self.cursor_index = new_cursor_index.clamp(0, self.input.chars().count());
    }
}
