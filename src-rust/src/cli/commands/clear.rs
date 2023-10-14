use std::rc::Rc;

use xterm_js_rs::Terminal;

use super::Command;

pub struct Clear {
    terminal: Rc<Terminal>,
}

impl Clear {
    pub fn new(terminal: Rc<Terminal>) -> Self {
        Self { terminal }
    }
}

impl Command for Clear {
    fn matches(&self, command: &str) -> bool {
        command == "clear"
    }

    fn execute(&self) {
        self.terminal.clear();
    }
}
