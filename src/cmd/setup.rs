use clap::Args;

#[derive(Args)]
struct Setup {}

pub fn run_command(username: Option<String>) {
    todo!("setup --username={:?}", username)
}
