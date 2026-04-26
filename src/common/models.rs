use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessConfig {
    pub id: String,
    pub name: String,
    pub pid: u8,
    pub context_dir: String,
    pub image: Image,
    pub job: Job,
    pub env: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: String,
    pub name: String,
    pub context_temp_dir: String,
    pub imgpath: String,
    pub tag: String,
    pub created: String, // TODO: maybe a proper datetime?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub name: String,
    pub command: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub ports: Vec<Port>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub host_port: u16,
    pub proc_port: u16,
}

pub enum ContainerStatus {
    Creating,
    Created,
    Running,
    Stopped(i32),
}

#[warn(dead_code)]
pub struct ContainerState {
    pub oci_version: String,
    pub id: String,
    pub status: ContainerStatus,
    pub pid: u32,
    pub bundle: PathBuf,
    pub annotations: HashMap<String, String>,
}

impl fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContainerStatus::Creating => write!(f, "creating"),
            ContainerStatus::Created => write!(f, "created"),
            ContainerStatus::Running => write!(f, "running"),
            ContainerStatus::Stopped(_) => write!(f, "stopped"),
        }
    }
}


#[warn(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciSpec {
    pub oci_version: String,
    pub root: Option<OciRoot>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciRoot {
    pub path: PathBuf,
    pub readonly: Option<bool>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciMount {
    pub destination: PathBuf,
    pub source: Option<PathBuf>,
    pub options: Option<LinuxMountOptions>
}


// TODO: Support the rest of the bind mount options 
// right now only the following are present 
// you need to add at least the required ones to 
// have it oci compatible
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinuxMountOptions {
      Bind,
      Rbind,
      Ro,
      Rw,
      Noexec,
      Exec,
      Nosuid,
      Suid,
      Nodev,
      Dev,
      Private,
      Rprivate,
}


