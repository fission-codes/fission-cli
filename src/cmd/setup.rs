use anyhow::Result;
use clap::Args;
use std::process::Command;
use crate::legacy::prepare_args;

#[derive(Args)]
struct Setup {}

pub fn run_command(
    username: Option<String>,
    email: Option<String>,
    keyfile: Option<String>,
    os: Option<String>,
) -> Result<()> {
    let args = prepare_args(&[
        ("-u", username.as_ref()),
        ("-e", email.as_ref()),
        ("-k", keyfile.as_ref()),
        ("os", os.as_ref()),
    ]);

    Command::new("fission")
        .arg("setup")
        .args(args)
        .spawn()?
        .wait()?;

    Ok(())
}
