use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub aws_user_id: String,
    pub aws_profile: String,
}
const CONFIG_FILE_NAME: &str = "otis_config.json";

pub fn read() -> Config {
    let contents = fs::read_to_string(CONFIG_FILE_NAME).expect("Could not read the config file");

    let config: Config = serde_json::from_str(&contents).unwrap();

    config
}

pub fn exists() -> bool {
    Path::new(CONFIG_FILE_NAME).exists()
}

pub fn create(config_values: Config) {
    let path = Path::new(CONFIG_FILE_NAME);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let config = Config {
        aws_user_id: config_values.aws_user_id,
        aws_profile: config_values.aws_profile,
    };

    let data = serde_json::to_string(&config).unwrap();

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("config file successfully created"),
    }
}
