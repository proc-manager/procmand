extern crate yaml_rust;

use std::fs;

use crate::common::models;
use yaml_rust::{YamlLoader, Yaml};


fn get_image_struct(img: &Yaml) -> models::Image {    
    models::Image{
        id: String::from(img["id"].as_str().unwrap()),
        name: String::from(img["name"].as_str().unwrap()),
        context_temp_dir: String::from(img["context_temp_dir"].as_str().unwrap()),
        imgpath: String::from(img["imgpath"].as_str().unwrap()),
        tag: String::from(img["tag"].as_str().unwrap()),
        created: String::from(img["created"].as_str().unwrap()),
    }
}


fn get_process_job_struct(job: &Yaml) 
    -> models::ProcessJob {

    let mut command = models::ProcessJobCommand::new();

    if let Some(array) = job["items"].as_vec() {
        for item in array {
            if let Some(string_val) = item.as_str() {
                command.push(String::from(string_val))
            }
        }
    }

    models::ProcessJob {
        name: String::from(job["name"].as_str().unwrap()),
        command,
    }
}


fn get_process_env(en: &Yaml) -> models::ProcessEnv {
    let mut e = models::ProcessEnv::new();
   
    let e_hash = en.as_hash().unwrap();
    for (key, val) in e_hash {
        let k = String::from(key.as_str().unwrap());
        let v = String::from(val.as_str().unwrap());

        e.insert(k, v);
    }

    e
}


fn get_process_network(net: &Yaml) -> models::ProcessNetwork {
    let mut pm = models::PortMapping::new();

    let ports = net["ports"].as_vec().unwrap();

    for p in ports {

        let hp = String::from(p["hostPort"].as_str().unwrap());
        let pp = String::from(p["procPort"].as_str().unwrap());

        println!("hp: {}, pp: {}", hp, pp);

        let pmap = models::PortMap{
            host_port: hp,
            proc_port: pp
        };
        
        pm.push(pmap);
    }

    models::ProcessNetwork{
        pm
    }
}


pub fn parse(yaml_path: &str) 
    -> Result<models::Process, Box<dyn std::error::Error>> {

    let contents = fs::read_to_string(yaml_path)?;
    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = &docs[0];


    let process = models::Process{
        id: String::from(doc["id"].as_str().unwrap()),
        name: String::from(doc["name"].as_str().unwrap()),
        pid: doc["pid"].as_i64().unwrap() as u8,
        context_dir: String::from(doc["name"].as_str().unwrap()), 
        image: get_image_struct(&doc["image"]),
        job: get_process_job_struct(&doc["job"]),
        env: get_process_env(&doc["env"]),
        network: get_process_network(&doc["network"]),
    };

    Ok(process)
    
}

