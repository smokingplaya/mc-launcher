use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OsKind {
  #[serde(rename = "windows")]
  Windows,
  #[serde(rename = "osx")]
  #[serde(alias = "macos")]
  MacOS,
  #[serde(rename = "linux")]
  Linux
}

impl Default for OsKind {
  fn default() -> Self {
    match env::consts::OS {
      "windows" => OsKind::Windows,
      "macos" => OsKind::MacOS,
      "linux" => OsKind::Linux,
      _ => OsKind::Linux // this dude rn 🐧
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OsSpecifier {
  pub name: Option<OsKind>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleFeatures {
  is_demo_user: Option<bool>,
  has_custom_resolution: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RuleAction {
  #[serde(rename = "allow")]
  Allow,
  #[serde(rename = "disallow")]
  Disallow
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
  pub action: RuleAction,
  pub os: Option<OsSpecifier>,
  pub features: Option<RuleFeatures>
}

impl Rule {
  /// Is the rule being followed?
  /// TODO @ make features support
  pub fn is_followed(&self) -> bool {
    match &self.os {
      // lol
      Some(OsSpecifier{ name: Some(os)  }) => {
        let current_os = OsKind::default();
        match self.action {
          RuleAction::Allow => os == &current_os,
          RuleAction::Disallow => os != &current_os,
        }
      },
      _ => true
    }
  }
}