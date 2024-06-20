use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

use crate::state::{State};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Database {
    pub path: String,
}

impl Database {
    pub async fn initialize(_state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        // TODO: Implement database initialization.
        Ok(())
    }
}