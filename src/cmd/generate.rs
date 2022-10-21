use clap::{Args, Subcommand};
use ed25519_zebra::*;
use rand::thread_rng;
use colored::Colorize;

#[derive(Args)]
pub struct Generate {
    #[clap(subcommand)]
    command: GenerateCommands,
}

const DID_KEY_PREFIX:&str = "did:key:";
const DID_KEY_PREFIX_ED25519:&str = "z6M";

#[derive(Subcommand)]
pub enum GenerateCommands {
    #[clap(about = "Generate an Ed25519 key pair and an associated DID")]
    Credentials,
}
pub fn run_command(g: Generate) {
    match g.command {
        GenerateCommands::Credentials => {
            let private_key = SigningKey::new(thread_rng());
            let public_key = VerificationKey::from(&private_key);
            
            let private_key_bytes: [u8; 32] = private_key.into();
            let public_key_bytes: [u8; 32] = public_key.into();

            let did_data = bs58::encode(&public_key_bytes).into_string();

            let did = format!("{}{}{}", DID_KEY_PREFIX, DID_KEY_PREFIX_ED25519, did_data);

            println!("{}", "✅ Generated an Ed25519 key pair and associated DID".bright_green());
            println!("🗝️  Private key: {}", base64::encode(&private_key_bytes).bright_blue());
            println!("🔑 Public key: {}", base64::encode(&public_key_bytes).bright_blue());
            println!("🆔 DID: {}", did.bright_blue());
        }
    }
}
