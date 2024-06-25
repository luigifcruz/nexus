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

struct RoutineThread {
    handle: Option<std::thread::JoinHandle<()>>,
    stop_tx: mpsc::Sender<bool>,
}

fn spawn<R>(stop_rx: mpsc::Receiver<bool>, 
            error_tx: mpsc::Sender<bool>, 
            mut state: Arc<Mutex<State>>) -> std::thread::JoinHandle<()>
            where R: Routine + Send + 'static {
    std::thread::spawn(move || {
        let worker = async {
            loop {
                if let Err(e) = R::run(&mut state).await {
                    println!("[REPLICANT] Error running routine: {:?}", e);
                    error_tx.send(true).unwrap();
                }

                let interval = R::interval(&mut state);
                if let Ok(_) = stop_rx.recv_timeout(Duration::from_secs(interval)) {
                    println!("[REPLICANT] Stopping routine.");
                    break;
                }
            }
        };
        
        Runtime::new().unwrap().block_on(worker);
    })
}

macro_rules! initialize_and_spawn_routine {
    ($routine:ty, $state:expr, $error_tx:expr, $routines:expr) => {
        <$routine>::initialize($state).await?;

        if let Ok(_) = $state.lock() {
            let (stop_tx, stop_rx) = mpsc::channel();

            let handle = spawn::<$routine>(stop_rx, $error_tx.clone(), $state.clone());

            $routines.push(RoutineThread {
                handle: Some(handle),
                stop_tx,
            });
        }
    };
}

impl Routines {
    pub async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        let mut routines = vec![];
        let (error_tx, error_rx) = mpsc::channel();

        // Start all routines.

        initialize_and_spawn_routine!(Ping, state, error_tx, routines);
        initialize_and_spawn_routine!(Ping, state, error_tx, routines);

        // Install signal handler to stop all routines.

        ctrlc::set_handler(move || {
            println!("\n[REPLICANT] Received stop signal.");
            error_tx.send(true).unwrap();
        }).unwrap();

        // Wait for any error signal.

        if let Ok(_) = error_rx.recv() {
            println!("[REPLICANT] Stopping all routines.");

            for routine in routines.iter_mut() {
                routine.stop_tx.send(true).unwrap();
                if let Some(handle) = routine.handle.take() {
                    handle.join().unwrap();
                }
            }
        }

        println!("[REPLICANT] All routines have stopped.");

        Ok(())
    }
}