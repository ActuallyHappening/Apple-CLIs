# Apple-CLIs
Collection of crates wrapping various Apple CLIs for documentation and type-safety purposes.

This is heavily a work in progress, and will be added to as I need more functionality for building a 100% Rust app for iOS.

## CLI Install
The crates.io release is likely behind the development branch, but has a greater chance of working.
```sh
# install from crates.io
cargo install apple-clis
```

Install from source:
```sh
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

## Examples
```sh
cargo run --example ios-deploy-detect
cargo run --example security-find-certificates
cargo run --example simctl
```

![Example `ios-deploy --detect`](docs/ios-deploy-detect.png)

## Contributions
PRs welcome!

### Developing
```sh
# try the crates.io release if it works
cargo install --git https://github.com/burtonageo/cargo-bundle.git
brew install ios-deploy

# build an example bundle from the included example project
cd example-bundle
nu test-bundle
cd ..

# example command
cargo r -- codesign display
cargo r -- codesign sign
cargo r -- codesign display
```

### Todo/Features:
Split into different crates for documentation purposes.
- [ ] simctl