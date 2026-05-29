use crate::command::{ShellCommand, BUILTIN_COMMANDS, error::ShellError};

pub struct TypeCommand {
    command: Vec<String>
}

impl TypeCommand {
    pub fn new(command: Vec<String>) -> Self {
        TypeCommand {
            command
        }
    }
}

impl ShellCommand for TypeCommand {
    fn execute(&self) -> Result<(), ShellError> {
        for c in &self.command {
            if BUILTIN_COMMANDS.contains(&c.as_str()) {
                println!("{c} is a shell builtin")
            } else {
                println!("{c}: not found")
            }
        }
        
        Ok(())
    }
}