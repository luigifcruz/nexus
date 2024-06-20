#[derive(Debug, Default, Clone)]
pub struct Instance {
    name: String,
    instance_id: String,
    replicant_id: String,
    version: String,
    tags: Vec<String>,
    host: String,
    port: u16,

    // Metadata
    up_to_date_db: bool,
}

impl Instance {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_instance_id(&self) -> &str {
        &self.instance_id
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

    pub fn set_name(&mut self, name: String) {
        self.up_to_date_db = false;
        self.name = name;
    }

    pub fn set_instance_id(&mut self, instance_id: String) {
        self.up_to_date_db = false;
        self.instance_id = instance_id;
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

    pub fn add_tag(&mut self, tag: String) {
        self.up_to_date_db = false;
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.up_to_date_db = false;
        self.tags.retain(|t| t != tag);
    }

    pub fn clear_tags(&mut self) {
        self.up_to_date_db = false;
        self.tags.clear();
    }

    pub fn set_host(&mut self, host: String) {
        self.up_to_date_db = false;
        self.host = host;
    }

    pub fn set_port(&mut self, port: u16) {
        self.up_to_date_db = false;
        self.port = port;
    }
}