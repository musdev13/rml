# `rml login ms`

Official Microsoft / Minecraft account commands.

Backed by `rmlib::core::auth`. Tokens are stored in `ms.json` inside the auth directory, which is `auth_path` from `paths.json` by default (`~/rml/auth`).

Available with the default build and with `--no-default-features` — unlike [`rml login ely`](ely.md), it does not depend on the `ely` feature.

## `--auth-path`

All `login ms` subcommands accept a global `--auth-path <DIR>` flag, placed after `ms`:

```sh
rml login ms --auth-path /path/to/auth data
```

- With `--auth-path` — `ms.json` is read from and written to `DIR`.
- Without it — the `auth_path` from the config is used.

The flag is `global`, so it can also be given after the subcommand (`rml login ms data --auth-path DIR`).

## `rml login ms login`

```sh
rml login ms login
```

Starts the interactive Microsoft **device code** login:

1. Prints a verification URL and a user code (to **stderr**, so a piped stdout stays clean).
2. Waits until you confirm in the browser.
3. Saves the resulting tokens to `ms.json`.
4. Without `--json`, asks for a key press before printing the tokens — they are personal data.

| Flag | Meaning |
|---|---|
| `-j`, `--json` | Print the saved tokens as JSON instead of the formatted output. Skips the key-press prompt. |

```sh
rml login ms login -j | wl-copy   # stdout is the tokens JSON only
```

```sh
rml login ms login --json
```

```json
{
  "access_token": "…",
  "refresh_token": "…",
  "expires_in": 3600
}
```

## `rml login ms refresh`

```sh
rml login ms refresh
```

Refreshes the saved session without user interaction: reads the `refresh_token` from `ms.json`, exchanges it for new tokens, and saves them back. Microsoft rotates refresh tokens, so the file is updated.

Fails if there is no saved `ms.json` — run `rml login ms login` first.

Without `--json`, asks for a key press before printing the refreshed tokens (personal data). With `--json`, stdout is the tokens JSON only.

| Flag | Meaning |
|---|---|
| `-j`, `--json` | Print the refreshed tokens as JSON. Skips the key-press prompt. |

## `rml login ms data`

```sh
rml login ms data
```

Retrieves the Minecraft profile. This is the high-level command:

1. Reads `ms.json`.
2. Refreshes the tokens; if that fails, falls back to the device-code login.
3. If no tokens exist, starts the device-code login.
4. Runs Xbox Live → XSTS → Minecraft Services and prints the profile.

If the account has no Minecraft license, a warning is printed and the license-dependent fields contain the `"no_license"` placeholder — the command still prints all data it could obtain (for example `xuid`).

Without `--json`, asks for a key press before printing the profile — it contains a personal access token. With `--json`, stdout is the profile JSON only.

| Flag | Meaning |
|---|---|
| `-j`, `--json` | Print the profile as JSON. Skips the key-press prompt. |

```sh
rml login ms data --json
```

```json
{
  "username": "Steve",
  "uuid": "…",
  "access_token": "…",
  "xuid": "…",
  "has_license": true
}
```

## `rml login ms logout`

```sh
rml login ms logout
```

Deletes `ms.json` from the auth directory — logs the account out on this machine.

| Flag | Meaning |
|---|---|
| `-j`, `--json` | Print `{"ok": true}` instead of the status line. |

## Typical flow

```sh
rml login ms login      # first time
rml login ms data       # get the profile (refreshes or logs in as needed)
rml login ms refresh    # refresh the session explicitly
rml login ms logout     # log out
```

For scripting, `rml login ms data --json` is usually enough — it handles refresh and login internally. With `--json`, all human-readable output (device-code banner, prompts, warnings) goes to stderr, so `… --json | jq` / `| wl-copy` receives clean JSON.

## See also

- [Init](../../getting-started/init.md) — `auth_path` in `paths.json`.
- [Ely.by login](ely.md) — the alternative (non-official) auth flow.