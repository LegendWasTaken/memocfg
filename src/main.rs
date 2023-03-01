use dotenv::dotenv;
use crate::config::{Config, PortRange, User};

mod cli;
mod config;

fn main() {
    dotenv().ok();

    let mut cfg = Config::load();

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("register", register_matches)) => {
                    let username  = register_matches.get_one::<String>("user").unwrap();
                    let port_range_string = register_matches.get_one::<String>("range").unwrap();
                    let port_range = PortRange::new(port_range_string);

                    cfg.users.push(User::new(username.clone(), port_range.clone()));
                    println!("Registered new user [{}] with port range [{}:{}]", username, port_range.start, port_range.end);
                }
                _ => {
                    cfg.list_users();
                }
            }
        }
        Some(("ports", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("set", register_matches)) => {
                    let username  = register_matches.get_one::<String>("user").unwrap();
                    let port_range_string = register_matches.get_one::<String>("range").unwrap();
                    let port_range = PortRange::new(port_range_string);

                    cfg.update_port_range(username.clone(), port_range.clone());
                    println!("Updated port range for user [{}] to [{}:{}]", username, port_range.start, port_range.end);
                }
                _ => {
                    cfg.list_ports();
                }
            }
        }
        Some(("projects", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("create", _register_matches)) => {

                }
                _ => {
                    cfg.list_projects();
                }
            }
        }
        _ => unreachable!(),
    }

    cfg.save();
}