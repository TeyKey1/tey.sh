use std::rc::Rc;

use xterm_js_rs::Terminal;

mod clear;
mod tey_sh;

use clear::Clear;

pub trait Command {
    /// Whether or not the supplied command matches the respective command implementation
    fn matches(&self, command: &str) -> bool;

    fn execute(&self);
}

/// Registry for all available commands. Handles command matching, execution and autocompletion requests
pub struct CommandRegistry {
    terminal: Rc<Terminal>,
    registered_commands: Vec<Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        Self {
            terminal: terminal.clone(),
            registered_commands: vec![Box::new(Clear::new(terminal))],
        }
    }

    pub fn execute(&self, input: &str) {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if let Some(command) = tokens.first() {
            if let Some(command) = self
                .registered_commands
                .iter()
                .find(|cmd: &&Box<dyn Command>| cmd.matches(command))
            {
                command.execute();
            } else {
                self.terminal
                    .write(&format!("{}: command not found\n", command));
            }
        }
    }
}
