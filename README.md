# cargo-assemble

It's yet another day, and you list the members you want in the `Cargo.toml` file of your Rust workspace. Then you suddenly realize, you're stuck running `cargo new` for each missing member package.
*You sigh.*

`cargo-assemble` eliminates this tedious workflow by creating missing member packages on demand from the workspace `Cargo.toml`. Once the workspace members are defined, run `cargo assemble`, and see it smart-generate the missing crates as either `bin` or `lib`, depending on the flags used.

## Installation

Install from [crates.io](https://crates.io/crates/cargo-assemble):

```sh
cargo install cargo-assemble
```

This installs the binary to Cargo’s bin directory (`~/.cargo/bin` in most cases), making it available as a Cargo subcommand.

## Usage

**By default**, `cargo-assemble` creates all missing members as binary (`bin`) packages:

```sh
cargo assemble
```

### Controlling Package Types

**Given**, an example workspace `Cargo.toml` that contains these members:

```toml
[workspace]
members = ["server", "cmd", "core", "database", "auth"]
```

If specific members are flagged using the `--lib` (or `-l`) flag, they'll be created as library crates, and the rest default as binary crates:

```sh
cargo assemble --lib core utils
```

**However**, if some members are explicitly flagged using `--bin` (or `-b`), they'll be created as binary crates, and the remaining members shall be library crates for ease of use:

```sh
cargo assemble --bin server cmd
```

**In a nutshell**,

* `cargo assemble` → Missing members in the directory are (all) created as `bin` crates.
* `cargo assemble --lib core database auth` → `core`, `database`, and `auth` are created as `lib` crates; `server` and `cmd` as `bin` crates.
* `cargo assemble --bin cmd` → `cmd` is created as a `bin` crate; the rest (`server`, `core`, etc.) as `lib` crates.

## License

This project is licensed under the [MIT License](LICENSE).