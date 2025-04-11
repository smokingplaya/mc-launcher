use std::{path::Path, process::Command};
use crate::{java::error::{JavaError, JavaErrorKind}, utils::choice_by_os};
use super::error::Result;

/// https://www.tpointtech.com/java-9-new-version-string-scheme
#[derive(Debug, Clone)]
pub struct JavaVersion {
  major: u8,
  minor: u8,
  #[allow(unused)]
  security: u8,
}

macro_rules! get_item {
  ($i:ident, $j:expr) => {
    *$i.get($j).ok_or(JavaError::new(JavaErrorKind::OutputReadError))?
  }
}

impl JavaVersion {
  // TODO @ its looks weird
  pub fn get_verison(java_base_path: &Path) -> Result<Self> {
    let java_executable = java_base_path.join("bin")
      .join(choice_by_os("java", "java.exe"));

    let output = Command::new(java_executable)
      .arg("-version")
      .output()
      .map_err(|e| JavaError::new_with_details(JavaErrorKind::OutputReadError, e.to_string()))?;

    let version = String::from_utf8_lossy(&output.stderr)
      .to_string();

    let version_line = version
      .lines()
      .next()
      .ok_or(JavaError::new_with_details(JavaErrorKind::OutputReadError, String::from("No client line")))?;

    let version = version_line
      .split_whitespace()
      .find(|s| s.starts_with('"'))
      .ok_or(JavaError::new_with_details(JavaErrorKind::OutputReadError, String::from("Unable to get Java's version")))?
      .trim_matches('"')
      .split('.');

    let version_parts = version.collect::<Vec<&str>>()
      .iter()
      .map(|e| e.parse().unwrap_or_default())
      .collect::<Vec<u8>>();

    Ok(Self {
      major: get_item!(version_parts, 0),
      minor: get_item!(version_parts, 1),
      security: get_item!(version_parts, 2),
    })
  }

  /// 7, 8, 9, 11, 16, 17, 18, etc
  pub fn main_version(&self) -> u8 {
    if self.major == 1 {
      return self.minor;
    }

    self.major
  }
}