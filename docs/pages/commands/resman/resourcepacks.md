# `rml resman resourcepacks`

Inspect locally installed resource packs.

*Available when `rml` is built with the `resman-resourcepacks` feature (enabled by default).*

```sh
rml resman resourcepacks <COMMAND>
```

| Command | Purpose |
|---|---|
| `list` | List installed resource packs. |
| `info` | Show info for one resource pack. |

Backed by `rmlib::extra::resman::resourcepacks`.

There is no enable/disable/toggle here: resource packs are not toggled by renaming.

## `rml resman resourcepacks list`

```sh
rml resman resourcepacks list [OPTIONS]
```

| Option | Meaning |
|---|---|
| `--game-path <DIR>` | Game directory; packs are read from `<DIR>/resourcepacks`. |
| `--dir <DIR>` | Explicit resource-packs directory (overrides `--game-path`). |
| `--file <PATH>` | Operate on a single resource pack ZIP instead of a directory. |
| `--priority <SRCS>` | Comma-separated info sources: `local`, `modrinth`, `modrinth,local`. Default `local`. |
| `--skip` | Skip packs that no source could resolve. Conflicts with `--fallback-fields`. |
| `--fallback-fields <FIELDS>` | When no source resolves a pack, fill only these fields from the local ZIP. |
| `--fields <FIELDS>` | Comma-separated field list, or `all` (default: `filename,pack_format,description`). |
| `--format <FMT>` | `table` (default), `json`, `plain`. |
| `-j`, `--json` | Print raw JSON (overrides `--format`). |

Fields: `id`, `description`, `pack_format`, `supported_formats`, `icon`, `path`, `enabled`, `filename`, `size`. `id` (Modrinth project slug) is filled only with `--priority modrinth,...`; local parsing leaves it empty.

```sh
rml resman resourcepacks list --game-path ./game
rml resman resourcepacks list --dir ./my-packs --fields description,pack_format
rml resman resourcepacks list --game-path ./game --priority modrinth,local
```

### Priority and fallback

`--priority` controls where pack metadata comes from:

- `local` — read `pack.mcmeta` from the ZIP.
- `modrinth` — look the ZIP up by SHA-1 in the Modrinth API (requires the `modrinth` feature).

Sources are tried in order; the first that resolves a pack wins. **With `modrinth,local`, empty fields from Modrinth are filled from the local ZIP.** With `--priority modrinth` alone, nothing is filled from the local ZIP and, if Modrinth has no match, the local ZIP is not consulted.

When no source resolves a pack:

| Flags | Behaviour |
|---|---|
| *(none)* | Placeholder entry: file name, path, size, enabled; other fields empty. |
| `--skip` | The entry is dropped. |
| `--fallback-fields description,pack_format` | Those fields come from the local ZIP, the rest stay empty. |

## `rml resman resourcepacks info`

```sh
rml resman resourcepacks info [NAME] [OPTIONS]
rml resman resourcepacks info --file <PATH> [OPTIONS]
```

`NAME` is matched against the file stem (`.zip` / `.zip.disabled` stripped). With `--file`, the path is used directly and `NAME` is ignored.

Same options as `list` (`--game-path`, `--dir`, `--file`, `--priority`, `--skip`, `--fallback-fields`, `--fields`, `--format`, `-j`); `--format` defaults to `plain`, `--fields` defaults to all fields.

```sh
rml resman resourcepacks info "Faithful 32x - 1.21.1" --game-path ./game
rml resman resourcepacks info --file "./game/resourcepacks/Faithful 32x - 1.21.1.zip"
```

## Examples

List packs with their format and Minecraft version:

```sh
rml resman resourcepacks list --game-path ./game --fields filename,pack_format
```

Try Modrinth first, fall back to the local `pack.mcmeta`:

```sh
rml resman resourcepacks list --game-path ./game --priority modrinth,local
```

Only Modrinth, taking descriptions from the local ZIP when it cannot resolve:

```sh
rml resman resourcepacks list --game-path ./game --priority modrinth --fallback-fields description
```

Get a single pack as JSON:

```sh
rml resman resourcepacks info "Faithful 32x - 1.21.1" --game-path ./game -j
```

## See also

- [resman::mods](mods.md) — the mod side.
- [Run](../run.md) — launching the game.