use anyhow::{anyhow, Result};
use log::info;
use reqwest::StatusCode;
use serde_json;

pub enum CommandEnum {
  FindByName,
}

pub fn get_command_enum(cmd: &str) -> Result<CommandEnum> {
  match cmd {
    "findByName" => Ok(CommandEnum::FindByName),
    _ => Err(anyhow!("invalid_command")),
  }
}

const BASE_ENDPOINT: &str = "https://pokeapi.co/api/v2/";

pub fn make_request(cmd_enum: CommandEnum, arg: &str) -> Result<String> {
  info!("Making request...");
  let endpoint = match cmd_enum {
    CommandEnum::FindByName => format!("{}pokemon/", BASE_ENDPOINT),
  };
  let endpoint = format!("{}{}", endpoint, arg);
  info!("=> Endpoint: {}", endpoint);

  let resp = reqwest::blocking::get(endpoint)?;
  info!("Unpacking response...");
  info!("=> Response status: {}", resp.status());

  match resp.status() {
    StatusCode::OK => {}
    StatusCode::NOT_FOUND => return Err(anyhow!("Pokemon not found: \"{}\"", arg)),
    _ => return Err(anyhow!("HTTP error: {}", resp.status())),
  };

  let data = resp.json::<serde_json::Value>()?;

  // TODO: format data
  Ok(format!("{}", data))
}
