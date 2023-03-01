use crate::config::{Config, PortRange, User};

mod cli;
mod config;

fn main() {
    let mut cfg = Config::load();

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("register", register_matches)) => {
                    let username  = register_matches.get_one::<String>("user").unwrap();
                    let port_range_string = register_matches.get_one::<String>("range").unwrap();

                    cfg.users.push(User::new(username, PortRange::new(port_range_string)));
                }
                _ => {
                    cfg.list_users();
                }
            }
        }
        Some(("ports", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("set", _)) => {
                    println!("setting new port range restrictions on user");
                }
                _ => {
                    cfg.list_ports();
                }
            }
        }
        Some(("projects", _)) => {
            println!("displaying active projects");
        }
        _ => unreachable!(),
    }
}