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
cargo install --git https://github.com/ActuallyHappening/cargo-bundle.git
brew install ios-deploy
cd example-bundle
nu test-bundle
cd ..

# example command
cargo r -- codesign display
```