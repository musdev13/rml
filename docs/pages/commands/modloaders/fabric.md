```sh
rml modloaders fabric <COMMAND>
```

manage Fabric modloader versions

## fetch-list

```sh
rml modloaders fabric fetch-list [OPTIONS] <VERSION_ID>
```

fetch Fabric loader versions for a Minecraft version

### Arguments

| Argument       | Description                                      |
| -------------- | ------------------------------------------------ |
| `<VERSION_ID>` | Minecraft version ID (e.g., `26.2`).          |

### Options

| Option                | Description                                      | Default |
| --------------------- | ------------------------------------------------ | ------- |
| `-p, --page <PAGE>`   | Specifies the page to display.                  | `0`     |
| `-c, --count <COUNT>` | Specifies the number of versions to display.     | `10`    |
| `-j, --json`          | Outputs the result in JSON format.               | —       |
| `-h, --help`          | Displays help information.                       | —       |

### Examples

Fetch the latest 10 Fabric loader versions for Minecraft `26.2`:

```sh
rml modloaders fabric fetch-list 26.2
```

Fetch the latest 20 Fabric loader versions:

```sh
rml modloaders fabric fetch-list 26.2 --count 20
```

Fetch the second page of Fabric loader versions:

```sh
rml modloaders fabric fetch-list 26.2 --page 1
```

Output the versions in JSON format:

```sh
rml modloaders fabric fetch-list 26.2 --json
```

Fetch a specific page with a custom number of versions:

```sh
rml modloaders fabric fetch-list 26.2 --page 2 --count 5
```

## install

```sh
rml modloaders fabric install [OPTIONS] <VERSION_ID> <FABRIC_VERSION>
```

install a Fabric loader version

### Arguments

| Argument          | Description                                      |
| ----------------- | ------------------------------------------------ |
| `<VERSION_ID>`    | Minecraft version ID to install Fabric for.      |
| `<FABRIC_VERSION>` | Fabric loader version to install.                |

### Options

| Option                    | Description                                                        | Default |
| ------------------------- | ------------------------------------------------------------------ | ------- |
| `-v, --versions <VERSIONS_PATH>` | **Optional.** Path to the versions directory.              | —       |
| `-l, --libs <LIBS_PATH>`  | **Optional.** Path to the libraries directory.                      | —       |
| `-a, --assets <ASSETS_PATH>` | **Optional.** Path to the assets directory.                     | —       |
| `-h, --help`              | Print help.                                                        | —       |

### Examples

Install Fabric loader `0.19.3` for Minecraft `26.2`:

```sh
rml modloaders fabric install 26.2 0.19.3
```

Install Fabric with a custom versions directory:

```sh
rml modloaders fabric install 26.2 0.19.3 --versions ./versions
```

Install Fabric with custom libraries and assets directories:

```sh
rml modloaders fabric install 26.2 0.19.3 --libs ./libraries --assets ./assets
```

Install Fabric with all custom paths:

```sh
rml modloaders fabric install 26.2 0.19.3 --versions ./versions --libs ./libraries --assets ./assets
```
