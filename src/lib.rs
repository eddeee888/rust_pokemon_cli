use anyhow::{anyhow, Result};
use log::info;
use reqwest::StatusCode;
use serde_json;
use std::fmt;

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

  match convert_data_to_pokemon(&data) {
    Ok(p) => return Ok(format!("{}", p)),
    Err(e) => return Err(anyhow!(e)),
  }
}

struct PokemonStats {
  hp: u32,
  attack: u32,
  defense: u32,
  special_attack: u32,
  special_defense: u32,
  speed: u32,
}
struct Pokemon {
  id: u32,
  name: String,
  types: Vec<String>,
  stats: PokemonStats,
}
impl std::fmt::Display for Pokemon {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write strictly the first element into the supplied output
    // stream: `f`. Returns `fmt::Result` which indicates whether the
    // operation succeeded or failed. Note that `write!` uses syntax which
    // is very similar to `println!`.

    write!(
      f,
      " ID: {}\n Name: {}\n Types: {}\n Stats:\n  HP: {}\n  Atk: {}\n  Def: {}\n  Sp.Atk: {}\n  Sp.Def: {}\n  Spd: {}\n",
      self.id,
      self.name,
      self.types.join(","),
      self.stats.hp,
      self.stats.attack,
      self.stats.defense,
      self.stats.special_attack,
      self.stats.special_defense,
      self.stats.speed
    )
  }
}

fn convert_data_to_pokemon(data: &serde_json::Value) -> Result<Pokemon> {
  let stats = PokemonStats {
    hp: 48,
    attack: 48,
    defense: 48,
    special_attack: 48,
    special_defense: 48,
    speed: 48,
  };
  let mut types: Vec<String> = Vec::new();
  types.push("Normal".to_string());

  let pokemon = Pokemon {
    id: 148,
    name: "Ditto".to_string(),
    types: types,
    stats: stats,
  };

  Ok(pokemon)
}
