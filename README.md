### Brix
Brix is a CLI tool written in Rust for scaffolding and code generation.
#### Running
Usage:
```
brix [LANGUAGE] [CONFIG NAME] [PROJECT] [MODULE]
brix [OPTIONS] --config-dir | -d [CONFIG DIRECTORY]
brix [OPTIONS] --workdir | -w [WORKING DIRECTORY]
```

#### Installing locally
- Clone the repository and ensure you have the Rust toolchain and Cargo installed
- Run `cargo build`
- Run `cargo run`

##### Testing
Run `cargo test --all` to test the entire workspace.

##### Examples
There are a few examples located in `./config/brix/rust`.

- **copy** `cargo run -- rust copy brix foo`
- **mkdir** `cargo run -- rust mkdir brix foo`
- **search_replace** `cargo run -- rust search_replace brix foo`
- **template** `cargo run -- rust template brix foo`
