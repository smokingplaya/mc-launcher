use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadableObject {
  /// The SHA1 of the jar.
  sha1: String,
  /// The size of jar in bytes.
  size: usize,
  /// The URL where the jar is hosted.
  url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Downloads {
  /// The client.jar download information.
  client: Option<DownloadableObject>,
  /// The obfuscation maps for this client version. Added in Java Edition 19w36a but got included in 1.14.4 also.
  client_mappings: Option<DownloadableObject>,
  /// The server download information.
  server: Option<DownloadableObject>,
  /// The obfuscation maps for this server version. Added in Java Edition 19w36a but got included in 1.14.4 also.
  server_mappings: Option<DownloadableObject>
}