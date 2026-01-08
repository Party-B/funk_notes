use std::fmt::Error;

use crate::timestamp::now_timestamp;
use crate::types::*;

pub enum actions {

    create,
    delete,
    list,
}

pub enum Note {
    Project(Project),
    Object(Object),
    Item(Item),
    Milestone(Milestone),   
}

pub struct Operations {
    notes: Vec<Note>,

}


// Here we'll do all the actual function work with the types
pub fn new_method(target_object: &str, title: &str) {
    // Implement the logic for the new method here
    // Check what args are coming in and handle accordingly
    // just testing output for now with a println
    println!("Creating a new '{}' with title '{}'", target_object, title);

    match target_object {
        "project" => {
            let new_project = Project {
                id: 0, // This would be generated
                title: title.to_string(),
                description: String::new(),
                created_on: now_timestamp(),
                active: true,
                objects: Vec::new(),
                milestones: Vec::new(),
            };
            println!("New project created: {:?}", new_project);
        }
        "object" => println!("Creating a new object with title '{}'", title),
        "item" => println!("Creating a new item with title '{}'", title),
        "milestone" => println!("Creating a new milestone with title '{}'", title),
        _ => println!("Invalid target object: {}", target_object),
    }
    
}
