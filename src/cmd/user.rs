use clap::{Args, Subcommand};

#[derive(Args)]
pub struct User {
    #[clap(subcommand)]
    command: UserCommands,
}

#[derive(Subcommand)]
pub enum UserCommands {
    #[clap(about = "Log in to an existing account")]
    Login {
        #[clap(short, long, value_parser, help = "Username")]
        username: Option<String>,
    },
    #[clap(about = "Display current user")]
    Whoami,
}

pub fn run_command(u: User) {
    match u.command {
        UserCommands::Login { username: _ } => {
            todo!("login")
        }
        UserCommands::Whoami => {
            todo!("whoami")
        }
    }
}
