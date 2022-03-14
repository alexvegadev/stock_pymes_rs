use std::{fs};

use log::info;
use yaml_rust::{YamlLoader, Yaml};

use crate::utils;

pub struct Config {
    conf: Vec<Yaml>
}

impl Config {

    pub fn new(path: &str) -> Self {
        let scope_config = utils::env::get_config_name_scope();
        let final_path = format!("{}/{}", path,scope_config);
        info!("Loading config: {}", scope_config);
        let contents = fs::read_to_string(final_path)
        .expect("Something went wrong reading the file");
        Self {conf: YamlLoader::load_from_str(&contents).unwrap()}
    }


    pub fn get_conf(&self, config_key: &str) -> &Yaml {
        &self.conf[0][config_key]
    }

    pub fn get_str(&self, config_root: &str, config_value: &str) -> &str {
        &self.conf[0][config_root][config_value].as_str().unwrap()
    }

    pub fn get_bool(&self, config_root: &str, config_value: &str) -> bool {
        self.conf[0][config_root][config_value].as_bool().unwrap()
    }

}