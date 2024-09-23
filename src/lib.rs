// TODO: Bring in line with vscode extension
use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct GrainLspBinary {
  path: String,
  args: Option<Vec<String>>,
}

struct GrainExtension {
  cached_binary_path: Option<String>,
}

const REPO: &'static str = "grain-lang/grain";

const GRAIN_BINARIES: &'static [&'static str] =
  &["grain", "grain-mac-x64", "grain-linux-x64", "grain-win-x64"];
fn search_for_binary(worktree: &zed::Worktree) -> Option<String> {
  for binary_name in GRAIN_BINARIES.iter() {
    if let Some(path) = worktree.which(binary_name) {
      return Some(path);
    }
  }
  return None;
}

impl GrainExtension {
  fn language_server_binary_path(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<GrainLspBinary> {
    // Try to fetch the settings for grain
    let binary_settings = LspSettings::for_worktree("grain", worktree)
      .ok()
      .and_then(|lsp_settings| lsp_settings.binary);
    // Look for the binary argument settings
    let binary_args = binary_settings
      .as_ref()
      .and_then(|binary_settings| binary_settings.arguments.clone());
    // If we have the settings lets look for if a binary path is specified
    if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
      return Ok(GrainLspBinary {
        path,
        args: binary_args,
      });
    }

    // If we don't have the settings, let's try to find it in the worktree
    if let Some(path) = search_for_binary(worktree) {
      return Ok(GrainLspBinary {
        path,
        args: binary_args,
      });
    }

    // Check our cache
    if let Some(path) = &self.cached_binary_path {
      if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
        return Ok(GrainLspBinary {
          path: path.clone(),
          args: binary_args,
        });
      }
    }

    // If we still haven't found the binary, let's try to install it
    zed::set_language_server_installation_status(
      language_server_id,
      &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );

    // Get Latest Release for version number
    let release = zed::latest_github_release(
      REPO,
      zed::GithubReleaseOptions {
        require_assets: true,
        pre_release: false,
      },
    )?;
    let release = match release.version.starts_with("grain") {
      true => release,
      false => {
        let release_name = release.version.split("-");
        match release_name.last() {
          Some(version) => {
            zed::github_release_by_tag_name(REPO, format!("grain-{version}").as_str())?
          }
          None => release,
        }
      }
    };

    let (platform, arch) = zed::current_platform();
    let asset_name = format!(
      "grain-{os}-{arch}{extension}",
      os = match platform {
        zed::Os::Mac => "mac",
        zed::Os::Linux => "linux",
        zed::Os::Windows => "win",
      },
      arch = match arch {
        zed::Architecture::Aarch64 => "x64", // we use the x64 binary even on arm for now
        zed::Architecture::X86 => "x86",
        zed::Architecture::X8664 => "x64",
      },
      extension = match platform {
        zed::Os::Mac | zed::Os::Linux => "",
        zed::Os::Windows => ".exe",
      }
    );

    let asset = release
      .assets
      .iter()
      .find(|asset| asset.name == asset_name)
      .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

    if !fs::metadata(&asset_name).map_or(false, |stat| stat.is_file()) {
      zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
      );
      zed::download_file(
        &asset.download_url,
        &asset_name,
        zed::DownloadedFileType::Uncompressed,
      )
      .map_err(|e| format!("failed to download file: {e}"))?;
    }

    self.cached_binary_path = Some(asset_name.clone());
    Ok(GrainLspBinary {
      path: asset_name,
      args: binary_args,
    })
  }
}

impl zed::Extension for GrainExtension {
  fn new() -> Self {
    Self {
      cached_binary_path: None,
    }
  }

  fn language_server_command(
    &mut self,
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
  ) -> Result<zed::Command> {
    let grain_binary = self.language_server_binary_path(language_server_id, worktree)?;
    let mut args = match grain_binary.args {
      Some(args) => args,
      None => vec![],
    };
    args.insert(0, String::from("lsp"));
    Ok(zed::Command {
      command: grain_binary.path,
      args,
      env: Default::default(),
    })
  }
}

zed::register_extension!(GrainExtension);
