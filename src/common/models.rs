use std::collections::HashMap;

pub type ProcessEnv = HashMap<String, String>;

pub struct Image {
    pub id: String,
    pub name: String,
    pub context_temp_dir: String,
    pub imgpath: String,
    pub tag: String,
    pub created: String,
}

pub struct PortMap {
    pub host_port: String,
    pub proc_port: String,
}

pub type PortMapping = Vec<PortMap>;

pub struct ProcessNetwork {
    pub pm: PortMapping,
}


pub type ProcessJobCommand = Vec<String>;

pub struct ProcessJob {
    pub name: String,
    pub command: ProcessJobCommand,
}

pub struct Process {
    // params of the process YAML
    pub id: String,
    pub name: String,
    pub pid: u8,
    pub context_dir: String,
    pub image: Image,
    pub job: ProcessJob,
    pub env: ProcessEnv,
    pub network: ProcessNetwork,
}


impl Process {
    pub fn pprint(&self) {
        println!("id: {}", &self.id);
        println!("name: {}", &self.name);
    }
}



