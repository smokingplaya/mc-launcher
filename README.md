# mc-launcher
``mc-launcher`` is a zero-dependency (lightweight) helpful crate, that adds functionality to run Minecraft via Rust.

# Features
* Lightweight
* Fast
* [authlib-injector](http://github.com/yushijinhun/authlib-injector/) support

# Installation

```shell
cargo add mc-launcher
```

# Usage
## Minecraft's game_dir structure
```
.minecraft/
  🗁 assets/
  🗁 libraries/
  🗁 versions/
    🗁 <version_name>/
      🗁 natives
      🖹 client.json
      🖹 client.jar
```

## Launching Minecraft
```rust
let config = MinecraftConfiguration {
  session: MinecraftSession {
    username: "smxkin", // username of player
    ..Default::default()
  },

  client: MinecraftClient {
    path: Path::new("C:\\Users\\smxkin\\AppData\\Roaming\\ru.riverfall.launcher\\clients\\technorpg").to_path_buf(),
    version: "Forge 1.12.2",
    ..Default::default()
  },

  ..Default::default()
};

MinecraftLauncher::new(config)
  .start()?;
```

## Authlib-injector support

> [!NOTE]
> The authlib-injector file should be located in ``.minecraft/libraries/moe/yushi/authlibinjector/<version>/authlibinjector-<version>.jar``.
>
> That is, for version 1.2.5 the path will be as follows: ``.minecraft/libraries/moe/yushi/authlibinjector/1.2.5/authlibinjector-1.2.5.jar``.

```rust
MinecraftConfiguration {
  session: MinecraftSession {
    username: "smxkin", // username of player
    authlib_server: Some(AuthLibConfiguration {
        server: "https://auth.mojang.com",
        version: "1.2.5"
      }),
    ..Default::default()
  },
};
```

## Finding Java on PC
```rust
let java = Java::find()?;
```

## Java version comparison
```rust
let java = Java::find()?;

if java.is_version_equal(8) {
  println!("Version == 8");
}

if java.is_version_at_least(8) {
  println!("Version >= 8");
}
```