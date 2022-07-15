use clap::{Args, Subcommand};

#[derive(Args)]
pub struct App {
    #[clap(subcommand)]
    command: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    #[clap(about = "Delegate capability to an audience DID")]
    Delegate,
    #[clap(about = "Upload the working directory")]
    Publish,
    #[clap(about = "Initialize an existing app")]
    Register,
}

pub fn run_command(a: App) -> () {
    match a.command {
        AppCommands::Delegate => {
            println!("Delegate")
        }
        AppCommands::Publish => {
            println!("Publish")
        }
        AppCommands::Register => {
            println!("Register")
        }
    }
}
