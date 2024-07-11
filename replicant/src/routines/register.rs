use std::error::Error;
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};
use tonic::Request;

use crate::routines::base::Routine;
use crate::state::State;
use crate::client::nexus::{
    DeleteReplicantRequest, ListReplicantsRequest, RegisterReplicantRequest,
};

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

impl Register {
    /// Check if the current replicant is registered with NEXUS.
    ///
    /// This is done by listing all the replicants listed with NEXUS
    /// and checking if the current replicant ID is there.
    pub async fn check_replicant(s: &mut State) -> Result<(), Box<dyn Error>> {
        let request = Request::new(ListReplicantsRequest {});

        if let Some(register) = &mut s.client.register {
            let response = register.list_replicants(request).await?;

            let replicants_id = response.into_inner().replicants_id;
            let replicant_id = s.replicant.get_replicant_id().to_string();

            if !replicants_id.contains(&replicant_id) {
                return Err("Replicant ID not found in NEXUS.".into());
            }
        }

        Ok(())
    }

    /// Registers the current replicant with NEXUS.
    pub async fn register_replicant(s: &mut State) -> Result<(), Box<dyn Error>> {
        let request = Request::new(RegisterReplicantRequest {
            name: s.replicant.get_name().to_string(),
            replicant_id: s.replicant.get_replicant_id().to_string(),
            version: s.replicant.get_version().to_string(),
            tags: s.replicant.get_tags().to_vec(),
            host: s.replicant.get_host().to_string(),
            port: s.replicant.get_port(),
        });

        if let Some(register) = &mut s.client.register {
            register.register_replicant(request).await?;
        }

        Ok(())
    }

    /// Unregister the current replicant with NEXUS.
    pub async fn unregister_replicant(s: &mut State) -> Result<(), Box<dyn Error>> {
        let request = Request::new(DeleteReplicantRequest {
            replicant_id: s.replicant.get_replicant_id().to_string(),
        });

        if let Some(register) = &mut s.client.register {
            register.delete_replicant(request).await?;
        }

        Ok(())
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
                match Register::check_replicant(&mut s).await {
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
                match Register::register_replicant(&mut s).await {
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
                match Register::unregister_replicant(&mut s).await {
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
