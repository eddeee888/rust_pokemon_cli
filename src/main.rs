use anyhow::{Context, Result};
use log::info;
use std::string::String;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  /// Command
  command: String,
  /// Search term
  arg: String,
}

fn main() -> Result<()> {
  env_logger::init();

  info!("Start!");
  info!("Parsing args...");
  let args = Cli::from_args();
  info!(
    "=> {}",
    format!("Command: \"{}\", Arg: \"{}\"", &args.command, &args.arg)
  );

  let cmd_enum = rust_pokemon_cli::get_command_enum(&args.command)
    .with_context(|| format!("Invalid command: {}", &args.command))?;

  match rust_pokemon_cli::make_request(cmd_enum, &args.arg) {
    Ok(res) => println!("Result: {}", res),
    Err(err) => println!("Error: {}", err),
  }

  info!("End!");

  Ok(())
}
