```sh
rml login ely [OPTIONS]
```

authenticate with an Ely.by account and optionally install the Ely.by Authlib patch.

### Options

| Option                  | Description                                                                                       | Default |
| ----------------------- | ------------------------------------------------------------------------------------------------- | ------- |
| `-a, --ask`             | Prompts interactively for the Ely.by login and password. Conflicts with `--login` and `--passwd`. | —       |
| `-l, --login <LOGIN>`   | Ely.by account login. Required unless `--ask` or `--patch` is specified.                          | —       |
| `-p, --passwd <PASSWD>` | Ely.by account password. Required unless `--ask` or `--patch` is specified.                       | —       |
| `--patch`               | Installs the Ely.by Authlib patch.                                                                | —       |
| `--libs <LIBS_PATH>`    | **Optional.** Path to the libraries directory. Requires `--patch`.                               | —       |
| `-j, --json`            | Outputs authentication information in JSON format. Conflicts with `--ask`.                       | —       |
| `-h, --help`            | Displays help information.                                                                        | —       |

### Authentication

You can provide your credentials directly using `--login` and `--passwd`:

```sh
rml login ely --login Musek --passwd mypassword
```

Alternatively, use `--ask` to enter the credentials interactively:

```sh
rml login ely --ask
```

The password is entered securely without being displayed in the terminal.

### JSON Output

Use `--json` to output the authentication result as formatted JSON:

```sh
rml login ely --login Musek --passwd mypassword --json
```

### Authlib Patch

Use `--patch` to install the Ely.by Authlib patch:

```sh
rml login ely --patch
```

Specify a custom libraries directory:

```sh
rml login ely --patch --libs ./libraries
```

### Examples

Authenticate using login and password:

```sh
rml login ely --login Musek --passwd mypassword
```

Authenticate interactively:

```sh
rml login ely --ask
```

Authenticate and output the result as JSON:

```sh
rml login ely --login Musek --passwd mypassword --json
```

Install the Ely.by Authlib patch:

```sh
rml login ely --patch
```

Install the patch using a custom libraries directory:

```sh
rml login ely --patch --libs ./libraries
```
