use crate::legacy::{prepare_args, prepare_flags};
use anyhow::Result;
use clap::Args;
use std::process::Command;

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
    let flags = prepare_flags(&[("-v", &verbose)]);
    let args = prepare_args(&[
        ("-u", username.as_ref()),
        ("-e", email.as_ref()),
        ("-k", keyfile.as_ref()),
        ("--os", os.as_ref()),
        ("-R", remote.as_ref()),
    ]);

    Command::new("fission")
        .arg("setup")
        .args(args)
        .args(flags)
        .spawn()?
        .wait()?;

    Ok(())
}
