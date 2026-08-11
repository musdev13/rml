```sh
rml modloaders neoforge <COMMAND>
```

manage NeoForge modloader versions

## fetch-list

```sh
rml modloaders neoforge fetch-list [OPTIONS] <VERSION_ID>
```

fetch NeoForge versions for a Minecraft version

### Arguments

| Argument       | Description                             |
| -------------- | --------------------------------------- |
| `<VERSION_ID>` | Minecraft version ID (e.g., `1.20.4`). |

### Options

| Option                | Description                                   | Default |
| --------------------- | --------------------------------------------- | ------- |
| `-p, --page <PAGE>`   | Specifies the page to display.               | `0`     |
| `-c, --count <COUNT>` | Specifies the number of versions to display.  | `10`    |
| `-j, --json`          | Outputs the result in JSON format.            | —       |
| `-h, --help`          | Displays help information.                    | —       |

### Examples

Fetch the latest 10 NeoForge versions for Minecraft `1.20.4`:

```sh
rml modloaders neoforge fetch-list 1.20.4
```

Fetch the latest 20 NeoForge versions:

```sh
rml modloaders neoforge fetch-list 1.20.4 --count 20
```

Fetch the second page of NeoForge versions:

```sh
rml modloaders neoforge fetch-list 1.20.4 --page 1
```

Output the versions in JSON format:

```sh
rml modloaders neoforge fetch-list 1.20.4 --json
```

Fetch a specific page with a custom number of versions:

```sh
rml modloaders neoforge fetch-list 1.20.4 --page 2 --count 5
```

## install

```sh
rml modloaders neoforge install [OPTIONS] <VERSION_ID> <NEOFORGE_VERSION>
```

install a NeoForge modloader version

### Arguments

| Argument            | Description                                |
| ------------------- | ------------------------------------------ |
| `<VERSION_ID>`      | Minecraft version ID to install NeoForge for. |
| `<NEOFORGE_VERSION>` | NeoForge version to install.              |

### Options

| Option                         | Description                                      | Default |
| ------------------------------ | ------------------------------------------------ | ------- |
| `-v, --versions <VERSIONS_PATH>` | **Optional.** Path to the versions directory. | —       |
| `-l, --libs <LIBS_PATH>`       | **Optional.** Path to the libraries directory. | —       |
| `-a, --assets <ASSETS_PATH>`   | **Optional.** Path to the assets directory.    | —       |
| `-h, --help`                   | Print help.                                     | —       |

### Examples

Install NeoForge for Minecraft `1.21.1`:

```sh
rml modloaders neoforge install 1.21.1 21.1.176
```

Install NeoForge with a custom versions directory:

```sh
rml modloaders neoforge install 1.21.1 21.1.176 --versions ./versions
```

Install NeoForge with custom libraries and assets directories:

```sh
rml modloaders neoforge install 1.21.1 21.1.176 --libs ./libraries --assets ./assets
```

Install NeoForge with all custom paths:

```sh
rml modloaders neoforge install 1.21.1 21.1.176 --versions ./versions --libs ./libraries --assets ./assets
```
