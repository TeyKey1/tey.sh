//! The cli mode of the app. Handles all text commands and emulates a basic shell

use std::rc::Rc;

use colored::*;
use crossterm::event::KeyCode;
use lazy_static::lazy_static;
use xterm_js_rs::Terminal;

use crate::app::AppEventHandler;

mod emulate;

use emulate::history::History;
use emulate::text_input::TextInput;

const MAX_HISTORY_SIZE: usize = 50;

lazy_static! {
    static ref CLI_PREFIX: String = format!(
        "{} {} {} ",
        "me@tey.sh".yellow(),
        "~".magenta(),
        "$".yellow()
    );
}

pub struct Cli {
    terminal: Rc<Terminal>,
    /// History of terminal commands/input
    cmd_history: History,
    /// Typed characters on current terminal row
    line_buffer: TextInput,
}

impl Cli {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        Self {
            terminal: terminal.clone(),
            cmd_history: History::new(MAX_HISTORY_SIZE),
            line_buffer: TextInput::new(terminal),
        }
    }

    pub fn init(&self) {
        self.terminal.writeln(&format!(
            "{}{}type {} to see all commands",
            "\x1B[0;0H", // Move cursor to 0,0 coordinates
            *CLI_PREFIX,
            "tey.sh --help".green(),
        ));
        self.terminal.write(CLI_PREFIX.as_str());
        self.terminal.write(&format!(
            "{} {} {} {} {} {} {} {}",
            "black".black(),
            "red".red(),
            "green".green(),
            "yellow".yellow(),
            "blue".blue(),
            "magenta".magenta(),
            "cyan".cyan(),
            "white".white()
        ));
        self.terminal.write(&format!(
            "{} {} {} {} {} {} {} {}",
            "black".bright_black(),
            "red".bright_red(),
            "green".bright_green(),
            "yellow".bright_yellow(),
            "blue".bright_blue(),
            "magenta".bright_magenta(),
            "cyan".bright_cyan(),
            "white".bright_white()
        ));
    }

    /// Execute a cli command
    pub fn execute_command(&self) {}
}

impl AppEventHandler for Cli {
    fn handle_event(&mut self, event: crossterm::event::Event) -> bool {
        match event {
            crossterm::event::Event::Key(event) => {
                match event.code {
                    KeyCode::Esc => return true,
                    KeyCode::Backspace => self.line_buffer.backspace(),
                    KeyCode::Delete => self.line_buffer.del(),
                    KeyCode::Enter => {
                        // Reset history cursor
                        self.cmd_history.reset();

                        self.line_buffer.end_line();

                        // strip leading and trailing whitespace

                        // send command and write a new line
                        self.execute_command();

                        // send command to history

                        self.line_buffer.start_line();
                    }
                    KeyCode::Left => self.line_buffer.move_cursor_left(),
                    KeyCode::Right => self.line_buffer.move_cursor_right(),
                    KeyCode::Up => {
                        // get previous input from history if any
                        self.cmd_history.get_next();
                    }
                    KeyCode::Down => {
                        self.cmd_history.get_prev();
                    }
                    KeyCode::Tab => {
                        // handle autocompletion
                    }
                    KeyCode::Char(char) => self.line_buffer.input_char(char),
                    _ => (),
                }

                false
            }
            _ => false,
        }
    }
}
