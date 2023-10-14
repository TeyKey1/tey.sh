use std::{cmp::min, rc::Rc};

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

        let (cursor_pos_col, cursor_pos_row) = self.get_cursor_pos();

        self.cursor_pos -= 1;

        if cursor_pos_col == 0 && cursor_pos_row != 0 {
            // multiline move one row up
            self.terminal.write(&format!(
                "{}{}",
                ansi_escapes::CursorMove::Y(-1),
                ansi_escapes::CursorTo::AbsoluteX(self.terminal.get_cols() as u16)
            ))
        } else {
            self.terminal
                .write(&ansi_escapes::CursorMove::X(-1).to_string());
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.get_cursor_buffer_pos() >= self.buffer.chars().count() {
            return;
        }

        let (_, cursor_pos_row_old) = self.get_cursor_pos();

        self.cursor_pos += 1;

        let (_, cursor_pos_row) = self.get_cursor_pos();

        if cursor_pos_row != cursor_pos_row_old {
            // Move cursor to beginning of next multiline row
            self.terminal.write(&format!(
                "{}{}",
                ansi_escapes::CursorMove::Y(1),
                ansi_escapes::CursorTo::AbsoluteX(0)
            ));
        } else {
            self.terminal
                .write(&ansi_escapes::CursorMove::X(1).to_string());
        }
    }

    /// Move cursor to start of the user input (CTRL + a)
    pub fn move_cursor_start(&mut self) {
        self.cursor_pos = *LINE_PREFIX_LEN;

        self.write_line(Some(self.get_cursor_pos()));
    }

    /// Move cursor to the end of user input (CTRL + e)
    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = *LINE_PREFIX_LEN + self.buffer.chars().count() as u16;

        self.write_line(Some(self.get_cursor_pos()));
    }

    /// Handle a char keystroke
    ///
    /// # utf-8 encoded length
    /// Currently only chars with a max length of one byte (utf-8 encoded) are supported. If this function encounters chars longer than one byte they are simply ignored.
    pub fn input_char(&mut self, char: char) {
        if char.len_utf8() > 1 {
            // Safeguard due to naive implementation
            return;
        }

        self.buffer.insert(self.get_cursor_buffer_pos(), char); // TODO: Naive approach one char = one byte

        self.cursor_pos += 1;

        self.write_line(Some(self.get_cursor_pos()));
    }

    /// Handle backspace keystroke
    pub fn backspace(&mut self) {
        if self.cursor_pos <= *LINE_PREFIX_LEN {
            return;
        }

        self.buffer.remove(self.get_cursor_buffer_pos() - 1); // TODO: Naive approach one char = one byte

        self.cursor_pos -= 1;

        self.write_line(Some(self.get_cursor_pos()));
    }

    /// Handle DEL keystroke
    pub fn del(&mut self) {
        let buffer_cursor_pos = self.get_cursor_buffer_pos();

        if buffer_cursor_pos >= self.buffer.chars().count() {
            return;
        }

        self.buffer.remove(buffer_cursor_pos); // TODO: Naive approach one char = one byte

        self.write_line(Some(self.get_cursor_pos()));
    }

    /// End the current line and move the cursor to the next one
    pub fn end_line(&mut self) {
        // Get previous cursor pos to put the newline at the correct row
        let (_, cursor_row_pos) = self.get_cursor_pos();
        let buffer_line_count = self.get_line_count();

        let move_y = buffer_line_count as u16 - cursor_row_pos;

        // Reset data
        self.buffer.clear();
        self.cursor_pos = 0;

        self.terminal.write(&format!(
            "{}{}\n{}",
            ansi_escapes::CursorHide,
            ansi_escapes::CursorMove::Y(move_y as i16),
            ansi_escapes::CursorTo::AbsoluteX(0),
        ));
    }

    /// Start a new line for user input
    pub fn start_line(&mut self) {
        self.cursor_pos = *LINE_PREFIX_LEN;

        self.terminal
            .write(&ansi_escapes::CursorSavePosition.to_string()); // Save cursor position for exact line rendering / deletion

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
    fn write_line(&mut self, cursor_pos: Option<(u16, u16)>) {
        // Hide cursor during writing to avoid it flashing at wrong locations
        self.terminal.write(&ansi_escapes::CursorHide.to_string());

        // Move cursor to beginning of current prompt and erase everything below before rerendering input
        self.terminal.write(&format!(
            "{}{}",
            ansi_escapes::CursorRestorePosition,
            ansi_escapes::EraseDown
        ));

        // Render lines
        let buffer_line_count = self.get_line_count();
        for line in 0..=buffer_line_count {
            let line_prefix = if line == 0 { LINE_PREFIX.as_str() } else { "" };
            let line_ending = if buffer_line_count > line { "\n" } else { "" };

            self.terminal.write(&format!(
                "{}{}{}{}",
                ansi_escapes::CursorTo::AbsoluteX(0),
                line_prefix,
                self.get_line_content(line as usize),
                line_ending
            ));
        }

        // Move cursor to requested position
        if let Some((cursor_col_pos, cursor_row_pos)) = cursor_pos {
            self.terminal.write(&format!(
                "{}{}{}",
                ansi_escapes::CursorRestorePosition,
                ansi_escapes::CursorMove::Y(cursor_row_pos as i16),
                ansi_escapes::CursorTo::AbsoluteX(cursor_col_pos)
            ));
        }

        self.terminal.write(&ansi_escapes::CursorShow.to_string());
    }

    /// Returns the current buffer char index the cursor points at
    fn get_cursor_buffer_pos(&self) -> usize {
        (self.cursor_pos - *LINE_PREFIX_LEN) as usize
    }

    /// Returns the cursor pos col, row of the current input
    fn get_cursor_pos(&self) -> (u16, u16) {
        let terminal_cols = self.terminal.get_cols() as u16;

        (
            self.cursor_pos % terminal_cols,
            self.cursor_pos / terminal_cols,
        )
    }

    /// Returns how many lines (rows) the current buffer can fill
    fn get_line_count(&self) -> u32 {
        (self.buffer.chars().count() as u32 + *LINE_PREFIX_LEN as u32) / self.terminal.get_cols()
    }

    fn get_line_content(&self, line: usize) -> &str {
        let mut start_offset: usize = self.terminal.get_cols() as usize - *LINE_PREFIX_LEN as usize;
        let end: usize;

        if line == 0 {
            end = min(start_offset, self.buffer.chars().count()); // TODO: case when line prefix len is greater than term cols
            start_offset = 0;
        } else {
            start_offset += (line - 1) * self.terminal.get_cols() as usize;
            end = min(
                start_offset + self.terminal.get_cols() as usize,
                self.buffer.chars().count(),
            );
        }

        &self.buffer[start_offset..end] // TODO: Naive approach one char = one byte
    }
}
