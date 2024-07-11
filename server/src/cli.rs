use std::error::Error;
use std::sync::{Mutex, Arc};
use clap::Parser;

use crate::state::State;

#[derive(Default, Debug, Parser)]
pub struct Args {
    #[arg(value_name = "config", help = "Path for the NEXUS configuration file.")]
    pub config: String,

    #[arg(long, default_value = "0.0.0.0", help = "Host address to listen on.")]
    pub host: String,

    #[arg(long, default_value = "50051", help = "Port number to listen on.")]
    pub port: u16,
}

#[derive(Default, Debug)]
pub struct Cli {
    pub args: Args,
}

impl Cli {
    pub fn parse(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            s.cli.args = Args::parse();
        }

        Ok(())
    }
}