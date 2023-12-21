# Installation Instructions
## Install Rust
### MacOS / Linux
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows / Other Systems

[https://forge.rust-lang.org/infra/other-installation-methods.html](https://forge.rust-lang.org/infra/other-installation-methods.html)

## Install libraries
### cargo contract
Install the `cargo-contract` tool, which is a Cargo subcommand. You can install it using the following command:

```shell
cargo install cargo-contract --force
```

### Additional Steps
You may need to run:

```rustup component add rust-src --toolchain stable```

```rustup target add wasm32-unknown-unknown --toolchain stable```

## Coinfabrik Scout
### Installation
In order to install the Command Line Interface, first install Scout dependencies by running the following command:

```cargo install cargo-dylint dylint-link```

Afterwards, install Scout with the following command:

```cargo install cargo-scout-audit```

### Usage
To run Scout on your project, navigate to its root directory and execute the following command:

```cargo scout-audit```

Print additional information on run

```cargo scout-audit --verbose	```

Sets the output format. Selecting json, html or sarif will create a file with the output

```cargo scout-audit --output-format [text\|json\|html\|sarif]```
