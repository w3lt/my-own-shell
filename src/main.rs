#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Create a new variable to store user input
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or_else(|err| {
        eprintln!("Error: an unkown error occured: {err}");
        exit(1);
    });

    input = input.trim().to_string();

    println!("{input}: command not found")
}
