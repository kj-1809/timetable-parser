use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub name: String,
    pub location: String,
    pub professor: String,
    pub slot: u32,
}

impl Clone for Class {
    fn clone(&self) -> Self {
        Class {
            name: self.name.clone(),
            location: self.location.clone(),
            professor: self.professor.clone(),
            slot: self.slot.clone(),
        }
    }
}
