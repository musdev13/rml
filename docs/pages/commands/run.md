```sh
rml run [OPTIONS] <VERSION_ID>
```

run Minecraft client

### Arguments

| Argument       | Description                               |
| -------------- | ----------------------------------------- |
| `<VERSION_ID>` | Game version ID to run (e.g., `1.20.4`). |

### Options

| Option | Description | Default |
| --- | --- | --- |
| `-r, --ram <RAM>` | Amount of RAM allocated for JVM (e.g., `4G` or `3500M`). | `4G` |
| `-u, --username <USERNAME>` | Player nickname. | `MusPlayer` |
| `--uuid <UUID>` | Player UUID. | `00000000-0000-0000-0000-000000000000` |
| `--token <TOKEN>` | Authentication access token. | `null` |
| `-e, --ely` | Enable Ely.by skin system / authentication. | — |
| `-b, --betafix` | Apply Betacraft/ely.by fix for pre-1.6 versions. | — |
| `--just-command` | Print the launch command instead of starting the game. | — |
| `--jvm-args <JVM_ARGS>` | **Optional.** Additional JVM arguments. | — |
| `--game-args <GAME_ARGS>` | **Optional.** Additional game arguments. | — |
| `--versions <VERSIONS_PATH>` | **Optional.** Path to the versions directory where JSON and client JAR are stored. | Config |
| `--assets <ASSETS_PATH>` | **Optional.** Path to the assets directory. | Config |
| `--libs <LIBS_PATH>` | **Optional.** Path to the libraries directory. | Config |
| `-g, --game-path <GAME_PATH>` | **Optional.** Path to the game directory (working directory for launch). | `~/.minecraft` |
| `-s, --spawn-path <SPAWN_PATH>` | **Optional.** Path to the directory from which the game is launched. | `GAME_PATH` |
| `--fabric <FABRIC_VERSION>` | **Optional.** Specify Fabric Loader version to run. Conflicts with `--neoforge`. | — |
| `--neoforge <NEOFORGE_VERSION>` | **Optional.** Specify NeoForge version to run. Conflicts with `--fabric`. | — |

### Paths

By default, `rml` uses the configured paths for versions, assets, and libraries.

The game directory defaults to `~/.minecraft`.

If `--spawn-path` is not specified, it defaults to the same path as `--game-path`.

### Modloaders

The `--fabric` and `--neoforge` options allow you to run a specific modloader version.

These options are mutually exclusive.

### Custom Arguments

`--jvm-args` allows you to provide additional arguments for the JVM.

`--game-args` allows you to provide additional arguments for Minecraft.

### Examples

Run Minecraft version `26.2`:

```sh
rml run 26.2
```

Run Minecraft with `8G` of RAM:

```sh
rml run 26.2 --ram 8G
```

Run Minecraft with a custom username:

```sh
rml run 26.2 --username Mus
```

Run Minecraft with Ely.by authentication:

```sh
rml run 26.2 --ely
```

Run a Fabric installation:

```sh
rml run 26.2 --fabric 0.17.2
```

Run a NeoForge installation:

```sh
rml run 26.2 --neoforge 21.8.31
```

Apply the Betacraft fix for a pre-1.6 version:

```sh
rml run 1.5.2 --betafix
```

Use custom JVM arguments:

```sh
rml run 26.2 --jvm-args "-XX:+UseG1GC -XX:MaxGCPauseMillis=50"
```

Use custom Minecraft arguments:

```sh
rml run 26.2 --game-args "--fullscreen"
```

Use a custom game directory:

```sh
rml run 26.2 --game-path ./game
```

Use a custom launch directory:

```sh
rml run 26.2 --spawn-path ./instance
```

Use custom versions, libraries, and assets directories:

```sh
rml run 26.2 --versions ./versions --libs ./libraries --assets ./assets
```

Print the generated launch command without starting Minecraft:

```sh
rml run 26.2 --just-command
```

Run Fabric with custom RAM and JVM arguments:

```sh
rml run 26.2 --fabric 0.17.2 --ram 8G --jvm-args "-XX:+UseG1GC"
```

Run Minecraft with custom game and launch directories:

```sh
rml run 26.2 --game-path ./game --spawn-path ./instance
```

My full launch command _(with wayland fix for linux)_:
```sh
rml run --jvm-args "-Dorg.lwjgl.glfw.libname=/usr/lib/libglfw.so" --token "eyJ0e...FGA" --uuid "53f9...8f29e" -u Musek -e -r 6G 1.21.10 --fabric 0.19.3 -g ./minicraftjennymodfurryfemboys
```
