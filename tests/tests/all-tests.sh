echo "\n\n================================================="
echo "\n             Nautilus Program Tests"
echo "\n\n================================================="
sleep 2

echo "\nBuilding all test programs...\n"
sleep 5
cargo build-sbf --manifest-path="./programs/wallets/Cargo.toml"
cargo build-sbf --manifest-path="./programs/tokens/Cargo.toml"
cargo build-sbf --manifest-path="./programs/records/Cargo.toml"
cargo build-sbf --manifest-path="./programs/accounts/Cargo.toml"
echo "\nDeploying all test programs...\n"
solana program deploy ./programs/wallets/target/deploy/program_nautilus.so
solana program deploy ./programs/tokens/target/deploy/program_nautilus.so
solana program deploy ./programs/records/target/deploy/program_nautilus.so
solana program deploy ./programs/accounts/target/deploy/program_nautilus.so

echo "\nCommencing all tests...\n"
npm
sleep 3

echo "\nLaunching test suite: Wallets\n"
npm run test-wallets
sleep 5

echo "\nLaunching test suite: Tokens\n"
npm run test-tokens
sleep 5

echo "\nLaunching test suite: Records\n"
npm run test-records
sleep 5

echo "\nLaunching test suite: Accounts\n"
npm run test-accounts
sleep 5
