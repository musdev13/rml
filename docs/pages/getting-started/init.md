# Initializing paths
After installation, you should **initialize rml** and optionally change the default paths in the configuration.

To do this, run the following command:
```sh
rml init
```
Then check your `~/.config` or `%AppData%` directory for rml configuration

# paths config
**by default:**
```json
{
  "assets_path": "~/rml/assets",
  "libs_path": "~/rml/libs",
  "versions_path": "~/rml/versions",
  "auth_path": "~/rml/auth"
}
```
- **assets_path** - where game assets will be downloaded
- **libs_path** - where game libraries will be stored
- **versions_path** - where game versions will be...
- **auth_path** - where authentication data will be stored

# Checking the config

Before running any command, `rml` validates `paths.json` and prints a warning to stderr if it is invalid — the command still runs, so a stale config never blocks you.

To check your config without side effects, run:

```sh
rml test
```

`rml test` only prints an ASCII cat, so it is the safe way to trigger the validation. If nothing but the cat is printed, the config is valid.

**After updating `rml`, run `rml test` once.** New releases may add config fields (for example, `auth_path` appeared in `rmlib` 0.10); an older `paths.json` will then produce a warning. Run `rml init --force` to rewrite it with the current defaults — this discards any custom paths you set.