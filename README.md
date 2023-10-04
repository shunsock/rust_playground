# rust_playground

## Setup

### Install Rust
```sh
brew install rustup-init
rustup-init

# check versions
rustup --version
rustc --version
cargo --version
```

### Install Developer Tool
Shell Script for Installing Developer Tools
```sh
bash install_dev_tools.sh
```


<details>
<summary>Install Package Manager (cargo-edit)</summary>
```sh
cargo install cargo-edit
# cargo add package         // --dev for dev-env
# cargo rm package
# cargo upgrade package
```
</details>


<details>
<summary>Install Monitoring Software (cargo-watch)</summary>

```sh
cargo install cargo-watch
# cargo watch -x run        // run
# cargo watch -x test       // test
# cargo watch -x check      // compiler
# cargo watch -x fmt        // formatter
# cargo watch -x clippy     // linter
```
</details>

<details>
<summary>Install Formatter</summary>

```sh
rustup component add rustfmt
# cargo fmt
```
</details>


<details>
<summary>Install Linter (Clippy)</summary>

```sh
rustup component add clippy
# cargo clippy
```
</details>
