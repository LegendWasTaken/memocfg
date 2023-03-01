mod cli;

fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("register", _)) => {
                    println!("registering new user");
                }
                _ => {
                    println!("listing users");
                }
            }
        }
        Some(("ports", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("set", _)) => {
                    println!("setting new port range restrictions on user");
                }
                _ => {
                    println!("listing port range restrictions");
                }
            }
        }
        Some(("projects", _)) => {
            println!("displaying active projects");
        }
        _ => unreachable!(),
    }
}