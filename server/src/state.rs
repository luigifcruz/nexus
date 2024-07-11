use std::collections::HashMap;

use entities::Replicant;
use crate::cli::Cli;
use crate::server::Server;
use crate::database::Database;

#[derive(Debug, Default)]
pub struct State {
    pub version: String,
    pub replicants: HashMap<String, Replicant>,
    pub server: Server,
    pub database: Database,
    pub cli: Cli,
}
