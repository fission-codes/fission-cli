use crate::legacy::{prepare_args, prepare_flags};
use anyhow::Result;
use clap::{ArgEnum, Args, Subcommand};
use std::process::Command;

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
        #[clap(
            help = "The file path of the assets or directory to sync",
            default_value = "./"
        )]
        path: String,
        #[clap(short, long, help = "Open your default browser after publish")]
        open: bool,
        #[clap(short, long, help = "Watch for changes & automatically trigger upload")]
        watch: bool,
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

        #[clap(
            long = "update-data",
            help = "Upload the data",
            default_value = "True",
            value_name = "ARG"
        )]
        update_data: String,
        #[clap(
            long = "udpate-dns",
            help = "Update DNS",
            default_value = "True",
            value_name = "ARG"
        )]
        update_dns: String,
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
        AppCommands::Publish {
            path,
            open,
            watch,
            ipfs_bin,
            ipfs_timeout,
            update_data,
            update_dns,
        } => {
            let flags = prepare_flags(&[
                ("-o", &open),
                ("-w", &watch)
            ]);

            let args = prepare_args(&[
                ("--ipfs-bin", ipfs_bin.as_ref()),
                ("--ipfs-timeout", Some(ipfs_timeout).as_ref()),
                ("--update-data", Some(update_data).as_ref()),
                ("--update-dns", Some(update_dns).as_ref()),
            ]);

            Command::new("fission")
                .args(["app", "publish"])
                .arg(path)
                .args(flags)
                .args(args)
                .spawn()?
                .wait()?;

            Ok(())
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
