mod timestamp;
use timestamp::*;
use std::fs;
use std::path::Path;

const COUNTER_FILE: &str = "data/next_id.txt";

#[derive(Debug, Clone, Copy, PartialEq)]
struct Milestones {

    id: u32,
    description: String,
    date: u64,
}

pub enum Command {

    New(String),
    Add(String),
    Delete(String),
    Merge(String, String),
    Drop,
    Push(String, String),
    Pop(String, String),

}

pub struct Funk_note {

    pub id: usize,
    pub active: bool,
    pub title: String,
    pub created_on: u64,
    pub milestone: Vec<Milestones>,
    pub description: String,

}

impl Funk_note {

    pub fn new(title: &str, description: &str) -> Funk_note {
        Funk_note {
            id: get_next_id(),
            active = true,
            title,
            created_on: Timestamp::now(),
            milestone: Vec::new(),
            description,
            
        }
    }
}

/// Read next ID from counter file, increment file, return the ID.
pub fn get_next_id() -> usize {
    // Make sure file exists. If not, create it with "1".
    if !Path::new(COUNTER_FILE).exists() {
        fs::write(COUNTER_FILE, "1").expect("Failed to create counter file");
    }

    let contents = fs::read_to_string(COUNTER_FILE)
        .expect("Failed to read counter file");

    let current: usize = contents.trim().parse()
        .expect("Counter file does not contain a valid number");

    // Increment and save
    let next = current + 1;
    fs::write(COUNTER_FILE, next.to_string())
        .expect("Failed to write counter file");

    current
}

