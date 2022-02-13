### Brix

Brix is a CLI tool written in Rust for scaffolding and code generation.

#### Attributions

Special thanks to [Caleb Cushing](https://github.com/xenoterracide) for the original Java version, early interface design and internal architecture.

#### Installation

Brix is available on [crates.io](https://crates.io/crates/brix) and the [AUR](https://aur.archlinux.org/packages/brix-git) for Arch Linux.

Install with cargo:
```
cargo install brix
```

Arch Linux (use an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers) like `yay` or `trizen`)
```
yay -S brix-git
```

#### Running

Usage:

```
brix [LANGUAGE] [CONFIG NAME] [PROJECT] [MODULE]
brix [OPTIONS] --config-dir | -d [CONFIG DIRECTORY]
brix [OPTIONS] --workdir | -w [WORKING DIRECTORY]
```

#### Building locally

##### Requirements

- Cargo and a minimum Rust version of **1.43.1**

##### Running

- Run `cargo build`
- Run `cargo run`

##### Testing

Run `cargo test --all` to test the entire workspace.

##### Docs

Run `cargo doc --no-deps --workspace --document-private-items --open`

##### Examples

There are a few examples located in `./config/brix/rust`.

- **copy** `cargo run -- rust copy brix foo`
- **exec** `cargo run -- rust exec foo foo`
- **mkdir** `cargo run -- rust mkdir brix foo`
- **search_replace** `cargo run -- rust search_replace brix foo`
- **template** `cargo run -- rust template brix foo`
