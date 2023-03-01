use clap::{Arg, ArgAction, Command};

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
                        .arg(
                            Arg::new("user")
                                .action(ArgAction::Set)
                                .required(true)
                                .num_args(1)
                        )
                        .arg(
                            Arg::new("range")
                                .long("range")
                                .action(ArgAction::Set)
                                .required(true)
                                .num_args(1)
                        )
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
                        .arg(
                            Arg::new("user")
                                .action(ArgAction::Set)
                                .required(true)
                                .num_args(1)
                        )
                        .arg(
                            Arg::new("range")
                                .long("range")
                                .action(ArgAction::Set)
                                .required(true)
                                .num_args(1)
                        )
                )
        )
        .subcommand(
            Command::new("projects")
                .about("Project related commands")
                .subcommand(
                    Command::new("list")
                        .about("List currently active projects")
                )
                .subcommand(
                    Command::new("create")
                        .about("Set new port range restriction for the specified user")
                        .arg(
                            Arg::new("project_name")
                                .action(ArgAction::Set)
                                .required(true)
                                .num_args(1)
                        )
                )
        )
}