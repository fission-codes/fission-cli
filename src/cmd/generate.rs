use clap::{Args, Subcommand};

#[derive(Args)]
pub struct Generate {
    #[clap(subcommand)]
    command: GenerateCommands,
}

#[derive(Subcommand)]
pub enum GenerateCommands {
    #[clap(about = "Generate an Ed25519 key pair and an associated DID")]
    Credentials,
}
pub fn run_command(g: Generate) {
    match g.command {
        GenerateCommands::Credentials => {
            todo!("credentials")
        }
    }
}
