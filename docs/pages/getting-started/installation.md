# Java
**You need to install JRE/JDK any version (depends on minecraft version)**

---

# Installation
## Releases
You can download pre-built binaries for **Windows** and **Linux**. *MacOS users can build from source code*

- **GitHub -** [**Link**](https://github.com/musdev13/rml/releases)

---

## Build
### Depends
- **rust/cargo**
### Preparing
first you need to create new directory and cd to it.

after that you need to clone 2 libraries:

#### musutils
includes some base utils for my projects
```sh
git clone --depth 1 https://github.com/musdev13/musutils.git
```

#### rmlib
main library that includes launcher logic
```sh
git clone --depth 1 https://github.com/musdev13/rmlib.git
```

### rml build
clone stable **main** branch from **GitHub** repo:
```sh
git clone --depth 1 https://github.com/musdev13/rml.git
```
change directory
```sh
cd rml
```
and build
```sh
cargo build --release
```