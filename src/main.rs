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
    #[clap(short, long, global = true, help = "Print detailed output")]
    verbose: bool,
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
        #[clap(from_global)]
        verbose: bool,
    },
    #[clap(about = "User application management")]
    User(User),

    // Shortcuts
    #[clap(about = "Display current user")]
    Whoami {
        #[clap(from_global)]
        verbose: bool,
    },
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::App(a) => match run_app_command(a) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute app command."),
        },
        Commands::Generate(g) => run_generate_command(g),
        Commands::Setup {
            username,
            email,
            keyfile,
            os,
            verbose,
        } => match run_setup_command(username, email, keyfile, os, verbose) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute setup command."),
        },
        Commands::User(u) => match run_user_command(u) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute user command.",),
        },

        // Shortcuts
        Commands::Whoami { verbose } => match run_user_command(User {
            command: UserCommands::Whoami { verbose },
        }) {
            Ok(()) => (),
            Err(_err) => eprintln!("💥 Failed to execute whoami command.",),
        },
    }
}
