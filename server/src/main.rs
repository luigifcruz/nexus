// TODO: Timeout service for Replicants.
//         - Periodically ping Replicants. If no response, remove Replicant from list.

// TODO: Health service for Instances.
//         - Periodically check status of Instances and update DB.
// TODO: Health service for Runners.
//         - Periodically check status of Runners and update DB.

// TODO: Metrics collection service for Replicants.
// TODO: Metrics collection service for Instances.

// TODO: Notify update service.

mod services;
mod state;
mod cli;
mod config;
mod server;
mod database;

use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use config::Config;
use cli::Cli;
use server::Server;
use database::Database;
use state::{State};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[NEXUS] Welcome to Nexus!");

    let mut state = Arc::new(Mutex::new(State::default()));

    // WARNING: The order of these calls is **very** important!

    Cli::parse(&mut state)?;
    Server::initialize(&mut state).await?;
    Config::parse(&mut state)?;
    Database::initialize(&mut state).await?;
    Server::serve(&mut state).await?;

    println!("[NEXUS] Goodbye!");

    Ok(())
}
