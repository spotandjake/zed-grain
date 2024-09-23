use zed_extension_api::{self as zed, LanguageServerId, Result};

struct MyExtension {
  // TODO: Real State
  empty_state: Option<i8>,
}

fn get_path_to_language_server_executable() -> Result<String> {
  return Ok(String::from(
    "/Users/spotandjake/Library/Caches/fnm_multishells/3219_1677859702535/bin/grain",
  ));
}

impl zed::Extension for MyExtension {
  fn new() -> Self {
    Self { empty_state: None }
  }

  fn language_server_command(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<zed::Command> {
    Ok(zed::Command {
      command: get_path_to_language_server_executable()?,
      args: vec![String::from("lsp")],
      env: vec![],
    })
  }
}

zed::register_extension!(MyExtension);
