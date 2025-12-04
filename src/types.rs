
use crate::timestamp::now_timestamp;
use std::fs;
use std::path::Path;

const COUNTER_FILE: &str = "data/funk_metadata.txt";

pub enum Command {
    New { title: String, description: String },
    AddMilestone { note_id: usize, description: String },
    DeleteNote { note_id: usize },
    // ...other commands
}

#[derive(Debug, Clone, PartialEq)]
struct Milestone {

    id: usize,
    title: String,
    description: String,
    date: u64,
    completed: bool,
    completed_on: Option<u64>,
}

pub struct Funknote {

    pub id: usize,
    pub title: String,
    pub description: String,   
    pub created_on: u64,
    pub milestone: Vec<Milestone>,
    pub active: bool,

}

impl Funknote {

    pub fn new(title: &str, description: &str) -> Funknote {
        Funknote {
            id: get_next_id().expect("REASON"),
            active: true,
            title: title.to_string(),
            created_on: now_timestamp(),
            milestone: Vec::new(),
            description: description.to_string(),
            
        }
    }

}

/// Read next ID from counter file, increment file, return the ID.
pub fn get_next_id() -> Result<usize, std::io::Error> {
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

    Ok(current)
}

