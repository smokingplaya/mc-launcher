use std::path::{Path, PathBuf};
use error::{JavaError, JavaErrorKind, Result};
use version::JavaVersion;
use crate::{os::{process::ProgramPathFinder, OsAbstraction}, utils::choice_by_os};

pub mod error;
pub mod version;

pub const JAVA_SEPARATOR: &str = if cfg!(target_os = "windows") { ";" } else { ":" };

#[derive(Debug, Clone)]
pub struct Java {
  base_path: PathBuf,
  version: JavaVersion
}

impl TryFrom<String> for Java {
  type Error = JavaError;

  fn try_from(value: String) -> Result<Self> {
    Self::new(Path::new(&value).to_path_buf())
  }
}

impl Java {
  pub fn new(base_path: PathBuf) -> Result<Self> {
    if !base_path.is_dir() {
      return Err(JavaError::new(JavaErrorKind::NotFound));
    }

    let version = JavaVersion::get_verison(&base_path)?;

    Ok(Self {
      base_path,
      version
    })
  }

  /// Trying to find Java on the PC
  pub fn find() -> Result<Self> {
    // if JAVA_HOME provided
    if let Ok(path) = std::env::var("JAVA_HOME") {
      return Java::try_from(path);
    }

    // if not - we tryna find java in system

    // caching error instance
    let java_err = JavaError::new(JavaErrorKind::InvalidJavaFolderStructure);

    let path = OsAbstraction::find_path("java")?;

    let parent = path.ancestors()
      .next().ok_or(java_err.clone())?
      .file_name().ok_or(java_err.clone())?
      .to_str().ok_or(java_err.clone())?;

    if parent != "bin" {
      return Err(java_err);
    }

    Self::new(path)
  }

  pub fn get_java(&self) -> Result<PathBuf> {
    let path = self.base_path
      .join("bin")
      .join(choice_by_os("java", "java.exe"));

    if !path.is_file() {
      return Err(JavaError::new(JavaErrorKind::NotFound))
    }

    Ok(path)
  }

  pub fn get_javaw(&self) -> Result<PathBuf> {
    let path = self.base_path
      .join("bin")
      // linux/macos doesn't have javaw, so we just return java instead
      .join(choice_by_os("java", "javaw.exe"));

    if !path.is_file() {
      return Err(JavaError::new(JavaErrorKind::NotFound))
    }

    Ok(path)
  }

  pub fn version(&self) -> &JavaVersion {
    &self.version
  }

  /// Checks that Java client is STRICTLY equal to needed_version
  ///
  /// Example
  /// ```rs
  /// let client = Java::find()?;
  /// client.is_version_equal(16)?;
  /// ```
  pub fn is_version_equal(&self, needed_version: u8) -> Result<()> {
    if self.version.main_version() != needed_version {
      return Err(JavaError::new(JavaErrorKind::DifferentVersion))
    }

    Ok(())
  }

  /// Checks if Java client is higher or equal to needed_version
  ///
  /// Example
  /// ```rs
  /// let client = Java::find()?;
  /// client.is_version_at_least(16)?;
  /// ```
  pub fn is_version_at_least(&self, needed_version: u8) -> Result<()> {
    if self.version.main_version() != needed_version {
      return Err(JavaError::new(JavaErrorKind::DifferentVersion))
    }

    Ok(())
  }
}