# cargo-assemble

`cargo-assemble` reads a workspace's `Cargo.toml` file and creates missing member packages that are listed but don't exist in the workspace root. It supports creating both `bin` or `lib` packages depending on the user's preferences.

## Usage

By default, `cargo-assemble` creates members as binary packages. However, the type of package to be created can be controlled using the respective flags.

For example, this command via `--lib` (or `-l`) creates `member-1` and `member-2` as library crates, but the rest of the listed members as binary ones:

```sh
cargo assemble --lib member-1 member-2
```

On the other hand, if `--bin` (or `-b`) is explicitly specified, it creates binary crates for those members, but the rest default to library ones:

```sh
cargo assemble --bin member-3 member-4
```

This makes usage easier by marking exceptions and defaulting the rest to the other type.

### Examples

Given a workspace `Cargo.toml` with these members:
```toml
[workspace]
members = ["api", "cli", "core", "utils"]
```

- `cargo assemble` - Creates all missing members as binary packages
- `cargo assemble --lib core utils` - Creates `core` and `utils` as libraries, `api` and `cli` as binaries
- `cargo assemble --bin cli` - Creates `cli` as a binary, others as libraries

## Installation

To install the latest published version from [crates.io](https://crates.io/crates/cargo-assemble):

```sh
cargo install cargo-assemble
```

This builds and installs the `cargo-assemble` binary into Cargo's `bin` directory (typically `~/.cargo/bin`), making it available as a Cargo subcommand.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
