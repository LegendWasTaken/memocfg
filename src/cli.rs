use clap::Command;

pub fn cli() -> Command {
    Command::new("memocfg")
        .about("A linux service for managing users")
        .subcommand_required(true)
        .subcommand(
            Command::new("users")
                .about("User related commands")
                .subcommand(
                    Command::new("list")
                        .about("List currently registered users")
                )
                .subcommand(
                    Command::new("register")
                        .about("Register a new user to memocfg")
                )
        )
        .subcommand(
            Command::new("ports")
                .about("Port related commands")
                .subcommand(
                    Command::new("list")
                        .about("List current port range restrictions")
                )
                .subcommand(
                    Command::new("set")
                        .about("Set new port range restriction for the specified user")
                )
        )
        .subcommand(
            Command::new("projects")
                .about("Project related commands")
                .subcommand(
                    Command::new("list")
                        .about("List currently active projects")
                )
        )
}