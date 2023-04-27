echo "\n\n================================================="
echo "\n             Nautilus Program Tests"
echo "\n\n================================================="
sleep 2 

echo "\nBuilding all test programs...\n"
# sleep 5
# cargo build-sbf --manifest-path="./programs/create-source/Cargo.toml"
cargo build-sbf --manifest-path="./programs/create-records/Cargo.toml"
echo "\nDeploying all test programs...\n"
# solana program deploy ./programs/create-source/target/deploy/program_nautilus.so
solana program deploy ./programs/create-records/target/deploy/program_nautilus.so

# echo "\nCommencing all tests...\n"
# yarn
# sleep 3

# echo "\nLaunching test suite: Create Source\n"
# yarn run test-create-source
# sleep 5

echo "\nLaunching test suite: Create Records\n"
yarn run test-create-records
sleep 5