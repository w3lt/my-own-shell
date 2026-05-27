use std::process::exit;

use crate::command::command::ShellCommand;

pub struct ExitCommand {
    code: i32,
}

impl ExitCommand {
    pub fn new(code: Option<i32>) -> Self {
        let code = code.unwrap_or(0);
        Self { code }
    }
}

impl ShellCommand for ExitCommand {
    fn execute(&self) {
        exit(self.code)
    }
}
