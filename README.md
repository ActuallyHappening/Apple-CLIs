# Apple-CLIs
Collection of crates wrapping various Apple CLIs for documentation and type-safety purposes.

This is heavily a work in progress, and will be added to as I need more functionality for building a 100% Rust app for iOS.

## CLI Install
The crates.io release is likely behind the development branch, but has a greater chance of working.
```sh
# install from crates.io if it works
cargo install apple-clis
```

Install from source:
```sh
# install from git if crates.io doesn't work / missing features
cargo install --git https://github.com/ActuallyHappening/Apple-CLIs.git apple-clis
```

### Setup with NuShell
If you want completions for your shell, make an issue / PR and I'll add it.
Since I use NuShell (which is really awesome and built in Rust), I've added a command to automatically add completions for NuShell.
```zsh
# if you are using nushell,
apple-clis init nushell --auto

# if you want more control,
apple-clis init nushell --raw-script
```

## Run Rust Examples
```sh
# clone repo
git clone https://github.com/ActuallyHappening/Apple-CLIs.git
cd Apple-CLIs

cargo run --example ios-deploy-detect
cargo run --example security-find-certificates
cargo run --example simctl
```

## Examples using NuShell
### Ios-Deploy detect
```sh
apple-clis ios-deploy detect --json | from json

cargo run --example ios-deploy-detect
```
![apple-clis ios-deploy detect --machine | from json](docs/ios-deploy-detect-nu.png)

### xcrun simctl list
```sh
apple-clis xcrun simctl list --json | from json | get device_type_identifier
```
<!-- TODO: Add documentation examples -->

## Example build script
This uses `cargo bundle`, which you can install with `cargo install cargo-bundle`, and nushell, as an example script to build an iOS app.
```sh
# example Cargo.toml
# [package.metadata.bundle]
# identifier = "com.example-id"
let BUNDLE_ID = open Cargo.toml | get package.metadata.bundle.identifier | to text

cargo bundle --target aarch64-apple-ios-sim
apple-clis codesign sign --glob
apple-clis xcrun simctl boot --ipad
apple-clis open --well-known simulator
apple-clis xcrun simctl install --booted --glob
apple-clis xcrun simctl launch --booted --bundle-id $BUNDLE_ID
```

## Contributions
PRs welcome!

### Developing
```sh
# try the crates.io release if it works
cargo install --git https://github.com/burtonageo/cargo-bundle.git
brew install ios-deploy
cargo install nu # nushell is really awesome

# build an example bundle from the included example project + run tests
nu test.nu
```