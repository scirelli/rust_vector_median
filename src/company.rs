use std::collections::HashMap;

pub struct Company {
    pub database: HashMap<String, Vec<String>>,
}

pub struct Entry {
    pub name:String,
    pub department: String,
}

impl Company {
    pub fn new() -> Company {
        Company {
            database: HashMap::new()
        }
    }
}
