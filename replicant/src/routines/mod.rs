mod ping;
mod base;

use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use std::time::Duration;
use std::sync::mpsc;

use ping::Ping;
use base::Routine;
use crate::state::{State};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Routines {
    #[serde(default)]
    pub ping: Ping,
}

fn spawn<R>(rx: mpsc::Receiver<bool>, 
            tx: mpsc::Sender<bool>, 
            mut state: Arc<Mutex<State>>,
            interval: u64) -> std::thread::JoinHandle<()>
            where R: Routine + Send + 'static {
    std::thread::spawn(move || {
        let worker = async {
            loop {
                if let Err(e) = R::run(&mut state).await {
                    println!("[REPLICANT] Error running routine: {:?}", e);
                    tx.send(true).unwrap();
                }

                if let Ok(_) = rx.recv_timeout(Duration::from_secs(interval)) {
                    println!("[REPLICANT] Stopping routine.");
                    break;
                }
            }
        };
        
        Runtime::new().unwrap().block_on(worker);
    })
}

impl Routines {
    pub async fn initialize(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        Ping::initialize(state).await?;

        Ok(())
    }

    pub async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        let mut routines = vec![];
        let (tx, rx) = mpsc::channel();

        // Start all routines.

        if let Ok(s) = state.lock() {
            routines.push(spawn::<Ping>(rx, tx.clone(), state.clone(), s.routines.ping.interval));
        }

        // Install signal handler to stop all routines.

        ctrlc::set_handler(move || {
            tx.send(true).unwrap();
        }).unwrap();

        // Wait for all routines to stop.

        for routine in routines {
            routine.join().unwrap();
        }

        println!("[REPLICANT] All routines have stopped.");

        Ok(())
    }
}