use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientJavaVersion {
  component: String,
  #[serde(rename = "majorVersion")]
  major_version: usize
}