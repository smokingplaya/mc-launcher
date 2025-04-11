use std::fmt::Display;
use std::sync::LazyLock;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::client::rules::Rule;
use crate::java::JAVA_SEPARATOR;
use crate::minecraft::arguments::DataMap;
use crate::minecraft::configuration::MinecraftConfiguration as Configuration;

use super::CollectArguments;

static ARGUMENT_FIND_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\{([^}]+)\}").unwrap());

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arguments {
  /// Contains JVM arguments, such as information about memory allocation, garbage collector selection, or environmental variables.
  jvm: Vec<Argument>,
  /// Contains arguments supplied to the game, such as information about the username and the version.
  game: Vec<Argument>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ArgumentValue {
  String(String),
  Array(Vec<String>)
}

impl Display for ArgumentValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      ArgumentValue::String(s) => s.to_owned(), // cloning :(
      ArgumentValue::Array(a) => a.join(JAVA_SEPARATOR),
    };
    write!(f, "{}", str)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argument {
  value: ArgumentValue,
  rules: Option<Vec<Rule>>
}

impl Display for Argument {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(rules) = &self.rules {
      for rule in rules {
        if !rule.is_followed() {
          return write!(f, "{}", String::new())
        }
      }
    }

    write!(f, "{}", self.value)
  }
}

impl CollectArguments for Vec<Argument> {
  fn collect(&self) -> Vec<String> {
    let mut result = Vec::new();

    for arg in self {
      for part in arg.to_string().split(';') {
        let trimmed = part.trim();
        if !trimmed.is_empty() {
          result.push(trimmed.to_string());
        }
      }
    }

    result
  }
}

fn jvm_setup(config: &Configuration, jvm: &mut Vec<Argument>) {
  if let Some(authlib) = &config.session.authlib_server {
    let version = authlib.version;
    jvm.insert(0, Argument { value: ArgumentValue::String(format!("-javaagent:${{libraries_dir}}/moe/yushi/authlibinjector/{}/authlibinjector-{}.jar=${{authlib_server}}", version, version)), rules: None });
  }
}

fn game_setup(config: &Configuration, game: &mut Vec<Argument>) {
  if config.window.width.is_some() {
    game.push(Argument { value: ArgumentValue::String("--width; ${width}".to_string()), rules: None });
  }

  if config.window.height.is_some() {
    game.push(Argument { value: ArgumentValue::String("--height; ${height}".to_string()), rules: None });
  }

  if config.window.fullscreen.is_some() {
    game.push(Argument { value: ArgumentValue::String("--fullscreen".to_string()), rules: None });
  }
}

impl Arguments {
  /// Combines the arguments from ``jvm`` and ``game``,
  /// turning them into a single array of strings that can be used as arguments to start a process.
  pub fn collect(&mut self, config: &Configuration, data_map: DataMap<'_>) -> std::io::Result<Vec<String>> {
    jvm_setup(config, &mut self.jvm);
    game_setup(config, &mut self.game);

    let mut result = self.jvm.collect();
    result.extend(self.game.collect());

    let pattern = &ARGUMENT_FIND_PATTERN; // regex: \$\{([^}]+)\}
    let mut final_result = Vec::new();

    for arg in result.iter() {
      let mut new_arg = arg.clone();

      for cap in pattern.captures_iter(arg) {
        let key = &cap[1];

        if let Some(value) = data_map.get(key) {
          new_arg = new_arg.replace(&cap[0], value);
        }
        // TODO @ ignore not changed strings
      }

      final_result.push(new_arg);
    }

    Ok(final_result)
  }
}