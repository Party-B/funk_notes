mod interpret;
mod types;
mod functions;
mod timestamp;
mod storage;
mod operations;

use crate::functions::{handle_input, MethodRegistry};
use crate::interpret::interpret;
use crate::types::FunkState;
use std::env;
use std::io::{self, Write};

const EXIT_CMDS: &[&str] = &["exit", "quit", "q", "drop"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let full_input = args[1..].join(" ");
    
    let registry = MethodRegistry::new();
    
    if full_input.contains("shell()") || full_input == "shell" {
        run_shell(registry);
    } else if !full_input.is_empty() {
        let mut state = FunkState::new();
        match interpret(&full_input) {
            Ok(ast) => functions::handle_input(ast, &mut state, &registry),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Usage:");
        println!("  funknotes shell              - Enter REPL");
        println!("  funknotes .new().title(\"X\")  - Single command");
    }
}

fn run_shell(registry: MethodRegistry) {
    let mut state = FunkState::new();
    println!("Entering funk_notes shell.");
    println!("Available methods: {:?}", registry.list_methods());
    println!("Prefix with '?' to just parse without executing.\n");
    
    loop {
        let mut input = String::new();
        print!("funk> ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if EXIT_CMDS.contains(&input) {
            println!("Exiting shell mode.");
            break;
        }
        
        if input.trim() == "help" {
            registry.print_all_methods();
            continue;
        }

        if !input.is_empty() {
            if input.starts_with('?') {
                match interpret(&input[1..]) {
                    Ok(ast) => println!("✓ {:#?}\n", ast),
                    Err(e) => println!("✗ {}\n", e),
                }
            } else {
                match interpret(input) {
                    Ok(ast) => functions::handle_input(ast, &mut state, &registry),
                    Err(e) => println!("✗ {}\n", e),
                }
            }
        }
    }
}
