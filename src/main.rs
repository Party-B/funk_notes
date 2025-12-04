mod types;
mod timestamp;
use crate::types::{Command, Funknote};
use crate::timestamp::now_timestamp;
use std::io;

// Constants
const EXIT_COMMANDS: &[&str] = &["exit", "quit", "q", "drop"];

fn main() {
    let mut input = String::new();
    println!("Enter command (or 'shell' for shell mode):");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "shell" {
        // Shell mode loop
        loop {
            let mut shell_input = String::new();
            print!("funk_notes> ");
            //std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut shell_input).unwrap();
            let shell_input = shell_input.trim();

            if EXIT_COMMANDS.contains(&shell_input) {
                println!("Exiting shell mode.");
                break;
            }

            match parse_command(shell_input) {
                Some(cmd) => execute_command(cmd),
                None => println!("No valid command."),
            }
        }
    } else {
        // Single command mode
        match parse_command(input) {
            Some(cmd) => execute_command(cmd),
            None => println!("No valid command."),
        }
    }
}

fn parse_command(input: &str) -> Option<Command> {
    let mut parts = input.splitn(3, ' ');
    let cmd = parts.next()?;
    match cmd {
        "new" => {
            let title = parts.next()?.to_string();
            let description = parts.next().unwrap_or("").to_string();
            Some(Command::New { title, description })
        }
        // Add other commands here
        _ => None,
    }
}


fn execute_command(cmd: Command) {

    match cmd {
        Command::New { title, description } => {
            let note = Funknote::new(&title, &description);
            println!("Created new note: {}", note.title);
        }
        _ => println!("No valid command."),
    }
}



