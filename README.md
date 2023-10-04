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
<code style="white-space:nowrap;">
```
cargo install cargo-edit
# cargo add package         // --dev for dev-env
# cargo rm package
# cargo upgrade package
```
</code>
</details>


<details> 
<summary>Install Monitoring Software (cargo-watch)</summary>
<code style="white-space:nowrap;">
cargo install cargo-watch
# cargo watch -x run        // run
# cargo watch -x test       // test
# cargo watch -x check      // compiler
# cargo watch -x fmt        // formatter
# cargo watch -x clippy     // linter
</code>
</details>

<details> 
<summary>Install Formatter</summary>
<code style="white-space:nowrap;">
rustup component add rustfmt
# cargo fmt
</code>
</details>


<details> 
<summary>Install Linter (Clippy)</summary>
<code style="white-space:nowrap;">
rustup component add clippy
# cargo clippy
</code>
</details>
