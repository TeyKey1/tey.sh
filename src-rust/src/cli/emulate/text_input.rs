use std::rc::Rc;

use colored::*;
use lazy_static::lazy_static;
use strip_ansi_escapes::strip_str;
use xterm_js_rs::Terminal;

lazy_static! {
    static ref LINE_PREFIX: String = format!(
        "{} {} {} ",
        "me@tey.sh".yellow(),
        "~".magenta(),
        "$".yellow()
    );
    static ref LINE_PREFIX_LEN: u16 = {
        let stripped = strip_str(LINE_PREFIX.as_str());
        stripped
            .chars()
            .count()
            .try_into()
            .expect("line prefix should not be longer than u16")
    };
}

/// Handle text input and horizontal cursor movement
pub struct TextInput {
    terminal: Rc<Terminal>,
    buffer: String,
    /// X-Position of the cursor in the current terminal line
    cursor_pos: u16,
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
        if self.cursor_pos <= *LINE_PREFIX_LEN {
            return;
        }

        self.cursor_pos -= 1;

        self.terminal
            .write(&ansi_escapes::CursorMove::X(-1).to_string());
    }

    pub fn move_cursor_right(&mut self) {
        if self.get_cursor_buffer_pos() >= self.buffer.chars().count() {
            return;
        }

        self.cursor_pos += 1;

        self.terminal
            .write(&ansi_escapes::CursorMove::X(1).to_string());
    }

    /// Handle a char keystroke
    pub fn input_char(&mut self, char: char) {
        self.buffer.insert(self.get_cursor_buffer_pos(), char); // TODO: Naive approach one char = one byte

        self.cursor_pos += 1;

        self.write_line(Some(self.cursor_pos));
    }

    /// Handle backspace keystroke
    pub fn backspace(&mut self) {
        if self.cursor_pos <= *LINE_PREFIX_LEN {
            return;
        }

        self.buffer.remove(self.get_cursor_buffer_pos() - 1); // TODO: Naive approach one char = one byte

        self.cursor_pos -= 1;

        self.write_line(Some(self.cursor_pos));
    }

    /// Handle DEL keystroke
    pub fn del(&mut self) {
        let buffer_cursor_pos = self.get_cursor_buffer_pos();

        if buffer_cursor_pos >= self.buffer.chars().count() {
            return;
        }

        self.buffer.remove(buffer_cursor_pos); // TODO: Naive approach one char = one byte

        self.write_line(Some(self.cursor_pos));
    }

    /// End the current line and move the cursor to the next one
    pub fn end_line(&mut self) {
        self.buffer.clear();

        self.terminal.write(&format!(
            "{}\n{}",
            ansi_escapes::CursorHide,
            ansi_escapes::CursorTo::AbsoluteX(0)
        ));
    }

    /// Start a new line for user input
    pub fn start_line(&mut self) {
        self.terminal.write(LINE_PREFIX.as_str());

        self.cursor_pos = *LINE_PREFIX_LEN;

        self.write_line(None);
    }

    /// Set the current line buffer to the provided content and replace the currently active terminal row with it
    pub fn set_buffer(&mut self, content: &str) {
        self.buffer.replace_range(.., content);

        self.cursor_pos = *LINE_PREFIX_LEN + content.chars().count() as u16;

        self.write_line(None);
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    /// Overwrite/Write a new line including the [LINE_PREFIX] and the current content of the buffer
    fn write_line(&mut self, cursor_x_pos: Option<u16>) {
        self.terminal.write(&ansi_escapes::CursorHide.to_string()); // Hide cursor during writing to avoid it flashing at wrong locations

        self.terminal.write(&format!(
            "{}{}{}{}",
            ansi_escapes::EraseLine,
            ansi_escapes::CursorTo::AbsoluteX(0),
            *LINE_PREFIX,
            self.buffer
        ));

        if let Some(cursor_x_pos) = cursor_x_pos {
            self.terminal
                .write(&ansi_escapes::CursorTo::AbsoluteX(cursor_x_pos).to_string());
        }

        self.terminal.write(&ansi_escapes::CursorShow.to_string());
    }

    /// Returns the current buffer char index the cursor points at
    fn get_cursor_buffer_pos(&self) -> usize {
        (self.cursor_pos - *LINE_PREFIX_LEN) as usize
    }
}
