use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetIndex {
  /// The assets version.
  pub id: String,
  /// The SHA1 of the assets file.
  pub sha1: String,
  /// The size of the version.
  pub size: usize,
  /// The total size of the version.
  #[serde(rename = "totalSize")]
  pub total_size: usize,
  /// Undocumented
  pub known: Option<bool>,
  /// The URL that the game should visit to download the assets.
  pub url: String
}