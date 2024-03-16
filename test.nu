open Cargo.toml # assert

cd tests
nu generate-test-data.nu
cd ..

cd example-bundle
nu test-bundle.nu
cd ..

cargo test