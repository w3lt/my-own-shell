use crate::command::{ShellCommand, error::ShellError};

pub struct EchoCommand {
    content: String
}

impl EchoCommand {
    pub fn new(content: String) -> Self {
        EchoCommand {
            content
        }
    }
}

impl ShellCommand for EchoCommand {
    fn execute(&self) -> Result<(), ShellError> {
        Ok(println!("{}", self.content))
    }
}