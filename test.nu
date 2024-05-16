open Cargo.toml # assert

cd tests
nu generate-test-data.nu
cd ..

cd example-bundle
nu test-bundle.nu
cd ..

cargo t

# various regressions
cargo r -- ios-deploy detect --wifi