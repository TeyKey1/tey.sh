use std::{collections::HashMap, rc::Rc};

use colored::*;
use xterm_js_rs::Terminal;

mod clear;
mod rm;
mod tey_sh;

use clear::Clear;
use rm::Rm;
use tey_sh::TeySh;

pub trait Command {
    /// Executes a command with the provided tokens
    ///
    /// The returned bool is true if the application should switch from cli mode to tui mode
    fn execute(&self, tokens: &Vec<&str>, terminal: &Terminal) -> bool;
}

pub struct CommandManager {
    terminal: Rc<Terminal>,
    /// Maps command name to command impl
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandManager {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        let mut manager = Self {
            terminal,
            commands: HashMap::new(),
        };

        manager.register_command(Cmd::TeySh, TeySh::new());
        manager.register_command(Cmd::Clear, Clear);
        manager.register_command(Cmd::Rm, Rm);

        manager
    }

    pub fn execute(&self, input_trimmed: &str) -> bool {
        let tokens: Vec<&str> = input_trimmed.split_whitespace().collect();

        let cmd = tokens.first().expect("Failed to find top level command in tokens. The execute function should only be called if input_trimmed contains at least one entry.");

        match Cmd::from(*cmd) {
            Cmd::Unknown => {
                self.terminal.write(&format!(
                    "{}: command not found. Type {} to see available commands\n",
                    cmd,
                    "tey.sh --help".green()
                ));

                false
            }
            _ => self.get_command(cmd).execute(&tokens, &self.terminal),
        }
    }

    fn get_command(&self, name: &str) -> &dyn Command {
        self.commands.get(name).expect("Failed to find command in registry. This is a bug in the program. Make sure to only call this function if the provided name exists as a command.").as_ref()
    }

    fn register_command<T>(&mut self, cmd: Cmd, handler: T)
    where
        T: 'static + Command,
    {
        self.commands
            .insert(<Cmd as Into<&str>>::into(cmd).to_owned(), Box::new(handler));
    }
}

pub enum Cmd {
    /// Unknown if no other command in the enum matched
    Unknown,
    TeySh,
    Clear,
    Rm,
}

impl From<&str> for Cmd {
    fn from(value: &str) -> Self {
        match value {
            "tey.sh" => Self::TeySh,
            "clear" => Self::Clear,
            "rm" => Self::Rm,
            _ => Self::Unknown,
        }
    }
}

impl Into<&str> for Cmd {
    fn into(self) -> &'static str {
        match self {
            Cmd::Unknown => panic!("Cannot convert an unknown command into a string"),
            Cmd::TeySh => "tey.sh",
            Cmd::Clear => "clear",
            Cmd::Rm => "rm",
        }
    }
}
