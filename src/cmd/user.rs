use crate::legacy::{prepare_args, prepare_flags};
use anyhow::Result;
use clap::{Args, Subcommand};
use std::process::Command;

#[derive(Args)]
pub struct User {
    #[clap(subcommand)]
    pub command: UserCommands,
}

#[derive(Subcommand)]
pub enum UserCommands {
    #[clap(about = "Log in to an existing account")]
    Login {
        #[clap(short, long, value_parser, help = "Username")]
        username: Option<String>,
        #[clap(from_global)]
        verbose: bool,
    },
    #[clap(about = "Display current user")]
    Whoami {
        #[clap(from_global)]
        verbose: bool,
    },
}

pub fn run_command(u: User) -> Result<()> {
    match u.command {
        UserCommands::Login { username, verbose } => {
            let flags = prepare_flags(&[("-v", &verbose)]);
            let args = prepare_args(&[("-u", username.as_ref())]);

            Command::new("fission")
                .args(["user", "login"])
                .args(args)
                .args(flags)
                .spawn()?
                .wait()?;

            Ok(())
        }
        UserCommands::Whoami { verbose } => {
            let flags = prepare_flags(&[("-v", &verbose)]);

            Command::new("fission")
                .args(["user", "whoami"])
                .args(flags)
                .spawn()?
                .wait()?;

            Ok(())
        }
    }
}
