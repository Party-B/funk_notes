
use crate::timestamp::now_timestamp;
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::env;

// Constants
const METADATA_FILE: &str = "funk_metadata.txt";
const SPLIT_CODE: &str = "(note.id";
const FILE_PATH: &str = "all_notes.txt";


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
        let note = Funknote {
            id: get_next_id().expect("REASON"),
            active: true,
            title: title.to_string(),
            created_on: now_timestamp(),
            milestone: Vec::new(),
            description: description.to_string(),
            
        };
        write_funknote_to_file(&note).expect("Failed to write funk note to file");
        return note;
    }
}

pub fn write_funknote_to_file(note: &Funknote) -> io::Result<()> {
    
    let note_str = base_note(note);
    append_to_file(FILE_PATH, &note_str)
}

/// Read next ID from counter file, increment file, return the ID.
pub fn get_next_id() -> Result<usize, std::io::Error> {
    // Make sure file exists. If not, create it with "1".
    if !Path::new(METADATA_FILE).exists() {
        fs::write(METADATA_FILE, base_meta(1))?;
        return Ok(1 as usize);
    }
    
    let contents = fs::read_to_string(METADATA_FILE)?;  // ← Use ?
    
    let value: i64 = contents
        .split_once('=')
        .and_then(|(_, right)| right.trim_end_matches(')').parse::<i64>().ok())
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid counter format"
        ))?;
    
    let value = value + 1;
    
    // Safe conversion with validation
    let value_usize = usize::try_from(value)
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Counter value out of range"
        ))?;
    
    fs::write(METADATA_FILE, base_meta(value))?;  // ← Use ?
    Ok(value_usize)
}

fn base_note(note: &Funknote) -> String {
    // First build a local set of variables to fill the format string
    let id = note.id;
    let status = if note.active { "active" } else { "inactive" };
    let title = &note.title;
    let description = &note.description;
    let created_on = note.created_on;

    // Using a raw string literal - no escaping needed
   format!( r#"
(note.id{id}.{status}.start)

{id}.title: {title}

{id}.description: {description}

{id}.date: {created_on}

    ##### Milestones

(note.id.{id}.end)
"#)
}

fn base_meta(id: i64) -> String {
    // Using a raw string literal - no escaping needed
   format!( r#"
(next.id={id})

"#)
}

fn append_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)   // Creates if doesn't exist
        .append(true)   // Appends if does exist
        .open(file_path)?;
    
    file.write_all(content.as_bytes())?;
    Ok(())
}
fn read_from_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}
fn chunked(text: &str, chunkno: i64) -> Option<String> {
    if chunkno < 1 { return None }
    
    text.split(SPLIT_CODE)
        .nth(chunkno as usize)
        .map(|s| s.to_string())
}

fn next_chunk(file: &str) -> io::Result<i64> {
    let content = read_from_file(file).unwrap_or_default();  // Empty string if error
    Ok(content.split(SPLIT_CODE).count() as i64)
}


