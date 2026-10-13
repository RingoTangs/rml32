# Repository Guidelines

## Project Structure & Module Organization

This repository is a starting point for analyzing `magic_Linux32` and attempting a Rust rewrite, as described in `README.md`.

- `magic_Linux32`: the reference executable, a dynamically linked 32-bit Intel x86 Linux ELF with debug information.
- `README.md`: the project purpose; expand it as setup and behavior become understood.

There is currently no Rust package, source tree, test suite, or asset directory. When introducing a Cargo project, use `src/` for implementation and `tests/` for integration tests. Keep analysis notes in `docs/` and small reproducible inputs in `tests/fixtures/`.

## Build, Test, and Development Commands

No build or test commands are configured yet. Useful initial inspection commands are:

- `file magic_Linux32`: identify the executable format and architecture.
- `readelf -h magic_Linux32`: inspect the ELF header where GNU binutils is available.

Running the reference executable requires a compatible Linux environment with the 32-bit loader `/lib/ld-linux.so.2` and its required libraries; it cannot run natively on macOS.

After adding `Cargo.toml`, document the actual setup and use `cargo build`, `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets` for compilation, tests, formatting, and linting.

## Coding Style & Naming Conventions

For new Rust code, use standard rustfmt formatting and four-space indentation. Use `snake_case` for functions and modules, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Separate observed executable behavior from assumptions in code comments and analysis notes.

## Testing Guidelines

No testing framework or coverage threshold is established. For the rewrite, use Rust's built-in test framework and descriptive names such as `parses_empty_input`. Add regression tests for confirmed behavior, recording input, expected output, and the reference environment. Run `cargo test` once a Cargo package exists.

## Commit & Pull Request Guidelines

No Git metadata is present, so commit conventions cannot be inferred. Use concise, imperative messages such as `Add input parsing regression tests`. Pull requests should explain the change, supporting behavioral evidence, validation performed, and remaining uncertainties. Link relevant issues when available.

## Reference Artifact Handling

Preserve `magic_Linux32` unchanged. Store generated analysis output separately, and avoid committing large dumps or environment-specific artifacts.

<!-- CODEGRAPH_START -->
## CodeGraph

In repositories indexed by CodeGraph (a `.codegraph/` directory exists at the repo root), reach for it BEFORE grep/find or reading files when you need to understand or locate code:

- **MCP tool** (when available): `codegraph_explore` answers most code questions in one call — the relevant symbols' verbatim source plus the call paths between them, including dynamic-dispatch hops grep can't follow. Name a file or symbol in the query to read its current line-numbered source. If it's listed but deferred, load it by name via tool search.
- **Shell** (always works): `codegraph explore "<symbol names or question>"` prints the same output.

If there is no `.codegraph/` directory, skip CodeGraph entirely — indexing is the user's decision.
<!-- CODEGRAPH_END -->
