//! The cli mode of the app. Handles all text commands and emulates a basic shell

use std::rc::Rc;

use colored::*;
use crossterm::event::{KeyCode, KeyModifiers};
use lazy_static::lazy_static;
use xterm_js_rs::Terminal;

use crate::app::AppEventHandler;

mod commands;
mod emulate;
mod render_md;

use commands::CommandManager;
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
    /// Commands available
    commands: CommandManager,
}

impl Cli {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        Self {
            terminal: terminal.clone(),
            cmd_history: History::new(MAX_HISTORY_SIZE),
            line_buffer: TextInput::new(terminal.clone()),
            commands: CommandManager::new(terminal),
        }
    }

    pub fn init(&mut self) {
        render_md::render_md(include_str!("../../../content/projects.md"));

        self.terminal.writeln(&format!(
            "{}{}type {} to see all commands",
            "\x1B[0;0H", // Move cursor to 0,0 coordinates
            *CLI_PREFIX,
            "tey.sh --help".green(),
        ));
        self.terminal.writeln("\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1mtey.sh\u{1b}[0m \u{1b}[1m--version\u{1b}[0m");
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

        self.terminal
            .write(&format!("{}{}", ansi_escapes::CursorNextLine, *CLI_PREFIX));

        self.line_buffer.start_line();
    }
}

impl AppEventHandler for Cli {
    fn handle_event(&mut self, event: crossterm::event::Event) -> bool {
        match event {
            crossterm::event::Event::Key(event) => {
                let mut switch_context = false;

                match event.code {
                    KeyCode::Esc => return true,
                    KeyCode::Backspace => self.line_buffer.backspace(),
                    KeyCode::Delete => self.line_buffer.del(),
                    KeyCode::Enter => {
                        // Reset history
                        self.cmd_history.reset();

                        let input = self.line_buffer.get_buffer().to_owned();
                        let trimmed = input.trim();

                        self.line_buffer.end_line();

                        if !trimmed.is_empty() {
                            switch_context = self.commands.execute(trimmed);

                            // send command to history
                            self.cmd_history.push(trimmed);
                        }

                        self.line_buffer.start_line();
                    }
                    KeyCode::Left => self.line_buffer.move_cursor_left(),
                    KeyCode::Right => self.line_buffer.move_cursor_right(),
                    KeyCode::Up => {
                        // get previous input from history if any
                        self.cmd_history
                            .cache_user_input(self.line_buffer.get_buffer());

                        if let Some(history_entry) = self.cmd_history.get_next() {
                            self.line_buffer.set_buffer(history_entry);
                        }
                    }
                    KeyCode::Down => {
                        if let Some(history_entry) = self.cmd_history.get_prev() {
                            self.line_buffer.set_buffer(history_entry);
                        }
                    }
                    KeyCode::Tab => {
                        // handle autocompletion
                    }
                    KeyCode::Char(char) => {
                        if event.modifiers.is_empty() || event.modifiers == KeyModifiers::SHIFT {
                            self.line_buffer.input_char(char);
                        } else if event.modifiers == KeyModifiers::CONTROL {
                            match char {
                                'a' => self.line_buffer.move_cursor_start(),
                                'e' => self.line_buffer.move_cursor_end(),
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }

                switch_context
            }

            _ => false,
        }
    }
}
