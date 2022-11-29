pub mod file_management;
pub mod json;
pub mod math;

use anyhow::{anyhow, Result};
use std::io::{self, Write};
use std::process::Output;

pub struct OutputOptions {
    verbose: bool,
    quiet: bool,
    error: bool,
}

impl OutputOptions {
    fn verbose() -> OutputOptions {
        OutputOptions {
            verbose: true,
            quiet: false,
            error: false,
        }
    }

    fn default() -> OutputOptions {
        OutputOptions {
            verbose: false,
            quiet: false,
            error: false,
        }
    }

    fn quiet() -> OutputOptions {
        OutputOptions {
            verbose: false,
            quiet: true,
            error: false,
        }
    }
}

pub fn write_output(output: &Output) -> Result<()> {
    let mut options = OutputOptions::verbose();

    if output.stdout != [] {
        print_output(std::str::from_utf8(&output.stdout)?, &options)?;
    }
    if output.stderr != [] {
        options.error = true;
        print_output(std::str::from_utf8(&output.stdout)?, &options)?;
    }
    Ok(())
}

pub fn print_output(output: &str, options: &OutputOptions) -> Result<()> {
    println!("{}", output);
    Ok(())
}
