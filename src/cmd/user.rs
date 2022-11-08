use crate::legacy::{prepare_args, prepare_flags};
use anyhow::Result;
use clap::{Args, Subcommand};
use std::{collections::HashMap, process::Command};

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
        #[clap(from_global)]
        remote: Option<String>,
    },
    #[clap(about = "Display current user")]
    Whoami {
        #[clap(from_global)]
        verbose: bool,
        #[clap(from_global)]
        remote: Option<String>,
    },
}

pub fn run_command(u: User) -> Result<()> {
    match u.command {
        UserCommands::Login {
            username,
            verbose,
            remote,
        } => {
            let args = prepare_args(&HashMap::from([("-u", username.as_ref())]));
            let remote = prepare_args(&HashMap::from([("-R", remote.as_ref())]));
            let flags = prepare_flags(&HashMap::from([("-v", verbose)]));

            Command::new("fission")
                .args(["user", "login"])
                .args(args)
                .args(remote)
                .args(flags)
                .spawn()?
                .wait()?;

            Ok(())
        }
        UserCommands::Whoami { verbose, remote } => {
            let remote = prepare_args(&HashMap::from([("-R", remote.as_ref())]));
            let flags = prepare_flags(&HashMap::from([("-v", verbose)]));

            Command::new("fission")
                .args(["user", "whoami"])
                .args(remote)
                .args(flags)
                .spawn()?
                .wait()?;

            Ok(())
        }
    }
}
