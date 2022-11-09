use std::{collections::HashMap, iter::once};

/// Convert flags to a vector of flag strings
///
/// This function is glue to reformat flags parsed by CLAP for
/// Command::args (https://doc.rust-lang.org/std/process/struct.Command.html#method.args).
///
/// It keeps flags CLAP parsed as True and drops False flags.
pub fn prepare_flags(flags: &HashMap<&str, bool>) -> Vec<String> {
    flags
        .iter()
        .filter(|tup| *tup.1)
        .flat_map(|tup| once(tup.0.to_string()))
        .collect()
}

/// Convert arguments to a vector of argument strings.
///
/// This function is glue to reformat args parsed by CLAP for
/// Command::args (https://doc.rust-lang.org/std/process/struct.Command.html#method.args).
///
/// It keeps optional arguments CLAP parsed as Some(arg) and drops None arguments.
pub fn prepare_args(args: &HashMap<&str, Option<&String>>) -> Vec<String> {
    args.iter()
        .filter(|tup| tup.1.is_some())
        .flat_map(|tup| once(tup.0.to_string()).chain(once(tup.1.unwrap().to_string())))
        .collect()
}
