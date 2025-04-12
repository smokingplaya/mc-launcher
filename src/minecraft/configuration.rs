use std::io;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::client::ClientFile;
use crate::java::Java;

#[derive(Debug, Clone)]
pub struct WindowConfiguration {
  /// Width of the window
  pub width: Option<usize>,
  /// Height of the window
  pub height: Option<usize>,
  /// Is window going to be in fullscreen mode?
  pub fullscreen: Option<bool>,
}

impl Default for WindowConfiguration {
  fn default() -> Self {
    Self {
      width: None,
      height: None,
      fullscreen: Some(true),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinecraftSessionUserType {
  #[serde(rename = "mojang")]
  Mojang,
  #[serde(rename = "legacy")]
  Legacy
}

impl Default for MinecraftSessionUserType {
  fn default() -> Self {
    Self::Mojang
  }
}

#[derive(Debug, Clone)]
pub struct AuthLibConfiguration {
  /// Url of AuthLib server
  pub server: String,
  /// Version of authlib-injector
  /// 1.2.5 by default
  pub version: String
}

impl Default for AuthLibConfiguration {
  fn default() -> Self {
    Self {
      server: String::new(),
      version: String::from("1.2.5")
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct MinecraftSession {
  /// Username of user
  pub username: String,
  /// User type (mojang/legacy)
  pub user_type: Option<MinecraftSessionUserType>,
  /// UUID
  pub uuid: String,
  /// Access Token
  pub access_token: String,
  /// Adds support for authlib-injector.\
  /// This field is responsible for a reference to the server that will be used for authorization, etc.
  pub authlib_server: Option<AuthLibConfiguration>
}

#[derive(Debug, Clone, Default)]
pub struct MinecraftClient {
  /// Path to the .minecraft folder
  pub path: PathBuf,
  /// Folder name of a version, that you're going to run
  pub version: String,
  /// Server IP of the server to which the player will be connected after the game starts (does not work on versions higher than 1.20)
  pub server: Option<String>,
}

impl MinecraftClient {
  pub fn get_client_file(&self) -> io::Result<PathBuf> {
    Ok(
      self.path // Client Folder
        .join("versions") // Client Folder/versions/
        .join(self.version.clone()) // Client Folder/versions/Forge 1.12.2/
        .join("client.json") // Client Folder/versions/Forge 1.12.2/client.json
        .to_path_buf()
    )
  }

  pub fn get_client_info(&self) -> io::Result<ClientFile> {
    let path = self.get_client_file()?;

    ClientFile::new(path)
  }
}

#[derive(Debug, Clone)]
pub struct MinecraftConfiguration {
  /// Information about client that you want to run
  pub client: MinecraftClient,
  /// By default, library will be search java on PC and use it\
  /// But you can override it, and use custom Java.
  pub java: Option<Java>,
  /// Minecraft Session information\
  /// Settings: ``Player's Username``, ``UUID``, ``Access Token``
  pub session: MinecraftSession,
  /// Minecraft window settings\
  /// Settings: ``Size of window``, ``Fullscreen mode``
  pub window: WindowConfiguration,
}