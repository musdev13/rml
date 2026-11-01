# `rml resman mods`

Inspect and toggle locally installed mods.

*Available when `rml` is built with the `resman-mods` feature (enabled by default).*

```sh
rml resman mods <COMMAND>
```

| Command | Purpose |
|---|---|
| `list` | List installed mods. |
| `info` | Show info for one mod. |
| `enable` | Enable a mod (remove the `.disabled` suffix). |
| `disable` | Disable a mod (append the `.disabled` suffix). |
| `toggle` | Toggle a mod's enabled state. |

Backed by `rmlib::extra::resman::mods`.

## `rml resman mods list`

```sh
rml resman mods list [OPTIONS]
```

| Option | Meaning |
|---|---|
| `--game-path <DIR>` | Game directory; mods are read from `<DIR>/mods`. |
| `--dir <DIR>` | Explicit mods directory (overrides `--game-path`). |
| `--file <PATH>` | Operate on a single mod JAR instead of a directory. |
| `--priority <SRCS>` | Comma-separated info sources: `local`, `modrinth`, `modrinth,local`. Default `local`. |
| `--skip` | Skip mods that no source could resolve. Conflicts with `--fallback-fields`. |
| `--fallback-fields <FIELDS>` | When no source resolves a mod, fill only these fields from the local JAR. |
| `--fields <FIELDS>` | Comma-separated field list, or `all` (default: `id,name,version,loader,minecraft,enabled`). |
| `--format <FMT>` | `table` (default), `json`, `plain`. |
| `-j`, `--json` | Print raw JSON (overrides `--format`). |

Fields: `id`, `name`, `version`, `description`, `authors`, `license`, `minecraft`, `dependencies`, `icon`, `path`, `enabled`, `loader`, `filename`, `size`, `fabric_api`.

```sh
rml resman mods list --game-path ./game
rml resman mods list --game-path ./game --fields id,name,version --format table
rml resman mods list --game-path ./game --priority modrinth,local
rml resman mods list --game-path ./game --priority modrinth --skip
rml resman mods list --game-path ./game --priority modrinth --fallback-fields icon,description
```

### Priority and fallback

`--priority` controls where mod metadata comes from:

- `local` — read the JAR manifest (`META-INF/neoforge.mods.toml` / `fabric.mod.json`).
- `modrinth` — look the JAR up by SHA-1 in the Modrinth API (requires the `modrinth` feature).

Sources are tried in order; the first that resolves a mod wins. **With `modrinth,local`, empty fields from Modrinth are filled from the local JAR.** With `--priority modrinth` alone, nothing is filled from the local JAR and, if Modrinth has no match, the local JAR is not consulted.

Modrinth resolves by the JAR's SHA-1 (`/version_file/{hash}`), so `version`, `loader` and `minecraft` come from the exact version, not from the whole project.

When no source resolves a mod:

| Flags | Behaviour |
|---|---|
| *(none)* | Placeholder entry: file name, path, size, enabled; other fields empty. |
| `--skip` | The entry is dropped. |
| `--fallback-fields icon,description` | Those fields come from the local JAR, the rest stay empty. |

## `rml resman mods info`

```sh
rml resman mods info [ID] [OPTIONS]
rml resman mods info --file <PATH> [OPTIONS]
```

`ID` is matched against `ModInfo::id`. With `--file`, the path is used directly and `ID` is ignored.

Same options as `list` (`--game-path`, `--dir`, `--file`, `--priority`, `--skip`, `--fallback-fields`, `--fields`, `--format`, `-j`); `--format` defaults to `plain`, `--fields` defaults to all fields (there is no default subset for a single mod).

```sh
rml resman mods info sodium --game-path ./game
rml resman mods info --file ./game/mods/sodium.jar
```

## `rml resman mods enable` / `disable` / `toggle`

```sh
rml resman mods enable <ID> [OPTIONS]
rml resman mods disable <ID> [OPTIONS]
rml resman mods toggle <ID> [OPTIONS]
```

Disabling appends `.disabled` to the file name; enabling removes it; toggling flips the current state. All three accept `--file <PATH>` in place of an `ID`.

Options: `--game-path`, `--dir`, `--file`, `-j`.

With `-j`, the result is:

```json
{ "path": "./game/mods/sodium.jar.disabled", "enabled": false }
```

```sh
rml resman mods disable sodium --game-path ./game
rml resman mods enable  sodium --game-path ./game -j
rml resman mods toggle  --file ./game/mods/sodium.jar
```

## See also

- [resman::resourcepacks](resourcepacks.md) — the resource-pack side.
- [Run](../run.md) — launching the game.