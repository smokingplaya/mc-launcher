// use std::{io, process::{Command, Stdio}};
use std::{io::Result, process::Stdio};
use crate::{client::classpath::ClassPathCollector, java::Java};
use super::{arguments::generate_data_map, configuration::MinecraftConfiguration as Configuration};

#[cfg(feature = "tokio")]
use tokio::process::{Child, Command};

#[cfg(not(feature = "tokio"))]
use std::process::{Child, Command};


#[derive(Debug, Clone)]
pub struct MinecraftLauncher<'a>(Configuration<'a>);

impl<'a> From<Configuration<'a>> for MinecraftLauncher<'a> {
  fn from(config: Configuration<'a>) -> Self {
    Self(config)
  }
}

impl<'a> MinecraftLauncher<'a> {
  pub fn new(config: Configuration<'a>) -> Self {
    Self(config)
  }

  pub fn start(self) -> Result<Child> {
    let mut client = self.0.client.get_client_info()?;

    let java = self.0.java.clone()
      .unwrap_or(Java::find()?);

    let class_path = ClassPathCollector::collect(&self.0, &client);
    let arguments = generate_data_map(self.0.clone(), client.clone(), class_path);
    let process_args = client.arguments.collect(&self.0, arguments)?;

    let child = Command::new(java.get_javaw()?)
      .args(process_args)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn()?;

    Ok(child)
  }
}