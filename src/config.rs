use std::{env, fs};
use std::path::{Path, PathBuf};
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
pub struct Project {
    pub nice_name: String,
    pub full_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub port_range: PortRange,
    pub projects: Vec<Project>,
}

impl User {
    pub fn get_group_name(self) -> String {
        format!("memocfg-{}", self.name)
    }

    pub fn new(name: String, port_range: PortRange) -> User {
        User {
            name: name.to_string(),
            port_range,
            projects: vec![],
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
        for user in &self.clone().users {
            println!("{} - {}", user.clone().name, user.clone().get_group_name());
        }
    }

    pub fn list_ports(&self) {
        for user in &self.clone().users {
            let range = user.port_range;
            let name = user.clone().name;
            println!("{}:{} - {}", range.start, range.end, name);
        }
    }

    pub fn list_projects(&self) {
        for user in &self.users {
            let name = user.clone().name;
            let projects = user.clone().projects;
            for project in projects {
                println!("{} - {}", name, project.nice_name);
            }
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

    pub fn update_port_range(&mut self, name: String, range: PortRange) {
        for user in &mut self.users {
            if user.name == name {
                user.port_range = range;
                break;
            }
        }
    }

    pub fn create_user(&mut self, name: String, range: PortRange) -> Result<(), ()> {
        for user in self.users.iter_mut() {
            if user.name == name {
                return Err(());
            }
        }

        self.users.push(User {
            name,
            port_range: range,
            projects: vec![]
        });
        Ok(())
    }
}
