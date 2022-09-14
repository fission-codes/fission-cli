use clap::{Parser, Subcommand};
use fission::cmd::{
    app::{run_command as run_app_command, App},
    generate::{run_command as run_generate_command, Generate},
    setup::run_command as run_setup_command,
    user::{run_command as run_user_command, User},
};

#[derive(Parser)]
#[clap(author, version, about="Fission makes developing, deploying, updating, and iterating on web apps quick and easy.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "User application management")]
    App(App),
    #[clap(about = "Generate key pairs and DIDs")]
    Generate(Generate),
    #[clap(about = "Initial Fission setup")]
    Setup {
        #[clap(short, long, value_parser, help = "The username to register")]
        username: Option<String>,
    },
    #[clap(about = "User application management")]
    User(User),
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::App(a) => run_app_command(a),
        Commands::Generate(g) => run_generate_command(g),
        Commands::Setup { username } => run_setup_command(username),
        Commands::User(u) => run_user_command(u),
    }
}
