use std::rc::Rc;

use xterm_js_rs::Terminal;

/// Handle text input and horizontal cursor movement
pub struct TextInput {
    terminal: Rc<Terminal>,
    buffer: String,
    cursor_pos: usize,
}

impl TextInput {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        Self {
            terminal,
            buffer: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        todo!()
    }

    pub fn move_cursor_right(&mut self) {
        todo!()
    }

    /// Handle a char keystroke
    pub fn input_char(&mut self, char: char) {
        todo!()
    }

    /// Handle backspace keystroke
    pub fn backspace(&mut self) {
        todo!()
    }

    /// Handle DEL keystroke
    pub fn del(&mut self) {
        todo!()
    }

    /// End the current line and move the cursor to the next one
    pub fn end_line(&mut self) {
        todo!()
    }

    /// Start a new line for user input
    pub fn start_line(&mut self) {
        todo!()
    }

    /// Set the current line buffer to the provided content and replace the currently active terminal row with it
    pub fn set_buffer(&mut self, content: &str) {
        self.buffer.replace_range(.., content);

        self.terminal.write(&self.buffer);
        todo!()
    }
}
