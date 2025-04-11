mod argument;
mod assetindex;
pub(crate) mod classpath;
mod downloads;
mod javaversion;
mod library;
mod rules;

pub(crate) trait CollectArguments {
  fn collect(&self) -> Vec<String>;
}

use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::client::argument::Arguments;
use crate::client::assetindex::AssetIndex;
use crate::client::downloads::Downloads;
use crate::client::javaversion::ClientJavaVersion;
use crate::client::library::Library;

/// Implementation of ``client.json`` files structure
///
/// Reference: https://minecraft.fandom.com/wiki/Client.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientFile {
  pub arguments: Arguments,
  #[serde(rename = "assetIndex")]
  pub asset_index: AssetIndex,
  pub assets: String,
  #[serde(rename = "complianceLevel")]
  /// Its value is 1 for all recent versions of the game (1.16.4 and above) or 0 for all others.\
  /// This tag tells the launcher whether it should urge the user to be careful since this version is older and might not support the latest player safety features.
  pub compliance_level: Option<u8>,
  pub downloads: Downloads,
  /// The name of this version client (e.g. 1.14.4).
  pub id: String,
  #[serde(rename = "javaVersion")]
  pub java_version: ClientJavaVersion,
  pub libraries: Vec<Library>,
  // logging: ,
  #[serde(rename = "mainClass")]
  pub main_class: String,
  #[serde(rename = "minimumLauncherVersion")]
  pub minimum_launcher_version: usize,
  #[serde(rename = "releaseTime")]
  pub release_time: String,
  pub time: String,
  pub r#type: String
}

impl ClientFile {
  pub fn new(version_file: PathBuf) -> io::Result<Self> {
    if !version_file.is_file() {
      return Err(Error::new(ErrorKind::NotFound, "Version file is not a file"));
    }

    let mut content = String::new();
    let mut file = File::open(version_file)?;
    file.read_to_string(&mut content)?;

    Ok(serde_json::from_str(&content)?)
  }
}