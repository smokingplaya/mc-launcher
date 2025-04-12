use crate::minecraft::configuration::MinecraftConfiguration as Configuration;
use super::ClientFile;

const JAVA_CLASSPATH_SEPARATOR: &str = if cfg!(target_os = "windows") { ";" } else { ":" };

pub struct ClassPathCollector;

impl ClassPathCollector {
  pub fn collect(
    config: &Configuration,
    client_file: &ClientFile
  ) -> String {
    let mut result: Vec<String> = Vec::new();
    let libraries_dir = config.client.path.join("libraries");

    for lib in &client_file.libraries {
      let mut is_followed = true;
      if let Some(rules) = &lib.rules {
        for rule in rules {
          if !rule.is_followed() {
            is_followed = false;
            break
          }
        }
      }

      if !is_followed {
        continue;
      }

      let path = lib.to_path();

      result.push(libraries_dir.join(&path).to_str().unwrap().to_string());
    }

    result.push(config.client.path.join("versions").join(&config.client.version).join("client.jar").to_str().unwrap().to_string());

    result.join(JAVA_CLASSPATH_SEPARATOR)
  }
}