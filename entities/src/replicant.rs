use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use crate::instance::Instance;
use crate::hardware::Hardware;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Replicant {
    name: String,
    #[serde(default)]
    hardware: Hardware,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(skip)]
    replicant_id: String,
    #[serde(skip)]
    version: String,
    #[serde(skip)]
    host: String,
    #[serde(skip)]
    port: u16,
    #[serde(skip)]
    instances: Option<HashMap<String, Instance>>,

    // Metadata
    #[serde(skip)]
    up_to_date_db: bool,
}

impl Replicant {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_replicant_id(&self) -> &str {
        &self.replicant_id
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_hardware(&self) -> &Hardware {
        &self.hardware
    }

    pub fn get_instances(&self) -> Option<&HashMap<String, Instance>> {
        self.instances.as_ref()
    }

    pub fn set_name(&mut self, name: String) {
        self.up_to_date_db = false;
        self.name = name;
    }

    pub fn set_replicant_id(&mut self, replicant_id: String) {
        self.up_to_date_db = false;
        self.replicant_id = replicant_id;
    }

    pub fn set_version(&mut self, version: String) {
        self.up_to_date_db = false;
        self.version = version;
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.up_to_date_db = false;
        self.tags = tags;
    }

    pub fn set_host(&mut self, host: String) {
        self.up_to_date_db = false;
        self.host = host;
    }

    pub fn set_port(&mut self, port: u16) {
        self.up_to_date_db = false;
        self.port = port;
    }

    pub fn set_hardware(&mut self, hardware: Hardware) {
        self.up_to_date_db = false;
        self.hardware = hardware;
    }

    pub fn add_instance(&mut self, name: String, instance: Instance) {
        self.up_to_date_db = false;

        if let Some(instances) = &mut self.instances {
            instances.insert(name, instance);
        } else {
            let mut instances = HashMap::new();
            instances.insert(name, instance);
            self.instances = Some(instances);
        }
    }

    pub fn remove_instance(&mut self, name: &str) {
        self.up_to_date_db = false;

        if let Some(instances) = &mut self.instances {
            instances.remove(name);
        }
    }

    pub fn clear_instances(&mut self) {
        self.up_to_date_db = false;

        self.instances = None;
    }
}