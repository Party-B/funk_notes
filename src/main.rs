mod types;
use types::*;
use std::io::{self, Write};
use std::fs;
use std::path::Path;

fn main() {
    
    // Cli interface
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    
}

fn parse_command(input: &str) -> Option<Command> {

    let Some((cmd, arg)) = input.to_lowercase().trim().split_once(' ');

    match cmd {
        "new" => return Some(Command::New);
        "add" => return Some(Command::Add);
        "delete" => return Some(Command::Delete);
        "merge" => return Some(Command::Merge);

        _=> return Err("No valid command.");
    }
}


fn execute_command(cmd: Command) {

}


