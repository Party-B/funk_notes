use crate::timestamp::now_timestamp;

// ============ The Hierarchy ============
#[derive(Debug, Clone)]
pub struct Funknote {
    pub id: usize,
    pub title: String,
    pub description: String, 
    pub created_on: u64,
    pub active: bool,
    pub objects: Vec<usize>,      // IDs of child objects
    pub milestones: Vec<usize>,   // IDs of milestones
}

pub struct Object {
    pub id: usize,
    pub project_id: usize,        // Which project owns this?
    pub title: String,
    pub description: String,
    pub created_on: u64,
    pub active: bool,
    pub items: Vec<usize>,        // IDs of child items
}

pub struct Item {
    pub id: usize,
    pub object_id: usize,         // Which object owns this?
    pub text: String,
    pub created_on: u64,
    pub completed: bool,
    pub completed_on: Option<u64>,
}

pub struct Milestone {
    pub id: usize,
    pub project_id: usize,        // Always belongs to a project
    pub title: String,
    pub description: String,
    pub target_date: u64,
    pub completed: bool,
    pub completed_on: Option<u64>,
    pub target: MilestoneTarget,  // What does this milestone track?
}

// What can a milestone point to?
#[derive(Debug, Clone, PartialEq)]
pub enum MilestoneTarget {
    Project(usize),     // Milestone for the whole project
    Object(usize),      // Milestone for a specific object
    Item(usize),        // Milestone for a specific item
}

// ============ Application State ============

pub struct FunkState {
    pub current_project_id: Option<usize>,  // Which project are we in?
    pub current_object_id: Option<usize>,   // Which object are we in?
    // Maybe later: pub history: Vec<usize>,  // Navigation history
}
