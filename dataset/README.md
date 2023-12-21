## Install Rust
### MacOS / Linux
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Other Systems

`https://forge.rust-lang.org/infra/other-installation-methods.html`

# Install cargo-contract
Install the `cargo-contract` tool, which is a Cargo subcommand. You can install it using the following command:

```shell
cargo install cargo-contract --force
```

## Coinfabrik Scout
### Installation
In order to install the Command Line Interface, first install Scout dependencies by running the following command:

`cargo install cargo-dylint dylint-link`

Afterwards, install Scout with the following command:

`cargo install cargo-scout-audit`

### Usage
To run Scout on your project, navigate to its root directory and execute the following command:

`cargo scout-audit`

### Additional Steps
### May need to run:

```rustup component add rust-src --toolchain stable```

```rustup target add wasm32-unknown-unknown --toolchain stable```