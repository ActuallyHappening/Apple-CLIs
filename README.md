# Apple-CLIs
Collection of crates wrapping various Apple CLIs for documentation and type-safety purposes.

This is heavily a work in progress, and will be added to as I need more functionality for building a 100% Rust app for iOS.

## CLI
Latest released version:
```sh
cargo install apple-clis
```

Install from source:
```sh
cargo install --git https://github.com/ActuallyHappening/Apple-CLIs.git apple-clis

# if you are using nushell,
apple-clis init nushell --auto
```

## Examples
```sh
cargo run --example ios-deploy-detect
cargo run --example security-find-certificates
```

![Example `ios-deploy --detect`](docs/ios-deploy-detect.png)

## Contributions
PRs welcome!

### Developing
```sh
# try the crates.io release if it works
cargo install --git https://github.com/burtonageo/cargo-bundle.git
brew install ios-deploy

cd example-bundle
nu test-bundle
cd ..

# example command
cargo r -- codesign display
```

### Todo/Features:
Split into different crates for documentation purposes.
- [ ] simctl