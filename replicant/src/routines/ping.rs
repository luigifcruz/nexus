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
    pub connected: bool,
    #[serde(default)]
    pub interval: u64,
    #[serde(default)]
    pub counter: u64,
}

impl Default for Ping {
    fn default() -> Self {
        Ping {
            connected: false,
            interval: 10,
            counter: 0,
        }
    }
}

impl Routine for Ping {
    async fn initialize(_state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        println!("[REPLICANT] Initializing ping routine.");

        Ok(())
    }

    async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        match Client::ping(state).await {
            Ok(_) => {
                if let Ok(mut s) = state.lock() {
                    if !s.routines.ping.connected {
                        println!("[REPLICANT] Nexus connected.");
                        s.routines.ping.counter = 0;
                    }

                    s.routines.ping.connected = true;
                    s.routines.ping.counter += 1;
                }
            },
            Err(e) => {
                if let Ok(mut s) = state.lock() {
                    if s.routines.ping.connected {
                        println!("[REPLICANT] Nexus disconnected.");
                        s.routines.ping.counter = 0;
                    }

                    s.routines.ping.connected = false;
                    s.routines.ping.counter += 1;
                }
                println!("[REPLICANT] Error pinging Nexus: {:?}", e);
            }
        }

        Ok(())
    }

    async fn terminate(_state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        println!("[REPLICANT] Terminating ping routine.");

        Ok(())
    }

    fn interval(state: &mut Arc<Mutex<State>>) -> u64 {
        state.lock().unwrap().routines.ping.interval
    }
}