use clap::{Args, Subcommand};
use did_key::{Ed25519KeyPair, KeyMaterial, DIDCore, Config};
use colored::Colorize;

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
            let keys = did_key::generate::<Ed25519KeyPair>(None);

            println!("{}", "✅ Generated an Ed25519 key pair and associated DID".bright_green());
            println!("🗝️  Private key: {}", base64::encode(keys.private_key_bytes().as_slice()).bright_blue());
            println!("🔑 Public key: {}", base64::encode(keys.public_key_bytes().as_slice()).bright_blue());
            println!("🆔 DID: {}", keys.get_did_document(Config::default()).id.bright_blue());
        }
    }
}