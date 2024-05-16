# cargo install toml-cli
# cargo install nu_plugin_semver
plugin add $"($env.HOME)/.cargo/bin/nu_plugin_semver"


cargo update
nu test.nu

# ideally get it to work, see https://github.com/abusch/nu_plugin_semver/issues/3
# let current_version = open "Cargo.toml" | get package.version | into semver
# let new_version = $current_version | semver bump patch
# let v = $new_version | into string
# let new_cargo_toml = toml set Cargo.toml package.version $v

# new_cargo_toml | save -f "Cargo.toml"

let cv = open "Cargo.toml" | get package.version | parse "{maj}.{min}.{patch}"
let patch = $cv.0.patch | into int
# update the patch version
let nv = $"($cv.0.maj).($cv.0.min).($patch + 1)"
let nt = toml set Cargo.toml package.version $nv
$nt | save --force "Cargo.toml"

git commit -am $"feat\(v($nv)): Publishing changes"

cargo publish

git commit -am "chore: Cargo.lock update"