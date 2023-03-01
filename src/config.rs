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
    pub users: Vec<User>
}

impl Config {
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
        // Todo: Implementing loading from file
        Config {
            users: vec![User {
                name: String::from("User1"),
                port_range: PortRange {
                    start: 2001,
                    end: 2500,
                }
            }]
        }
    }

    pub fn save(self) {
        // Todo
    }
}
