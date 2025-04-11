use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::client::downloads::DownloadableObject;
use crate::client::rules::Rule;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Natives {
  pub windows: Option<String>,
  pub macos: Option<String>,
  pub osx: Option<String>,
  pub linux: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryDownloads {
  pub artifact: Option<LibraryArtifact>,
  pub classifiers: Option<HashMap<String, DownloadableObject>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryArtifact {
  pub path: Option<String>,
  pub sha1: String,
  pub size: usize,
  pub url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
  /// A maven name for the library, in the form of "groupId:artifactId:version".
  pub name: String,
  /// The URL of the Maven repository (used by Forge).
  pub url: Option<String>,
  /// Information about native libraries (in C) bundled with this library. Appears only when there are classifiers for natives.
  pub natives: Option<Natives>,
  /// Shows what to exclude from the extraction.
  pub extract: Option<HashMap<String, Vec<String>>>,
  /// Contains a compound with the tags "action" and "os", as shown above.
  pub rules: Option<Vec<Rule>>,
  pub downloads: Option<LibraryDownloads>
}

impl Library {
  pub fn to_path(&self) -> PathBuf {
    let parts: Vec<&str> = self.name.split(':').collect();
    let subparts: Vec<&str> = parts[0].split('.').collect();
    let joined_subparts = subparts.join(std::path::MAIN_SEPARATOR_STR);

    Path::new(&joined_subparts)
      .join(parts[1])
      .join(parts[2])
      .join(format!("{}-{}.jar", parts[1], parts[2]))
  }
}