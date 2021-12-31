use anyhow::{anyhow, Result};

pub enum CommandEnum {
  FindByName,
}

pub fn get_command_enum(cmd: &String) -> Result<CommandEnum> {
  match cmd.as_str() {
    "findByName" => Ok(CommandEnum::FindByName),
    _ => Err(anyhow!("invalid_command")),
  }
}
