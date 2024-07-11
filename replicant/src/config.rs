use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};
use entities::Replicant;

use crate::client::Client;
use crate::routines::Routines;
use crate::serial_number::SerialNumber;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
struct YamlConfig {
    version: String,
    nexus: Client,
    replicant: Replicant,
    #[serde(default)]
    routines: Routines,
}

impl YamlConfig {
    fn fill_state_with_server_info(&self, s: &mut State) {
        if let Some(listener) = &s.server.listener {
            let local_addr = listener.local_addr().unwrap();
            s.replicant.set_host(local_addr.ip().to_string());
            s.replicant.set_port(local_addr.port().into());
        }
    }

    fn fill_state_with_replicant_info(&self, s: &mut State) {
        s.replicant.set_replicant_id(SerialNumber::new().to_string());
        s.replicant.set_version(env!("CARGO_PKG_VERSION").to_string());
        s.replicant.set_name(self.replicant.get_name().to_string());
        s.replicant.set_tags(self.replicant.get_tags().clone());
    }

    fn fill_state_with_hardware_info(&self, s: &mut State) {
        s.replicant.set_hardware(self.replicant.get_hardware().clone());
    }

    fn fill_state_with_nexus_info(&self, s: &mut State) {
        s.client = self.nexus.clone();
    }

    fn fill_state_with_routines_info(&self, s: &mut State) {
        s.routines = self.routines.clone();
    }
}

#[derive(Default, Debug)]
pub struct Config;

impl Config {
    pub fn parse(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() {
            println!("[REPLICANT] Opening configuration file: '{}'", s.cli.args.config);

            if !std::path::Path::new(&s.cli.args.config).exists() {
                return Err("Configuration file does not exist.".into());
            }

            let file = File::open(&s.cli.args.config)?;
            let reader = BufReader::new(file);
            let config: YamlConfig = serde_yaml::from_reader(reader)?;

            if config.version != "1.0" {
                return Err("Configuration file version mismatch.".into());
            }

            config.fill_state_with_server_info(&mut s);

            config.fill_state_with_replicant_info(&mut s);
            config.fill_state_with_hardware_info(&mut s);
            config.fill_state_with_nexus_info(&mut s);
            config.fill_state_with_routines_info(&mut s);
        }

        Ok(())
    }
}
