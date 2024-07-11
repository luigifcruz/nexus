use std::error::Error;
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

use crate::methods;
use crate::routines::base::Routine;
use crate::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    #[serde(default)]
    pub registered: bool,
    #[serde(default)]
    pub interval: u64,
    #[serde(default)]
    pub counter: u64,
}

impl Default for Register {
    fn default() -> Self {
        Register {
            registered: false,
            interval: 10,
            counter: 0,
        }
    }
}

impl Routine for Register {
    async fn initialize(_state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        println!("[REPLICANT] Initializing register routine.");

        Ok(())
    }

    async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            // If not connected, do nothing.

            if !s.routines.ping.connected {
                return Ok(());
            }

            // If registered, check if we are really registered.

            if s.routines.register.registered {
                match methods::Register::check_replicant(&mut s).await {
                    Ok(_) => {
                        s.routines.register.counter += 1;
                    },
                    Err(e) => {
                        println!("[REPLICANT] Error checking in with Nexus: {:?}", e);
                        s.routines.register.counter = 0;
                        s.routines.register.registered = false;
                    }
                }
            }

            // If not registered, attempt to register.

            if !s.routines.register.registered {
                match methods::Register::register_replicant(&mut s).await {
                    Ok(_) => {
                        println!("[REPLICANT] Successfully registered with Nexus.");
                        s.routines.register.counter = 0;
                        s.routines.register.registered = true;
                    },
                    Err(_) => {
                        s.routines.register.counter += 1;
                    }
                }
            }
        }

        Ok(())
    }

    async fn terminate(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        println!("[REPLICANT] Terminating register routine.");

        if let Ok(mut s) = state.lock() {
            // If not connected, do nothing.

            if !s.routines.ping.connected {
                return Ok(());
            }

            // If registered, unregister.

            if s.routines.register.registered {
                match methods::Register::unregister_replicant(&mut s).await {
                    Ok(_) => {
                        println!("[REPLICANT] Successfully unregistered with Nexus.");
                    },
                    Err(e) => {
                        println!("[REPLICANT] Error unregistering with Nexus: {:?}", e);
                    }
                }
            }
        }

        Ok(())
    }

    fn interval(state: &mut Arc<Mutex<State>>) -> u64 {
        state.lock().unwrap().routines.register.interval
    }
}
