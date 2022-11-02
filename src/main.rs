use clap::{Parser, Subcommand};
use fission::cmd::{
    app::{run_command as run_app_command, App},
    generate::{run_command as run_generate_command, Generate},
    setup::run_command as run_setup_command,
    user::{run_command as run_user_command, User, UserCommands},
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
        #[clap(short, long, value_parser, help = "The email address for the account")]
        email: Option<String>,
        #[clap(
            short,
            long = "with-key",
            value_parser,
            help = "A root keyfile to import"
        )]
        keyfile: Option<String>,
        #[clap(short, long, value_parser, help = "Override OS detection")]
        os: Option<String>,
    },
    #[clap(about = "User application management")]
    User(User),

    // Shortcuts
    #[clap(about = "Display current user")]
    Whoami,
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::App(a) => run_app_command(a),
        Commands::Generate(g) => run_generate_command(g),
        Commands::Setup {
            username,
            email,
            keyfile,
            os,
        } => match run_setup_command(username, email, keyfile, os) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute setup command."),
        },
        Commands::User(u) => match run_user_command(u) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute user command.",),
        },

        // Shortcuts
        Commands::Whoami => match run_user_command(User {
            command: UserCommands::Whoami,
        }) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute whoami command.",),
        },
    }
}
