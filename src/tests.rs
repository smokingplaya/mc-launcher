#[test]
fn validate_java_version() -> anyhow::Result<()> {
  use crate::java::Java;
  let java = Java::find()?;

  assert_eq!(java.version().main_version(), 8, "java version is not 8");

  Ok(())
}

#[test]
fn start_game() -> anyhow::Result<()> {
  use std::path::Path;
  use crate::java::Java;
  use crate::minecraft::{launcher::MinecraftLauncher, configuration::{AuthLibConfiguration, MinecraftClient, MinecraftConfiguration, MinecraftSession, WindowConfiguration}};

  let java = Java::find()?;

  let config = MinecraftConfiguration {
    java: Some(java),
    window: WindowConfiguration::default(),
    session: MinecraftSession {
      username: "smxkin",
      authlib_server: Some(AuthLibConfiguration {
        server: "https://riverfall.ru/api/session",
        ..Default::default()
      }),
      ..Default::default()
    },

    client: MinecraftClient {
      path: Path::new("C:\\Users\\smxkin\\AppData\\Roaming\\ru.riverfall.launcher\\clients\\technorpg").to_path_buf(),
      version: "Forge 1.12.2",
      ..Default::default()
    }
  };

  MinecraftLauncher::new(config)
    .start()?;

  Ok(())
}