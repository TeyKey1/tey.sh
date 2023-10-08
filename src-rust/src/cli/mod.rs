//! The cli mode of the app. Handles all text commands and emulates a basic shell

use std::rc::Rc;

use colored::*;
use crossterm::event::KeyCode;
use lazy_static::lazy_static;
use xterm_js_rs::Terminal;

use crate::app::AppEventHandler;

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
}

impl Cli {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        Self { terminal }
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
}

impl AppEventHandler for Cli {
    fn handle_event(&mut self, event: crossterm::event::Event) -> bool {
        match event {
            crossterm::event::Event::Key(event) => {
                if event.code == KeyCode::PageUp {
                    true
                } else {
                    self.terminal.writeln("someline");
                    false
                }
            }
            _ => false,
        }
    }
}
