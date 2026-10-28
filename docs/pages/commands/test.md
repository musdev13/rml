```sh
rml test
```
just test command.

prints an ASCII cat to stdout:

```
  /\_/\
 ( o.o )
  > ^ <
  meow~
```

there's nothing else to do. **:3**

## Using `test` to check your config

Before running any command, `rml` validates `paths.json` (see `rmlib::core::config::check_config`) and prints a warning to stderr if it is invalid — but the command still runs.

`rml test` does nothing except print the cat, so it is the safe way to trigger that check without side effects:

```sh
rml test
```

- Config is valid → only the cat is printed.
- Config is invalid → a `!Warn: config is invalid (...); continuing anyway` line appears before the cat, with the specific error.

**After updating `rml`, run `rml test` once** to confirm your config is still valid (for example, after a release that adds new config fields).