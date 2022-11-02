use std::iter::once;

/// Convert argument tuples to a vector of argument strings.
/// 
/// This function is glue to reformat args parsed by CLAP for
/// Command::args (https://doc.rust-lang.org/std/process/struct.Command.html#method.args).
/// 
/// It keeps optional arguments CLAP parsed as Some(arg) and drops None arguments.
pub fn prepare_args(args: &[(&str, Option<&String>)]) -> Vec<String> {
  args.iter()
      .filter(|tup| tup.1.is_some())
      .flat_map(|tup| once(tup.0.to_string()).chain(once(tup.1.unwrap().to_string())))
      .collect()
}