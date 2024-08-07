use std::error::Error;
use std::sync::{Mutex, Arc};

use crate::state::State;

pub trait Routine {
    async fn initialize(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>>;
    async fn run(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>>;
    async fn terminate(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>>;

    fn interval(state: &mut Arc<Mutex<State>>) -> u64;
}
