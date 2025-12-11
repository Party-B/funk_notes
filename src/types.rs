
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
    ListNotes,
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
    
    let contents = fs::read_to_string(METADATA_FILE)?.trim().to_string();  // ← Use ?
    
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
    // Need to put in identifier for 'primary note' attribute
   format!( r#"(next.id={id})"#)
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

/// Parse a single note chunk into a Funknote struct
fn parse_note_chunk(chunk: &str, id: usize) -> Result<Funknote, String> {
    // Split the chunk into lines for parsing
    let lines: Vec<&str> = chunk.lines().collect();
    
    let mut title = String::new();
    let mut description = String::new();
    let mut created_on: u64 = 0;
    let mut active = true;
    
    // Parse the status from the first line if it exists
    if let Some(first_line) = lines.first() {
        if first_line.contains("inactive") {
            active = false;
        }
    }
    
    // Parse each line looking for our fields
    for line in lines {
        let line = line.trim();
        
        if line.contains(".title:") {
            title = line.split_once(".title:")
                .map(|(_, t)| t.trim().to_string())
                .unwrap_or_default();
        } else if line.contains(".description:") {
            description = line.split_once(".description:")
                .map(|(_, d)| d.trim().to_string())
                .unwrap_or_default();
        } else if line.contains(".date:") {
            created_on = line.split_once(".date:")
                .and_then(|(_, d)| d.trim().parse::<u64>().ok())
                .unwrap_or(0);
        }
    }
    
    // Validate we got the essential fields
    if title.is_empty() {
        return Err(format!("Note {} missing title", id));
    }
    
    Ok(Funknote {
        id,
        title,
        description,
        created_on,
        milestone: Vec::new(),
        active,
    })
}

/// Read all notes from the file and return them as a vector
pub fn list_all_notes() -> io::Result<Vec<Funknote>> {
    // Check if file exists first
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new()); // Return empty vector if no notes exist yet
    }
    
    let contents = fs::read_to_string(FILE_PATH)?;
    
    // Split by the delimiter and collect into funknotes
    let notes: Vec<Funknote> = contents
        .split(SPLIT_CODE)
        .enumerate()
        .skip(1) // Skip first empty chunk before first delimiter
        .filter_map(|(idx, chunk)| {
            // Extract the ID from the chunk itself
            let id = chunk
                .lines()
                .find(|line| line.contains(".title:"))
                .and_then(|line| {
                    line.split('.')
                        .next()
                        .and_then(|s| s.trim().parse::<usize>().ok())
                })?;
            
            // Parse the chunk, log errors but continue
            match parse_note_chunk(chunk, id) {
                Ok(note) => Some(note),
                Err(e) => {
                    eprintln!("Warning: Failed to parse note: {}", e);
                    None
                }
            }
        })
        .collect();
    
    Ok(notes)
}
