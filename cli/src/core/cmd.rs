use crate::core::config::execute_command;

pub fn build() {
    println!();
    println!();
    execute_command("cargo build-bpf");
}

pub fn clean() {
    execute_command("cargo clean");
}

pub fn deploy() {
    execute_command("solana program deploy ./target/deploy/program_native.so");
}