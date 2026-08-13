```sh
rml login ely [OPTIONS]
```

authenticate with an Ely.by account and optionally install the Ely.by Authlib patch.

### Options

| Option                  | Description                                                                                       | Default |
| ----------------------- | ------------------------------------------------------------------------------------------------- | ------- |
| `-a, --ask`             | Prompts interactively for the Ely.by login and password. After authentication, waits for a key press before displaying authentication data. Conflicts with `--login` and `--passwd`. | — |
| `-l, --login <LOGIN>`   | Ely.by account login. Required unless `--ask` or `--patch` is specified.                          | —       |
| `-p, --passwd <PASSWD>` | Ely.by account password. Required unless `--ask` or `--patch` is specified.                       | —       |
| `--patch`               | Installs the Ely.by Authlib patch.                                                                | —       |
| `--libs <LIBS_PATH>`    | **Optional.** Path to the libraries directory. Requires `--patch`.                               | —       |
| `-j, --json`            | Outputs authentication information in JSON format. Conflicts with `--ask`.                       | —       |
| `-h, --help`             | Displays help information.                                                                        | —       |

### Authentication

You can provide your credentials directly using `--login` and `--passwd`:

```sh
rml login ely --login Musek --passwd mypassword
```

When using this mode, the authentication information, including the access token, is displayed immediately after successful authentication.

Alternatively, use `--ask` to enter the credentials interactively:

```sh
rml login ely --ask
```

The password is entered securely without being displayed in the terminal.

When `--ask` is used, `rml` waits for a key press after successful authentication before displaying the authentication information. This prevents sensitive information such as the access token and client token from being displayed immediately after entering the password.

### JSON Output

Use `--json` to output the authentication result as formatted JSON:

```sh
rml login ely --login Musek --passwd mypassword --json
```

The JSON output is displayed immediately after successful authentication.

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

Authenticate using login and password. Authentication data is displayed immediately:

```sh
rml login ely --login Musek --passwd mypassword
```

Authenticate interactively. Press any key after successful authentication to display the authentication data:

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
