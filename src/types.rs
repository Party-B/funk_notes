#[derive(Debug, Clone, Copy, PartialEq)]

mod timestamp;
use timestamp::*;

pub enum Status {
    // Variants of note status
    Active,
    Deleted,
}

struct Milestones {

    id: u32,
    description: String,
    date: u64,
}

pub struct Funk_note {

    pub id: usize,
    pub title: String,
    pub created_on: u64,
    pub milestone: Vec<Milestones>,
    pub description: String,

}


