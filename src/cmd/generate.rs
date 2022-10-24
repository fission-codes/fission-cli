use clap::{Args, Subcommand};
use ed25519_zebra::*;
use rand::thread_rng;
use colored::Colorize;

#[derive(Args)]
pub struct Generate {
    #[clap(subcommand)]
    command: GenerateCommands,
}

const DID_KEY_PREFIX:&str = "did:key:z";
const DID_KEY_PREFIX_ED25519:[u8; 2] = [237, 1];

#[derive(Subcommand)]
pub enum GenerateCommands {
    #[clap(about = "Generate an Ed25519 key pair and an associated DID")]
    Credentials,
}
pub fn run_command(g: Generate) {
    match g.command {
        GenerateCommands::Credentials => {
            let private_key = SigningKey::new(thread_rng());

            let (private_key_str, public_key_str, did) = generate_key_strs(private_key);

            println!("{}", "✅ Generated an Ed25519 key pair and associated DID".bright_green());
            println!("🗝️  Private key: {}", private_key_str.bright_blue());
            println!("🔑 Public key: {}", public_key_str.bright_blue());
            println!("🆔 DID: {}", did.bright_blue());
        }
    }
}

fn generate_key_strs(private_key: SigningKey) -> (String, String, String){
    let public_key = VerificationKey::from(&private_key);

    let private_key_bytes: [u8; 32] = private_key.into();
    let public_key_bytes: [u8; 32] = public_key.into();
    
    let mut did_bytes:Vec<u8> = vec![];
    for byte in DID_KEY_PREFIX_ED25519{
        did_bytes.push(byte);
    }
    for byte in public_key_bytes{
        did_bytes.push(byte);
    }

    let did_data = bs58::encode(did_bytes.as_slice()).into_string();

    let did = format!("{}{}", DID_KEY_PREFIX, did_data);
    return (base64::encode(&private_key_bytes), base64::encode(&public_key_bytes), did);
}

#[cfg(test)]
mod cred_tests {
    use ed25519_zebra::SigningKey;
    use crate::cmd::generate::generate_key_strs;

    #[test]
    fn can_create_did() {
        let example_b64 = "4WJzyjS9iGmWJTvWbLA3RbzuwrR23Cp1RhddLAiQiPE=";
        let example_bytes:[u8; 32] = match base64::decode(example_b64){
            Ok(bytes) => bytes.try_into().unwrap(),
            Err(_) => panic!("😱 HELP THERE WAS A DECODE ERROR!!!!")
        };
        
        let private_key = SigningKey::from(example_bytes);
        let (private, public, did) = generate_key_strs(private_key);
        
        assert!(private == "4WJzyjS9iGmWJTvWbLA3RbzuwrR23Cp1RhddLAiQiPE=");
        assert!(public == "JOO9pzY2IJLnBalBwX+KHOa2cMYyvRnXnEFEbs6drvU=");
        assert!(did == "did:key:z6MkgwG8wdXCoG22CwsAKNxwVXnnYgckHsHhapWVvwbAe63v");
    }
}