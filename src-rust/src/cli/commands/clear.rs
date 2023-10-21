use super::Command;

pub struct Clear;

impl Command for Clear {
    fn execute(&self, _tokens: &Vec<&str>, terminal: &xterm_js_rs::Terminal) -> bool {
        // TODO: Behavior if command is invoked with args
        terminal.clear();
        false
    }
}
