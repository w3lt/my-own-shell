use crate::command::command::ShellCommand;

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
    fn execute(&self) {
        println!("{}", self.content)
    }
}