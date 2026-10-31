# `rml modrinth`

Modrinth API commands.

*Available when `rml` is built with the `modrinth` feature (enabled by default).*

```sh
rml modrinth [OPTIONS] <COMMAND>
```

Backed by `rmlib::extra::modrinth`. All subcommands accept two global flags:

| Flag | Meaning |
|---|---|
| `-j`, `--json` | Print raw JSON instead of the formatted output. |
| `--game-path <DIR>` | Game directory, used by `install*` subcommands. |

Flags are global, so they can be given before or after the subcommand.

## Commands

| Command | Purpose |
|---|---|
| `search` | Search projects with filters and pagination. |
| `project` | Project by slug or ID. |
| `project-by-hash` | Project by a file SHA-1 / SHA-512. |
| `versions` | Versions of a project, with filters. |
| `version` | Version by ID. |
| `version-by-hash` | Version by a file hash. |
| `deps` | Dependencies of a project. |
| `members` | Team members of a project. |
| `user` | User by ID or username. |
| `user-projects` | Projects of a user. |
| `team` | Team members. |
| `tags` | Tag directories (`categories`, `loaders`, `game-versions`, `project-types`). |
| `install` | Install a project (auto-selecting a compatible version). |
| `install-version` | Install a specific version. |
| `install-hash` | Install the version containing a given file hash. |
| `install-datapack` | Install a datapack into a world. |

## Search

```sh
rml modrinth search [OPTIONS] [QUERY]
```

| Option | Meaning |
|---|---|
| `-t, --type <TYPE>` | `mod`, `modpack`, `resourcepack`, `shader`, `datapack`, `world`. |
| `--game-version <VER>` | Filter by Minecraft version (repeatable). |
| `--loader <LOADER>` | Filter by mod loader (repeatable). |
| `--category <CATEGORY>` | Filter by category (repeatable). |
| `--client-side <SIDE>` | `required`, `optional`, `unsupported`, `unknown`. |
| `--server-side <SIDE>` | Same values. |
| `-i, --index <INDEX>` | `relevance`, `downloads`, `follows`, `newest`, `updated`. |
| `-p, --page <N>` | 1-based page (default `1`). |
| `-c, --limit <N>` | Items per page, clamped to `1..=100` (default `20`). |

```sh
rml modrinth search sodium -t mod --game-version 1.21.1 --loader fabric -i downloads
```

## Projects and versions

```sh
rml modrinth project <SLUG_OR_ID>
rml modrinth project-by-hash <HASH>
rml modrinth versions <SLUG_OR_ID> [--game-version VER]... [--loader LOADER]... [--type release|beta|alpha]
rml modrinth version <VERSION_ID>
rml modrinth version-by-hash <HASH>
rml modrinth deps <SLUG_OR_ID>
rml modrinth members <SLUG_OR_ID>
```

## Users and teams

```sh
rml modrinth user <ID_OR_USERNAME>
rml modrinth user-projects <USER_ID>
rml modrinth team <TEAM_ID>
```

## Tags

```sh
rml modrinth tags <categories|loaders|game-versions|project-types>
```

## Installation

```sh
rml modrinth install <SLUG_OR_ID> [OPTIONS]
rml modrinth install-version <VERSION_ID> [OPTIONS]
rml modrinth install-hash <HASH> [OPTIONS]
rml modrinth install-datapack <SLUG_OR_ID> -w <WORLD> [OPTIONS]
```

Install options (shared):

| Option | Meaning |
|---|---|
| `--game-path <DIR>` | Game directory (global). |
| `--install-dir <DIR>` | Explicit target directory; wins over `--game-path` auto-resolution. |
| `--mc <VERSION>` | Minecraft version filter. |
| `--loader <LOADER>` | Loader filter. |
| `--version-type <TYPE>` | `release`, `beta`, `alpha`. |
| `--deps` | Also install required dependencies. |
| `--overwrite` | Overwrite existing files with the same name. |

Without `--install-dir`, the target is chosen by content type:

| Content | Directory |
|---|---|
| Mod | `<game>/mods/` |
| Resource pack | `<game>/resourcepacks/` |
| Shader | `<game>/shaderpacks/` |
| Datapack | `<game>/saves/<world>/datapacks/` |
| Modpack / World | requires `--install-dir` |

`install` selects a version matching `--mc` / `--loader` / `--version-type`; if no compatible version exists, the command fails instead of installing a random one.

`install-datapack` requires `--game-path` and an existing world directory `saves/<WORLD>/`; it is never created.

```sh
rml modrinth install sodium --game-path ./game --mc 1.21.1 --loader fabric --deps
rml modrinth install-version <VERSION_ID> --install-dir ./game/mods
rml modrinth install-datapack terralith -w MyWorld --game-path ./game
```

## JSON output

With `-j`, every subcommand prints raw JSON (the corresponding `rmlib` DTO), suitable for `jq` or `wl-copy`:

```sh
rml modrinth search sodium -j | jq '.hits[].title'
rml modrinth project sodium -j | wl-copy
```

## Errors

API, network, hash-mismatch and filesystem errors are printed to stderr with the `Status::Err` prefix; the exit path is normal (no panic). With `-j`, errors still go to stderr, so stdout stays valid JSON on success.

## See also

- [Run](run.md) — launching the game.
- [rmlib extra::modrinth](https://github.com/musdev13/rmlib) — the Rust-side API.