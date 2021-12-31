use anyhow::{Context, Result};
use std::string::String;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  /// Command
  command: String,
  /// Search term
  term: String,
}

fn main() -> Result<()> {
  let args = Cli::from_args();

  let cmd_enum = rust_pokemon_cli::get_command_enum(&args.command)
    .with_context(|| format!("Invalid command: {}", &args.command))?;

  Ok(())
}
