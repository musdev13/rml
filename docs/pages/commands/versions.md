```sh
rml versions <COMMAND>
```
versions list and installation

## fetch-list
```sh
rml versions fetch-list [OPTIONS]
```
fetch versions list from mojang launcher-meta
### Options

| Option | Description |
|---|---|
| `-t, --type <TYPE>` | Specifies the type of Minecraft versions to display. Possible values: `release`, `snapshot`, `old-beta`, `old-alpha`. |
| `-p, --page <PAGE>` | Specifies the page to display. Defaults to `0`. |
| `-c, --count <COUNT>` | Specifies the number of versions to display. Defaults to `10`. |
| `-j, --json` | Outputs the result in JSON format. |
| `--show-id` | Includes the `id` field in the output. |
| `--show-release-time` | Includes the `releaseTime` field in the output. |
| `--show-time` | Includes the `time` field in the output. |
| `--show-type` | Includes the `type` field in the output. |
| `--show-url` | Includes the `url` field in the output. |
| `-h, --help` | Displays help information. |

### Examples
Fetch the latest 10 release versions:

    rml versions fetch-list

Fetch the latest 20 snapshot versions:

    rml versions fetch-list --type snapshot --count 20

Fetch the second page of release versions:

    rml versions fetch-list --type release --page 1

Output the versions in JSON format:

    rml versions fetch-list --json

Show additional version information:

    rml versions fetch-list --show-id --show-release-time --show-type

Fetch old beta versions:

    rml versions fetch-list --type old-beta --count 10

Fetch old alpha versions as JSON with all available fields:

    rml versions fetch-list --type old-alpha --json --show-id --show-release-time --show-time --show-type --show-url

## install
```sh
rml versions install rml versions install [OPTIONS] [VERSION_ID]
```
install vanilla version
### Arguments

| Argument | Description |
|---|---|
| `[VERSION_ID]` | Game version ID to install (e.g., `1.20.4`). Conflicts with `--json`. |

### Options

| Option                       | Description                                                                    | Default |
| ---------------------------- | ------------------------------------------------------------------------------ | ------- |
| `-j, --json <PATH>`          | **Optional.** Path to a locally downloaded version JSON file. Conflicts with `[VERSION_ID]`. | — |
| `-d, --directory <PATH>`     | **Optional.** Path to the versions directory.                                      | —       |
| `-l, --libs <LIBS_PATH>`     | **Optional.** Path to the libraries directory.                                 | —       |
| `-a, --assets <ASSETS_PATH>` | **Optional.** Path to the assets directory.                                    | —       |
| `-s, --soft`                 | Perform a soft installation.                                                   | —       |
| `--iclient`                  | Install the client.                                                             | —       |
| `--ilibs`                    | Install libraries.                                                              | —       |
| `--iassets`                  | Install assets.                                                                | —       |
| `--alibs <ALIBS>`            | Maximum number of parallel library downloads.                                  | `150`   |
| `--aassets <AASSETS>`        | Maximum number of parallel asset downloads.                                    | `350`   |
| `-h, --help`                 | Print help.                                                                     | —       |

### Installation Components

By default, `rml install` installs **the client, libraries, and assets**.

If at least one of `--iclient`, `--ilibs`, or `--iassets` is specified, only the explicitly selected components will be installed.

For example:

```sh
rml install 26.2 --ilibs --iassets
```

### Examples

Install Minecraft version `26.2`:

    rml versions install 26.2

Install only the libraries and assets:

    rml versions install 26.2 --ilibs --iassets

Install only the client:

    rml versions install 26.2 --iclient

Install only the libraries:

    rml versions install 26.2 --ilibs

Install only the assets:

    rml versions install 26.2 --iassets

Perform a soft installation:

    rml versions install 26.2 --soft

Install using a locally downloaded version JSON file:

    rml versions install --json ./1.20.4.json

Specify a custom versions directory:

    rml versions install 26.2 --directory ./versions

Specify custom libraries and assets directories:

    rml versions install 26.2 --libs ./libraries --assets ./assets

Increase the maximum number of parallel library and asset downloads:

    rml versions install 26.2 --alibs 300 --aassets 500

Install only libraries and assets with custom download limits:

    rml versions install 26.2 --ilibs --iassets --alibs 300 --aassets 500
