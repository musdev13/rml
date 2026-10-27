# Contributing to rml / Вклад в rml

---

## EN

Thanks for your interest in the project!

### Structure

`rml` is a CLI application built on top of `rmlib` (launcher logic) and `musutils` (shared helpers). Both are used as local path dependencies during development.

- `src/main.rs` — entry point: parses CLI, dispatches to command handlers or falls back to the TUI stub.
- `src/cli/mod.rs` — `clap` `Cli` struct (top-level arguments: subcommand, `--version`).
- `src/cli/handler.rs` — top-level dispatch. `handle_cli()` matches `Commands` and calls the appropriate `commands::<area>::<action>::handle/handler`. No business logic here.
- `src/cli/commands/` — one directory per command group, one file per action:
  - `init.rs`
  - `test.rs`
  - `run.rs`
  - `login/` — `ely.rs`, `mod.rs` — gated behind `#[cfg(feature = "ely")]`
  - `versions/` — `fetch_list.rs`, `install.rs`, `mod.rs`
  - `modloaders/` — gated behind `#[cfg(any(feature = "fabric", feature = "neoforge"))]`
    - `fabric/` (`fetch_list.rs`, `install.rs`, `mod.rs`) — `#[cfg(feature = "fabric")]`
    - `neoforge/` (`fetch_list.rs`, `install.rs`, `mod.rs`) — `#[cfg(feature = "neoforge")]`
    - `mod.rs`
  - `mod.rs` — the `Commands` enum (clap `Subcommand`) and re-exports of nested `Commands` enums.
- `src/tui/mod.rs` — TUI entry point. Currently a stub, launched when no CLI subcommand is given.
- `docs/` — MkDocs documentation (see `mkdocs.yml`).
- `Makefile` — convenience targets for checks, builds and dist packaging.

The CLI is a thin layer on top of `rmlib`. `rml` must not re-implement launcher logic (version resolution, downloads, classpath assembly, JVM spawning). If something is missing, add it to `rmlib` first, then wire it here.

### Rules

1. **All changes must be submitted to the `dev` branch.** Open a Pull Request against `dev`, not `main`.

2. **Keep `handler.rs` thin.** `handler.rs` only parses the top-level command and delegates. All per-command logic lives in `src/cli/commands/<area>/<action>.rs`.

3. **One file per command action.** Do not pile multiple unrelated commands into a single `mod.rs`. If `versions` gets a new action, add `src/cli/commands/versions/<action>.rs`, register it in `versions/mod.rs` (the `VersionsCommands` enum) and in `commands/mod.rs` (`Commands`), and handle it in `handler.rs`.

4. **Arguments live next to the handler.** Each action defines its own `#[derive(Args)]` struct in the same file as its `handle`/`handler` function. The parent `mod.rs` only declares `pub mod <action>;` and references `<action>::Args` from the parent enum.

5. **Use `rmlib` for launcher logic.** Installation, downloads, classpath, JVM arguments, auth — call into `rmlib` (`rmlib::core::...`, `rmlib::extra::ely::...`, `rmlib::extra::fabric::...`, `rmlib::extra::neoforge::...`). Do not duplicate its code in `rml`. If a helper is missing, add it to `rmlib` first.

6. **Use `musutils` for utilities.** Colored status output (`musutils::types::Status`), line drawing (`musutils::types::line`), pagination (`musutils::vec::get_page`), JSON tables (`musutils::vec::print_json_as_table`). Do not re-implement these.

7. **Optional features mirror `rmlib`.** `rml` has its own `[features]` table. Each optional area is a feature that forwards to the matching `rmlib` feature, and the affected CLI code is gated with `#[cfg(feature = "...")]`:

   ```toml
   [features]
   default = ["ely", "fabric", "neoforge"]
   ely = ["rmlib/ely"]
   fabric = ["rmlib/fabric"]
   neoforge = ["rmlib/neoforge"]
   ```

   What each feature enables:

   | Feature | CLI effect |
   |---|---|
   | `ely` | `rml login ely …` subcommand; `-e, --ely` flag on `rml run` |
   | `fabric` | `rml modloaders fabric …` subcommands; `--fabric` flag on `rml run` |
   | `neoforge` | `rml modloaders neoforge …` subcommands; `--neoforge` flag on `rml run` |

   Without a feature, the corresponding command / flag is **absent** from `rml --help`, and the underlying `rmlib` module is not compiled into the binary. `rml --no-default-features` is a minimal launcher: vanilla only, no `login`, no `modloaders`.

   When adding a new optional area, mirror the pattern: new `extra` module in `rmlib` → new feature in `rmlib` → matching feature in `rml` → `#[cfg(feature = "...")]`-gated commands and args.

8. **`rml --version` reflects the actual build.** The version output prints:
   ```
   rml <ver>
   rmlib <ver> (<modules>)
   musutils <ver> (<modules>)
   ```
   The module lists come from `rmlib::available_modules()` and `musutils::available_modules()`. Do not hardcode them. Do not change the output format — only the actual module lists change with the build. Building with `--no-default-features` should show only `core` in the `rmlib` list.

9. **Do not add new direct dependencies on `musutils` internals or `rmlib` internals.** Use the public API of each library. If something is missing, propose adding it to the library first.

10. **No unnecessary dependencies.** Prefer `std`. `clap`, `tokio`, `serde_json`, `rpassword`, `crossterm` are already in use. Add new crates only when genuinely useful.

11. **Keep user-facing behavior stable.** Command names, arguments, defaults and output format must not change without a documented reason. If a breaking CLI change is unavoidable, note it in the PR description and (if present) release notes.

12. **Run the checks before pushing:**

    ```bash
    make check
    ```

    `make check` runs:

    - `cargo fmt --check`
    - `cargo clippy --all-targets -- -D warnings`
    - `cargo clippy --all-targets --no-default-features -- -D warnings`
    - `cargo check --all-targets` (default: `ely` + `fabric` + `neoforge`)
    - `cargo check --all-targets --no-default-features`
    - `cargo check --all-targets --no-default-features --features ely`
    - `cargo check --all-targets --no-default-features --features fabric`
    - `cargo check --all-targets --no-default-features --features neoforge`

    All must be green. The individual targets are also available (`make fmt-check`, `make clippy`, `make clippy-no-default`, `make check-default`, `make check-no-default`, `make check-ely`, `make check-fabric`, `make check-neoforge`, `make check-all`).

13. **Building release artifacts.**

    ```bash
    make build           # Linux: target/release/rml
    make build-windows   # Windows: target/x86_64-pc-windows-gnu/release/rml.exe
    make dist            # both, then copies rml and rml.exe into dist/
    ```

    The Windows target requires `rustup target add x86_64-pc-windows-gnu` and an `x86_64-w64-mingw32-gcc` linker in `PATH` (e.g. the `mingw-w64-gcc` package on Arch). `make dist` produces `dist/rml` and `dist/rml.exe`.

14. **No tests in `rml`.** The CLI is a thin wrapper over `rmlib`; the launcher logic (and its tests) live in `rmlib`. `rml` only validates compilation: `cargo check` / `clippy` / `fmt`. If you add actual business logic to `rml` (which you generally should not — see rule 5), it must come with tests.

15. By submitting a Pull Request, you agree that your code will be distributed under the **MIT** license.

---

Questions and suggestions — in Issues.

---

## RU

Спасибо за интерес к проекту!

### Структура

`rml` — CLI-приложение поверх `rmlib` (логика лаунчера) и `musutils` (общие хелперы). Обе библиотеки подключаются локальными путями во время разработки.

- `src/main.rs` — точка входа: разбор CLI, диспетчеризация в обработчики команд либо fallback в TUI-заглушку.
- `src/cli/mod.rs` — структура `Cli` (`clap`) с верхнеуровневыми аргументами (подкоманда, `--version`).
- `src/cli/handler.rs` — верхнеуровневая диспетчеризация. `handle_cli()` матчит `Commands` и вызывает нужный `commands::<area>::<action>::handle/handler`. Бизнес-логики здесь нет.
- `src/cli/commands/` — каталог на группу команд, файл на действие:
  - `init.rs`
  - `test.rs`
  - `run.rs`
  - `login/` — `ely.rs`, `mod.rs` — под `#[cfg(feature = "ely")]`
  - `versions/` — `fetch_list.rs`, `install.rs`, `mod.rs`
  - `modloaders/` — под `#[cfg(any(feature = "fabric", feature = "neoforge"))]`
    - `fabric/` (`fetch_list.rs`, `install.rs`, `mod.rs`) — `#[cfg(feature = "fabric")]`
    - `neoforge/` (`fetch_list.rs`, `install.rs`, `mod.rs`) — `#[cfg(feature = "neoforge")]`
    - `mod.rs`
  - `mod.rs` — enum `Commands` (clap `Subcommand`) и реэкспорт вложенных enum-ов.
- `src/tui/mod.rs` — вход в TUI. Сейчас заглушка, запускается когда CLI-подкоманда не задана.
- `docs/` — документация MkDocs (см. `mkdocs.yml`).
- `Makefile` — цели для проверок, сборки и упаковки.

CLI — тонкий слой над `rmlib`. `rml` не должен заново реализовывать логику лаунчера (разрешение версий, загрузки, сборка classpath, спавн JVM). Если чего-то не хватает — сначала добавьте это в `rmlib`, затем используйте здесь.

### Правила

1. **Все изменения отправляются в ветку `dev`.** Pull Request открывается против `dev`, а не `main`.

2. **`handler.rs` остаётся тонким.** `handler.rs` только парсит верхнеуровневую команду и делегирует. Вся логика конкретных команд — в `src/cli/commands/<area>/<action>.rs`.

3. **Один файл — одно действие команды.** Не складывайте несколько несвязанных команд в один `mod.rs`. Если у `versions` появляется новое действие — добавьте `src/cli/commands/versions/<action>.rs`, зарегистрируйте его в `versions/mod.rs` (enum `VersionsCommands`) и в `commands/mod.rs` (enum `Commands`), обработайте в `handler.rs`.

4. **Аргументы — рядом с обработчиком.** Каждое действие определяет собственный `#[derive(Args)]`-struct в том же файле, что и функция `handle`/`handler`. Родительский `mod.rs` только объявляет `pub mod <action>;` и ссылается на `<action>::Args` из родительского enum-а.

5. **Логика лаунчера — через `rmlib`.** Установка, загрузки, classpath, JVM-аргументы, авторизация — вызывайте `rmlib` (`rmlib::core::...`, `rmlib::extra::ely::...`, `rmlib::extra::fabric::...`, `rmlib::extra::neoforge::...`). Не дублируйте его код в `rml`. Если хелпера нет — добавьте сначала в `rmlib`.

6. **Утилиты — через `musutils`.** Цветной вывод статусов (`musutils::types::Status`), рисование линий (`musutils::types::line`), пагинация (`musutils::vec::get_page`), таблицы JSON (`musutils::vec::print_json_as_table`). Не реализуйте их повторно.

7. **Опциональные фичи повторяют `rmlib`.** У `rml` своя таблица `[features]`. Каждая опциональная область — фича, которая прокидывает фичу `rmlib`, а затронутый CLI-код оборачивается `#[cfg(feature = "...")]`:

   ```toml
   [features]
   default = ["ely", "fabric", "neoforge"]
   ely = ["rmlib/ely"]
   fabric = ["rmlib/fabric"]
   neoforge = ["rmlib/neoforge"]
   ```

   Что включает каждая фича:

   | Фича | Эффект в CLI |
   |---|---|
   | `ely` | подкоманда `rml login ely …`; флаг `-e, --ely` у `rml run` |
   | `fabric` | подкоманды `rml modloaders fabric …`; флаг `--fabric` у `rml run` |
   | `neoforge` | подкоманды `rml modloaders neoforge …`; флаг `--neoforge` у `rml run` |

   Без фичи соответствующая команда/флаг **отсутствует** в `rml --help`, а соответствующий модуль `rmlib` не компилируется в бинарь. `rml --no-default-features` — минимальный лаунчер: только vanilla, без `login` и `modloaders`.

   При добавлении новой области паттерн повторяется: новый модуль в `rmlib/src/extra/` → новая фича в `rmlib` → соответствующая фича в `rml` → `#[cfg(feature = "...")]`-гейтинг команд/аргументов.

8. **`rml --version` отражает реальную сборку.** Формат вывода:
   ```
   rml <ver>
   rmlib <ver> (<modules>)
   musutils <ver> (<modules>)
   ```
   Списки модулей берутся из `rmlib::available_modules()` и `musutils::available_modules()`. Не хардкодьте их. Формат вывода не меняется — меняются только фактические списки модулей в зависимости от сборки. При сборке с `--no-default-features` в списке `rmlib` должен остаться только `core`.

9. **Не добавляйте прямых зависимостей от внутренностей `musutils`/`rmlib`.** Используйте публичный API каждой библиотеки. Если чего-то не хватает — предложите сначала добавить это в библиотеку.

10. **Никаких лишних зависимостей.** Предпочитайте `std`. `clap`, `tokio`, `serde_json`, `rpassword`, `crossterm` уже используются. Новые крейты добавляйте только при реальной пользе.

11. **Пользовательское поведение стабильно.** Имена команд, аргументы, дефолты и формат вывода не должны меняться без документированной причины. Если ломающее изменение CLI неизбежно — отразите это в описании PR и (если есть) в release notes.

12. **Перед пушем прогоняйте проверки:**

    ```bash
    make check
    ```

    `make check` прогоняет:

    - `cargo fmt --check`
    - `cargo clippy --all-targets -- -D warnings`
    - `cargo clippy --all-targets --no-default-features -- -D warnings`
    - `cargo check --all-targets` (default: `ely` + `fabric` + `neoforge`)
    - `cargo check --all-targets --no-default-features`
    - `cargo check --all-targets --no-default-features --features ely`
    - `cargo check --all-targets --no-default-features --features fabric`
    - `cargo check --all-targets --no-default-features --features neoforge`

    Всё должно быть зелёным. Отдельные цели тоже доступны (`make fmt-check`, `make clippy`, `make clippy-no-default`, `make check-default`, `make check-no-default`, `make check-ely`, `make check-fabric`, `make check-neoforge`, `make check-all`).

13. **Сборка релизных артефактов.**

    ```bash
    make build           # Linux: target/release/rml
    make build-windows   # Windows: target/x86_64-pc-windows-gnu/release/rml.exe
    make dist            # обе сборки + копия rml и rml.exe в dist/
    ```

    Для Windows-таргета нужны `rustup target add x86_64-pc-windows-gnu` и линкер `x86_64-w64-mingw32-gcc` в `PATH` (например, пакет `mingw-w64-gcc` в Arch). `make dist` кладёт в `dist/` файлы `rml` и `rml.exe`.

14. **В `rml` нет тестов.** CLI — тонкая обёртка над `rmlib`; логика лаунчера (и её тесты) живёт в `rmlib`. `rml` проверяет только сборку: `cargo check` / `clippy` / `fmt`. Если вы добавляете в `rml` реальную бизнес-логику (чего вообще-то делать не стоит — см. правило 5), она должна идти с тестами.

15. Отправляя Pull Request, вы соглашаетесь, что ваш код будет распространяться под лицензией **MIT**.

---

Вопросы и предложения — в Issues.