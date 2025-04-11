use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum JavaErrorKind {
  NotFound,
  OutputReadError,
  InvalidJavaFolderStructure,
  DifferentVersion,
}

#[derive(Debug, Clone)]
pub struct JavaError(JavaErrorKind, Option<String>);

impl JavaError {
  pub fn new(kind: JavaErrorKind) -> Self {
    Self(kind, None)
  }

  pub fn new_with_details(kind: JavaErrorKind, details: String) -> Self {
    Self(kind, Some(details))
  }

  pub fn kind(&self) -> &JavaErrorKind {
    &self.0
  }

  pub fn details(&self) -> Option<&str> {
    self.1.as_deref()
  }
}

impl fmt::Display for JavaError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.1 {
      Some(d) => write!(f, "{:?}: {}", self.0, d),
      None => write!(f, "{:?}", self.0),
    }
  }
}

impl Error for JavaError {}

impl From<JavaError> for std::io::Error {
  fn from(v: JavaError) -> Self {
    let kind = match v.0 {
      JavaErrorKind::NotFound => std::io::ErrorKind::NotFound,
      JavaErrorKind::OutputReadError => std::io::ErrorKind::InvalidInput,
      JavaErrorKind::InvalidJavaFolderStructure => std::io::ErrorKind::InvalidData,
      JavaErrorKind::DifferentVersion => std::io::ErrorKind::Other,
    };

    Self::new(kind, v)
  }
}

pub type Result<T> = std::result::Result<T, JavaError>;