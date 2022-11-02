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
    Register {
        #[clap(
            short,
            long = "app-dir",
            help = "The file path to initialize the app in (app config, etc.)",
            default_value = ".",
            value_name = "PATH"
        )]
        app_dir: String,
        #[clap(
            short,
            long = "build-dir",
            help = "The file path of the assets or directory to sync",
            value_name = "PATH"
        )]
        build_dir: Option<String>,
        #[clap(short, long = "name", help = "Optional app name")]
        name: Option<String>,
        #[clap(
            long = "ipfs-bin",
            help = "Path to IPFS binary [default: `which ipfs`]",
            value_name = "BIN_PATH"
        )]
        ipfs_bin: Option<String>,
        #[clap(
            long = "ipfs-timeout",
            help = "IPFS timeout",
            default_value = "1800",
            value_name = "SECONDS"
        )]
        ipfs_timeout: String,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Potency {
    Append,
    Destroy,
    SuperUser,
}
pub fn run_command(a: App) -> Result<()> {
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
        AppCommands::Register {
            app_dir,
            build_dir,
            name,
            ipfs_bin,
            ipfs_timeout,
        } => {
            let args = prepare_args(&[
                ("-a", Some(app_dir).as_ref()),
                ("-b", build_dir.as_ref()),
                ("-n", name.as_ref()),
                ("--ipfs-bin", ipfs_bin.as_ref()),
                ("--ipfs-timeout", Some(ipfs_timeout).as_ref()),
            ]);

            Command::new("fission")
                .args(["app", "register"])
                .args(args)
                .spawn()?
                .wait()?;

            Ok(())
        }
    }
}
