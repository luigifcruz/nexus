use entities::Replicant;

use crate::cli::Cli;
use crate::server::Server;
use crate::client::Client;
use crate::routines::Routines;

#[derive(Default, Debug)]
pub struct State {
    pub replicant: Replicant,
    pub server: Server,
    pub client: Client,
    pub routines: Routines,
    pub cli: Cli,
}
