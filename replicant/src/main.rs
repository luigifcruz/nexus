mod serial_number;
mod services;
mod state;
mod server;
mod config;
mod cli;
mod client;
mod routines;

use std::error::Error;
use std::sync::{Mutex, Arc};

use config::Config;
use cli::Cli;
use server::Server;
use client::Client;
use routines::Routines;
use state::State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[REPLICANT] Welcome to a Replicant!");

    let mut state = Arc::new(Mutex::new(State::default()));

    // WARNING: The order of these calls is **very** important!

    Cli::parse(&mut state)?;
    Server::initialize(&mut state).await?;
    Config::parse(&mut state)?;
    Client::initialize(&mut state).await?;
    Server::serve(&mut state).await?;
    Routines::run(&mut state).await?;

    println!("[REPLICANT] Goodbye!");

    Ok(())
}
