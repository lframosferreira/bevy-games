use bevy::prelude::Event;

#[derive(Event)]
pub struct EndGame {
    pub string: String,
}

impl EndGame {
    pub fn new_number(number: usize) -> Self {
        EndGame {
            string: format!("Score: {number}"),
        }
    }
    pub fn new_bool(boolean: bool) -> Self {
        if boolean {
            EndGame {
                string: "You Won".to_string(),
            }
        } else {
            EndGame {
                string: "You Lost".to_string(),
            }
        }
    }
}
