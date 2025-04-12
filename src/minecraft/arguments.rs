use std::collections::HashMap;
use crate::client::ClientFile;
use super::configuration::MinecraftConfiguration as Configuration;

pub type DataMap<'a> = HashMap<&'a str, String>;

pub fn generate_data_map<'a>(
  config: Configuration,
  client_file: ClientFile,
  class_path: String
) -> DataMap<'a> {
  let path = &config.client.path;
  let user_type = serde_json::to_string(&config.session.user_type.unwrap_or_default()).unwrap();
  let version = config.client.version;

  let mut map = HashMap::from([
    ("username", config.session.username.to_string()),
    ("uuid", config.session.uuid.to_string()),
    ("access_token", config.session.access_token.to_string()),
    ("user_type", user_type),
    ("launcher_name", String::from("java-minecraft-launcher")),
    ("launcher_version", String::from("1.6.84-j")),
    ("libraries_dir", path.join("libraries").to_str().unwrap().to_string()),
    ("natives_dir", path.join("versions").join(&version).join("natives").to_str().unwrap().to_string()),
    ("game_dir", path.to_str().unwrap().to_string()),
    ("assets_dir", path.join("assets").to_str().unwrap().to_string()),
    ("class_path", class_path),
    ("main_class", client_file.main_class),
    ("version_name", client_file.id),
    ("version_type", client_file.r#type),
    ("assets_index", client_file.assets),
    ("version_jar", path.join("versions").join(&version).join("client.jar").to_str().unwrap().to_string())
  ]);

  if let Some(ip) = config.client.server {
    map.insert("server_ip", ip.to_string());
  }

  if let Some(width) = config.window.width {
    map.insert("width", width.to_string());
  }

  if let Some(height) = config.window.height {
    map.insert("height", height.to_string());
  }

  if let Some(authlib) = config.session.authlib_server {
    map.insert("authlib_server", authlib.server.to_string());
  }

  map
}