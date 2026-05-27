#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

mod command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Create a new variable to store user input
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap_or_else(|err| {
            eprintln!("Error: an unkown error occured: {err}");
            exit(1);
        });

        input = input.trim().to_string();

        match command::command::parse(input) {
            Ok(command) => {
                command.execute();
            },
            Err(err) => {
                eprintln!("{err}");
            }
        };
    }
}
