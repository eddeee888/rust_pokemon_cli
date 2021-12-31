use anyhow::{anyhow, Result};
use log::info;
use reqwest::StatusCode;
use std::collections::HashMap;

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

#[tokio::main]
pub async fn make_request(cmd_enum: CommandEnum, arg: &str) -> Result<&str> {
  info!("Making request...");
  let endpoint = match cmd_enum {
    CommandEnum::FindByName => format!("{}pokemon/", BASE_ENDPOINT),
  };
  let endpoint = format!("{}{}", endpoint, arg);
  info!("=> Endpoint: {}", endpoint);

  let resp = reqwest::get(endpoint).await?;
  info!("Unpacking response...");
  info!("=> Response status: {}", resp.status());

  match resp.status() {
    StatusCode::OK => {}
    StatusCode::NOT_FOUND => return Err(anyhow!("Pokemon not found: \"{}\"", arg)),
    _ => return Err(anyhow!("HTTP error: {}", resp.status())),
  };

  // Parse the response body as Json in this case
  let data = resp.json::<HashMap<String, String>>().await?;
  info!("=> Response data: {:?}", data);

  Ok("")
}
