use anyhow::{anyhow, Context, Result};
use std::string::String;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  /// The pattern to look for
  command: String,
  /// The path to the file to read
  term: String,
}

const BASE_ENDPOINT: &str = "https://pokeapi.co/api/v2/";

fn main() -> Result<()> {
  let args = Cli::from_args();

  let cmd = get_endpoint_by_command(&args.command)
    .with_context(|| format!("Invalid command: {}", &args.command))?;

  println!("{}", cmd);
  println!("{}", &args.term);

  Ok(())
}

fn get_endpoint_by_command(cmd: &String) -> Result<String> {
  match cmd.as_str() {
    "findByName" => Ok(format!("{}pokemon/", BASE_ENDPOINT)),
    _ => Err(anyhow!("invalid_command")),
  }
}
