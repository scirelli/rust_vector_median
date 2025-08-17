use std::collections::HashMap;

pub struct Company {
    pub database: HashMap<String, String>,
}

impl Company {
    pub fn new() -> Company {
        Company {
            database: HashMap::new()
        }
    }
}
