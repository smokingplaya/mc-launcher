use std::{path::{Path, PathBuf}, process::Command};
use crate::java::error::{JavaError, JavaErrorKind, Result};
use super::OsAbstraction;

pub(crate) trait ProgramPathFinder {
  fn find_path(name: &str) -> Result<PathBuf>;
}

impl ProgramPathFinder for OsAbstraction {
  #[cfg(target_os = "windows")]
  fn find_path(name: &str) -> Result<PathBuf> {
    let output = Command::new("where")
      .arg(name)
      .output()
      .map_err(|e| JavaError::new_with_details(JavaErrorKind::OutputReadError, e.to_string()))?;

    let path = OsAbstraction::read_output(&output);

    Ok(
      Path::new(path.trim()) // windows moment
        .to_path_buf()
    )
  }

  #[cfg(not(target_os = "windows"))]
  fn find_path(name: &str) -> Result<PathBuf> {
    let output = Command::new("which")
      .arg(name)
      .output()
      .map_err(|e| JavaError::new_with_details(JavaErrorKind::OutputReadError, format!("Command not found: {e}")))?;

    let path = OsAbstraction::read_output(&output);

    Ok(
      Path::new(&path)
        .to_path_buf()
    )
  }
}