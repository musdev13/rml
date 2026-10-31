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
| `install` | Install one or more projects (auto-selecting compatible versions). |
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
rml modrinth deps <SLUG_OR_ID> [OPTIONS]
rml modrinth members <SLUG_OR_ID>
```

`deps` options:

| Option | Meaning |
|---|---|
| `--version <VERSION_ID>` | Show dependencies of this specific version only. |
| `--mc <VERSION>` | Restrict to versions for this Minecraft version. |
| `--loader <LOADER>` | Restrict to versions for this loader. |
| `--version-type <TYPE>` | `release`, `beta`, `alpha`. |

Without `--version`, `deps` aggregates dependencies over the project versions that match the filters. Without any filters it covers all versions — a dependency may come from a version unrelated to your target, so pass `--mc` / `--loader` when checking compatibility. With `--version`, the filters are ignored and only that version's dependencies are shown.

```sh
rml modrinth deps so-big-250 --mc 26.3 --loader fabric
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
rml modrinth install <SLUG_OR_ID>... [OPTIONS]
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

`install` accepts **several** projects at once. Both forms are equivalent:

```sh
rml modrinth install so-big-250 bcOsQHnC --game-path ./game --mc 26.3 --loader fabric --deps
rml modrinth install "so-big-250 bcOsQHnC" --game-path ./game --mc 26.3 --loader fabric --deps
```

Each target is installed with its own required dependencies; a failure for one target does not stop the others, and already-downloaded files whose SHA-1 matches are not downloaded again.

`install-datapack` requires `--game-path` and an existing world directory `saves/<WORLD>/`; it is never created.

With `--deps`, only **required** dependencies are installed. Any required dependency that could not be installed is reported on stderr:

```
!Warn: skipped required dependency <project_id> (no_compatible_version)
```

Reasons: `project_unresolved`, `no_compatible_version`, `unsupported_content_type`. All installed files are listed on stdout.

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

For install commands the JSON is an `InstallResult` with `installed` and `skipped` arrays. For multi-target `install`, the results are merged into one `InstallResult`.

## Errors

API, network, hash-mismatch and filesystem errors are printed to stderr with the `Status::Err` prefix; the exit path is normal (no panic). With `-j`, errors still go to stderr, so stdout stays valid JSON on success. Skipped dependencies from `--deps` are warnings on stderr and do not change the exit path.

## See also

- [Run](run.md) — launching the game.
- [rmlib extra::modrinth](https://github.com/musdev13/rmlib) — the Rust-side API.