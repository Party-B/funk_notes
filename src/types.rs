use crate::timestamp::now_timestamp;
use crate::storage::*;

#[derive(Debug, Clone)]
pub struct FunkState {

    pub title: String
    
}

impl FunkState {

    pub fn new() -> Self {
        Self {
            title: String::from("New"),
        }
    }
}

#[derive(Debug)]
pub struct Project {

    pub name: String,

}

impl Project {

    pub fn new(name: &str) -> Self {
        Self {
            //id: 
            name: name.to_string(),

        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct Milestone {

    id: usize,
    title: String,
    description: String,
    date: u64,
    completed: bool,
    completed_on: Option<u64>,
}

pub struct Object {

    pub id: usize,
    pub title: String,
    pub description: String,   
    pub created_on: u64,

}

pub struct Item {

    pub id: usize,
    pub text: String,
    pub created_on: u64,

}


