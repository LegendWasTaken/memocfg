use std::{env, fs};
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

impl PortRange {
    pub fn new(str: &String) -> PortRange {
        let mut parts = str.split(":");
        PortRange {
            start: parts.next().unwrap().parse::<u16>().unwrap(),
            end: parts.next().unwrap().parse::<u16>().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub port_range: PortRange,
}

impl User {
    pub fn get_group_name(self) -> String {
        format!("memocfg-{}", self.name)
    }

    pub fn new(name: &String, port_range: PortRange) -> User {
        User {
            name: name.to_string(),
            port_range,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub users: Vec<User>,
}

impl Config {
    fn get_path() -> PathBuf {
        match env::var("memocfg_environment") {
            Ok(str) => {
                if str == "DEVELOPMENT" {
                    Path::new("memocfg.json").to_path_buf()
                } else {
                    Path::new("/var/lib/memocfg.json").to_path_buf()
                }
            }
            Err(_) => {
                // Default to assume that we're in a live environment
                Path::new("/var/lib/memocfg.json").to_path_buf()
            }
        }
    }

    fn load_internal() -> Option<Config> {
        let data = fs::read_to_string(Config::get_path()).ok()?;
        let cfg: Config = serde_json::from_str(data.as_str()).ok()?;
        Some(cfg)
    }

    pub fn list_users(&self) {
        println!("[Registered Users]");
        for user in &self.clone().users {
            println!("{} - {}", user.clone().name, user.clone().get_group_name());
        }
    }

    pub fn list_ports(&self) {
        println!("[Internal port ranges]");
        for user in &self.clone().users {
            let range = user.port_range;
            let name = user.clone().name;
            println!("{}:{} - {}", range.start, range.end, name);
        }
    }

    pub fn load() -> Config {
        match Config::load_internal() {
            None => {
                Config {
                    users: vec![]
                }
            }
            Some(cfg) => { cfg }
        }
    }

    pub fn save(self) {
        let as_json = serde_json::to_string(&self).unwrap();
        fs::write(Config::get_path(), as_json).expect("Failed to save config");
    }
}
