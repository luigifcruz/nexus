use entities::{Replicant};

use crate::cli::Cli;
use crate::config::Config;
use crate::server::Server;
use crate::client::Client;
use crate::routines::Routines;

#[derive(Default, Debug)]
pub struct State {
    pub healthy: bool,
    pub replicant: Replicant,
    pub config: Config,
    pub server: Server,
    pub client: Client,
    pub routines: Routines,
    pub cli: Cli,
}