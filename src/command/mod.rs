pub mod exit;
pub mod echo;
pub mod error;

use crate::command::{echo::EchoCommand, error::ShellError, exit::ExitCommand};

pub trait ShellCommand {
    fn execute(&self) -> Result<(), ShellError>;
}

pub fn parse(input: &str) -> Result<Box<dyn ShellCommand>, ShellError> {
    if input.is_empty() {
        return Err(ShellError::EmptyInput)
    }
    let splitted: Vec<&str> = input.split_whitespace().collect();
    let command_name = splitted[0];
    match command_name {
        "exit" => Ok(Box::new(ExitCommand::new(None))),
        "echo" => {
            let content = splitted[1..].join(" ");
            Ok(Box::new(EchoCommand::new(content)))
        }
        other => Err(ShellError::UnknownCommand(other.to_string()))
    }
}
