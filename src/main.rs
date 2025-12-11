mod types;
mod timestamp;
use crate::types::{Command, Funknote};
use crate::timestamp::now_timestamp;
use std::io::{self, Write};

// Constants
const EXIT_CMDS: &[&str] = &["exit", "quit", "q", "drop"];

fn main() {

    // Collect cli args
    let args = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");

    // Assess if the first word is the shell command
    if args.split_whitespace().next() == Some("shell") {
        
        // Enter the shell loop
        loop {

            let mut shell_input = String::new();
            print!("funk_notes> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut shell_input).unwrap();
            let shell_input = shell_input.trim();

            if EXIT_CMDS.contains(&shell_input) {
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
        match parse_command(&args) {
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
        "list" => {
            // Implement list command parsing
            Some(Command::ListNotes)
        }
        "add_milestone" => {
            let note_id_str = parts.next()?;
            let note_id = note_id_str.parse::<usize>().ok()?;
            let description = parts.next()?.to_string();
            Some(Command::AddMilestone { note_id, description })
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
        Command::ListNotes => {
            match types::list_all_notes() {
                Ok(notes) => {
                    if notes.is_empty() {
                        println!("No notes found.");
                    } else {
                        println!("\n=== Your Funknotes ===\n");
                        for note in notes {
                            let status = if note.active { "Active" } else { "Inactive" };
                            println!("[{}] {} - {} ({})", 
                                note.id, 
                                note.title, 
                                note.description,
                                status
                            );
                        }
                        println!(); // Empty line at end
                    }
                }
                Err(e) => eprintln!("Error reading notes: {}", e),
            }
        }
        _ => println!("No valid command."),
    }
}




