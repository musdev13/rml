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
| `-u, --username <USERNAME>` | Player nickname. With `--profile`, also selects which profile to load. | `MusPlayer` |
| `--uuid <UUID>` | Player UUID. | `00000000-0000-0000-0000-000000000000` |
| `--token <TOKEN>` | Authentication access token. | `null` |
| `--xuid <XUID>` | Xbox User ID. | `noxuid` |
| `-e, --ely` | Enable Ely.by skin system / authentication. With `--profile`, selects the `ely` profile directory. | — |
| `-p, --profile` | Load account data from a saved profile. | — |
| `--auth-path <DIR>` | Directory holding saved profiles. | Config (`auth_path`) |
| `-b, --betafix` | Apply Betacraft/ely.by fix for pre-1.6 versions. | — |
| `--just-command` | Print the launch command instead of starting the game. | — |
| `--jvm-args <JVM_ARGS>` | **Optional.** Additional JVM arguments. | — |
| `--game-args <GAME_ARGS>` | **Optional.** Additional game arguments. | — |
| `--versions <VERSIONS_PATH>` | **Optional.** Path to the versions directory where JSON and client JAR are stored. | Config |
| `--assets <ASSETS_PATH>` | **Optional.** Path to the assets directory. | Config |
| `--libs <LIBS_PATH>` | **Optional.** Path to the libraries directory. | Config |
| `-g, --game-path <GAME_PATH>` | **Optional.** Path to the game directory (working directory for launch). | `~/.minecraft` |
| `-s, --spawn-path <SPAWN_PATH>` | **Optional.** Path to the directory from which the game is launched. | `GAME_PATH` |
| `--java <JAVA_PATH>` | **Optional.** Path to the Java executable to use for launching. | `java` from `PATH` |
| `--fabric <FABRIC_VERSION>` | **Optional.** Specify Fabric Loader version to run. Conflicts with `--neoforge`. | — |
| `--neoforge <NEOFORGE_VERSION>` | **Optional.** Specify NeoForge version to run. Conflicts with `--fabric`. | — |

### Profiles

With `--profile` / `-p`, `rml` loads account data from a saved profile before launching:

1. The profile is looked up by `--username` (default `MusPlayer`).
2. The kind is `ely` when `-e/--ely` is set, `ms` otherwise.
3. The file is read from `<auth-path>/profiles/<kind>/<username>.json` (`--auth-path` overrides the configured `auth_path`).

**Precedence:** `--username`, `--uuid`, `--token`, `--xuid` given on the command line always override the profile values; anything not given falls back to the profile, then to the built-in defaults.

So the minimal launch from a saved profile is just the version, nickname, and `-p`:

```sh
rml run -r 6G -u Musek -e -p -g ./ --fabric 0.20.1 26.3
```

If the profile cannot be found or read, a warning is printed and the launch continues with CLI values / defaults.

Profiles are created at login time with `rml login ely --save` / `rml login ms --save` (see [login/ms](login/ms.md)).

### Paths

By default, `rml` uses the configured paths for versions, assets, and libraries.

The game directory defaults to `~/.minecraft`.

If `--spawn-path` is not specified, it defaults to the same path as `--game-path`.

### Java executable

By default, `rml` launches the game using `java` from `PATH`.

Use `--java <JAVA_PATH>` to point at a specific Java executable (e.g. a different JDK version, or a full path like `/usr/lib/jvm/temurin-21/bin/java`). The path is used as-is — no shell expansion beyond what the OS performs.

This option is ignored by `--just-command` (that flag only prints the generated command; the executable is always rendered as `java`).

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
rml run b1.7.3 --betafix
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

Use a specific Java executable:

```sh
rml run 26.2 --java /usr/lib/jvm/temurin-21/bin/java
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

Launch from a saved Ely.by profile (`--uuid` / `--token` would override the profile values):

```sh
rml run 26.3 -r 6G -u Musek -e -p --fabric 0.20.1
```

Launch from a profile stored in a custom directory:

```sh
rml run 26.3 -r 6G -u Musek -p --auth-path ./my-auth
```

My full launch command _(with wayland fix for linux)_:
```sh
rml run --jvm-args "-Dorg.lwjgl.glfw.libname=/usr/lib/libglfw.so" -u Musek -e -p -r 6G 1.21.10 --fabric 0.19.3 -g ./minicraftjennymodfurryfemboys
```
> **UPD**: minecraft 26.3 works great without this fix, but I'll leave it here just in case