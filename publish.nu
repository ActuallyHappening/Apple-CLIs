# deps
# cargo install toml-cli
# cargo install nu_plugin_semver
plugin add $"($env.HOME)/.cargo/bin/nu_plugin_semver"

# test suite
cargo update
nu test.nu

# bumps patch version
let current_version = open "Cargo.toml" | get package.version | into semver
let new_version = $current_version | semver bump patch
let nv = $new_version | to text
let new_cargo_toml = toml set Cargo.toml package.version $nv
new_cargo_toml | save -f "Cargo.toml"


# publishes to crates.io
git commit -am $"feat\(v($nv)): Publishing changes"
cargo publish
git commit -am "chore: Cargo.lock update"