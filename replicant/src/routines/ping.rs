use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

use crate::client::Client;
use crate::routines::base::Routine;
use crate::state::{State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {
    #[serde(default)]
    pub interval: u64,
    #[serde(default)]
    pub timeout: u64,
}

impl Default for Ping {
    fn default() -> Self {
        Ping {
            interval: 10,
            timeout: 60,
        }
    }
}

impl Routine for Ping {
    async fn initialize(_state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        println!("[REPLICANT] Initializing ping routine.");

        Ok(())
    }

    async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        println!("[REPLICANT] Running ping routine.");

        Client::ping(state).await?;

        Ok(())
    }
}