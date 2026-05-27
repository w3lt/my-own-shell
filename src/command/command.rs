use std::io;

use crate::command::{echo::EchoCommand, exit::ExitCommand};

pub trait ShellCommand {
    fn execute(&self);
}

pub fn parse(input: String) -> Result<Box<dyn ShellCommand>, io::Error> {
    let splitted: Vec<&str> = input.split_whitespace().collect();
    let command_name = splitted[0];
    match command_name {
        "exit" => Ok(Box::new(ExitCommand::new(None))),
        "echo" => {
            let content = splitted[1..].join(" ");
            Ok(Box::new(EchoCommand::new(content)))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}: command not found", command_name),
        ))
    }
}
