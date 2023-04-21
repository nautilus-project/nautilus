echo "\n\n================================================="
echo "\n             Nautilus Program Tests"
echo "\n\n================================================="
sleep 2 

echo "\nDeploying all test programs...\n"
sleep 5
cargo build-sbf --manifest-path="./programs/source-basic/Cargo.toml"
cargo build-sbf --manifest-path="./programs/source-create/Cargo.toml"
# cargo build-sbf --manifest-path="./programs/source-robust/Cargo.toml"
solana program deploy ./programs/source-basic/target/deploy/program_nautilus.so
solana program deploy ./programs/source-create/target/deploy/program_nautilus.so
# solana program deploy ./programs/source-robust/target/deploy/program_nautilus.so

echo "\nCommencing all tests...\n"
yarn
sleep 3

echo "\nLaunching test suite: Source Basic\n"
yarn run test-source-basic
sleep 5

echo "\nLaunching test suite: Source Create\n"
yarn run test-source-create
sleep 5

# echo "\nLaunching test suite: Source Robust\n"
# yarn run test-source-robust
# sleep 5