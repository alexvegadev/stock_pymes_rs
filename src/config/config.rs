use std::fs;

use yaml_rust::{YamlLoader, Yaml};

pub struct Config {
    conf: Vec<Yaml>
}

impl Config {

    pub fn new(path: &str) -> Self {
        let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
        Self {conf: YamlLoader::load_from_str(&contents).unwrap()}
    }

    pub fn get_conf(&self, config_key: &str) -> &Yaml {
        &self.conf[0][config_key]
    }

}