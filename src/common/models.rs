use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessConfig {
    pub id: String,
    pub name: String,
    pub pid: u8,
    pub context_dir: String,
    pub image: Image,
    pub job: Job,
    pub env: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub name: String,
    pub context_temp_dir: String,
    pub imgpath: String,
    pub tag: String,
    pub created: String, // TODO: maybe a proper datetime?
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub command: Vec<String>,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub ports: Vec<Port>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub host_port: u16,
    pub proc_port: u16,
}

