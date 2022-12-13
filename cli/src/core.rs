use std::process::{ Command, Stdio };
use crate::source::{
    build_program_entrypoint,
    strip_program_entrypoint,
    try_read_program_name,
};

pub fn get_cli_configs() -> (&'static str, &'static str) {
    let (mut cmd_shell, mut cmd_prefix) = ("sh", "-c");
    if cfg!(target_os = "windows") {
        (cmd_shell, cmd_prefix) = ("cmd", "/C");
    };
    return (cmd_shell, cmd_prefix)
}

pub fn execute_command(args: &str) -> std::io::Result<()> {
    let (cmd_shell, cmd_prefix) = get_cli_configs();
    let mut cmd = Command::new(cmd_shell)
        .args([cmd_prefix, args])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command.");
    cmd.wait().unwrap();
    Ok(())
}

pub fn build() -> std::io::Result<()> {
    println!();
    println!();
    build_program_entrypoint()?;
    // execute_command("cargo build-bpf")?;
    strip_program_entrypoint()?;
    Ok(())
}

pub fn clean() -> std::io::Result<()> {
    println!();
    println!();
    execute_command("cargo clean")
}

pub fn deploy() -> std::io::Result<()> {
    println!();
    println!();
    execute_command(
        format!(
            "solana program deploy {}",
            try_read_program_name().unwrap().as_str(), 
        ).as_str()
    )
}