use std::fmt::Error;

use crate::types::*;

// Here we'll do all the actual function work with the types
pub fn new_method(target: &str, action: &str) {
    // Implement the logic for the new method here
    // Check what args are coming in and handle accordingly
    // just testing output for now with a println
    println!("Performing action '{}' on target '{}'", action, target);
}