echo "\n\n================================================="
echo "\n             Nautilus Program Tests"
echo "\n\n================================================="
sleep 2 

echo "\nBuilding all test programs...\n"
sleep 5
cargo build-sbf --manifest-path="./programs/wallets/Cargo.toml"
cargo build-sbf --manifest-path="./programs/tokens/Cargo.toml"
cargo build-sbf --manifest-path="./programs/records/Cargo.toml"
echo "\nDeploying all test programs...\n"
solana program deploy ./programs/wallets/target/deploy/program_nautilus.so
solana program deploy ./programs/tokens/target/deploy/program_nautilus.so
solana program deploy ./programs/records/target/deploy/program_nautilus.so

echo "\nCommencing all tests...\n"
yarn
sleep 3

echo "\nLaunching test suite: Wallets\n"
yarn run test-wallets
sleep 5

echo "\nLaunching test suite: Tokens\n"
yarn run test-tokens
sleep 5

echo "\nLaunching test suite: Records\n"
yarn run test-records
sleep 5