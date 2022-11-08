use crate::legacy::{prepare_args, prepare_flags};
use anyhow::Result;
use clap::Args;
use std::{collections::HashMap, process::Command};

#[derive(Args)]
struct Setup {}

pub fn run_command(
    username: Option<String>,
    email: Option<String>,
    keyfile: Option<String>,
    os: Option<String>,
    verbose: bool,
    remote: Option<String>,
) -> Result<()> {
    let args = prepare_args(&HashMap::from([
        ("-u", username.as_ref()),
        ("-e", email.as_ref()),
        ("-k", keyfile.as_ref()),
        ("--os", os.as_ref()),
    ]));
    let remote = prepare_args(&HashMap::from([("-R", remote.as_ref())]));
    let flags = prepare_flags(&HashMap::from([("-v", verbose)]));

    Command::new("fission")
        .arg("setup")
        .args(args)
        .args(remote)
        .args(flags)
        .spawn()?
        .wait()?;

    Ok(())
}
