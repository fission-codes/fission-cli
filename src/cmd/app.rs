use clap::{ArgEnum, Args, Subcommand};

#[derive(Args)]
pub struct App {
    #[clap(subcommand)]
    command: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    #[clap(about = "Delegate capability to an audience DID")]
    Delegate {
        #[clap(short, long, value_parser, help = "The target app")]
        app_name: Option<String>,
        #[clap(short, long, value_parser, help = "An audience DID")]
        did: Option<String>,
        #[clap(
            short,
            long,
            arg_enum,
            value_parser,
            default_value = "append",
            help = "The potency to delegate"
        )]
        potency: Potency,
        #[clap(
            short,
            long,
            value_parser,
            default_value_t = 300,
            help = "Lifetime in seconds before UCAN expires"
        )]
        lifetime: u16,
        #[clap(short, long, help = "Only output the UCAN on success")]
        quiet: bool,
    },
    #[clap(about = "Upload the working directory")]
    Publish {
        #[clap(short, long, help = "Open your default browser after publish")]
        open: bool,
        #[clap(short, long, help = "Watch for changes & automatically trigger upload")]
        watch: bool,
    },
    #[clap(about = "Initialize an existing app")]
    Register,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Potency {
    Append,
    Destroy,
    SuperUser,
}
pub fn run_command(a: App) {
    match a.command {
        AppCommands::Delegate {
            app_name: _,
            did: _,
            potency: _,
            lifetime: _,
            quiet: _,
        } => {
            todo!("delegate")
        }
        AppCommands::Publish { open: _, watch: _ } => {
            todo!("publish")
        }
        AppCommands::Register => {
            todo!("register")
        }
    }
}
