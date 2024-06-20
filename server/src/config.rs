use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

use crate::database::Database;

use crate::state::{State};

#[derive(Debug, Serialize, Deserialize)]
struct YamlConfig {
    version: String,
    database: Database,
}

impl YamlConfig {
    fn fill_state_with_default_info(&self, s: &mut State) {
        s.healthy = true;
        s.version = env!("CARGO_PKG_VERSION").to_string();
    }
    
    fn fill_state_with_database_info(&self, s: &mut State) {
        s.database = self.database.clone();
    }
}

#[derive(Default, Debug)]
pub struct Config;

impl Config {
    pub fn parse(state: &mut Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
        if let Ok(mut s) = state.lock() { 
            println!("[NEXUS] Opening configuration file: '{}'", s.cli.args.config);

            if !std::path::Path::new(&s.cli.args.config).exists() {
                return Err("Configuration file does not exist.".into());
            }

            let file = File::open(&s.cli.args.config)?;
            let reader = BufReader::new(file);
            let config: YamlConfig = serde_yaml::from_reader(reader)?;

            if config.version != "1.0" {
                return Err("Configuration file version mismatch.".into());
            }

            config.fill_state_with_default_info(&mut s);

            config.fill_state_with_database_info(&mut s); 
        }

        Ok(())
    }
}