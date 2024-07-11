mod register;
mod ping;
mod base;

use std::error::Error;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use std::time::Duration;
use std::sync::mpsc;
use std::collections::HashMap;

use ping::Ping;
use register::Register;
use base::Routine;
use crate::state::State;

enum InterruptType {
    Stop,
    Notify
}

#[derive(Debug, Default)]
struct Worker {
    pub handle: Option<std::thread::JoinHandle<()>>,
    pub interrupt_tx: Option<mpsc::Sender<InterruptType>>,
}

impl Worker {
    pub fn notify(&mut self) {
        if let Some(tx) = self.interrupt_tx.take() {
            tx.send(InterruptType::Notify).unwrap();
        }
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.interrupt_tx.take() {
            tx.send(InterruptType::Stop).unwrap();
        }

        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Routines {
    #[serde(default)]
    pub ping: Ping,
    #[serde(default)]
    pub register: Register,
    #[serde(skip)]
    workers: Arc<Mutex<HashMap<String, Worker>>>,
}

fn spawn<R>(interrupt_rx: mpsc::Receiver<InterruptType>,
            error_tx: mpsc::Sender<bool>,
            mut state: Arc<Mutex<State>>) -> std::thread::JoinHandle<()>
            where R: Routine + Send + 'static {
    std::thread::spawn(move || {
        let worker = async {
            R::initialize(&mut state).await.unwrap();

            loop {
                if let Err(e) = R::run(&mut state).await {
                    println!("[REPLICANT] Error running routine: {:?}", e);
                    error_tx.send(true).unwrap();
                }

                let interval = R::interval(&mut state);
                if let Ok(i) = interrupt_rx.recv_timeout(Duration::from_secs(interval)) {
                    match i {
                        InterruptType::Stop => {
                            R::terminate(&mut state).await.unwrap();
                            break;
                        },
                        InterruptType::Notify => { }
                    }
                }
            }

            println!("[REPLICANT] Routine stopped.");
        };

        Runtime::new().unwrap().block_on(worker);
    })
}

macro_rules! initialize_and_spawn_routine {
    ($routine:ty, $state:expr, $error_tx:expr, $workers:expr) => {
        let (interrupt_tx, interrupt_rx) = mpsc::channel();
        let handle = spawn::<$routine>(interrupt_rx, $error_tx.clone(), $state.clone());
        if let Ok(mut w) = $workers.lock() {
            w.insert(stringify!($routine).to_lowercase().to_string(), Worker {
                handle: Some(handle),
                interrupt_tx: Some(interrupt_tx),
            });
        }
    };
}

impl Routines {
    pub async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        let workers = Arc::new(Mutex::new(HashMap::new()));
        let (error_tx, error_rx) = mpsc::channel();

        // Reference workers in state.

        if let Ok(mut s) = state.lock() {
            s.routines.workers = workers.clone();
        }

        // Start all routines.

        initialize_and_spawn_routine!(Ping, state, error_tx, workers);
        initialize_and_spawn_routine!(Register, state, error_tx, workers);

        // Install signal handler to stop all routines.

        ctrlc::set_handler(move || {
            println!("\n[REPLICANT] Received stop signal.");
            error_tx.send(true).unwrap();
        }).unwrap();

        // Wait for any error signal.

        if let Ok(_) = error_rx.recv() {
            println!("[REPLICANT] Stopping all routines.");

            if let Ok(mut w) = workers.lock() {
                for (a, worker) in w.iter_mut() {
                    println!("[REPLICANT] Stopping routine: {}", a);
                    worker.stop();
                }
            }
        }

        println!("[REPLICANT] All routines have stopped.");

        Ok(())
    }

    pub fn notify(s: &mut State, routine: &str) {
        if let Ok(mut w) = s.routines.workers.lock() {
            if let Some(worker) = w.get_mut(routine) {
                worker.notify();
            }
        }
    }
}
