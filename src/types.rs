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


