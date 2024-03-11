# This script is used to build the example-bundle for iOS

cargo bundle --target aarch64-apple-ios
rm -rf bundle
mkdir bundle
mv ./target/aarch64-apple-ios/debug/bundle/ios/example-bundle.app ./bundle