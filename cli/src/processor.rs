use clap::Subcommand;
use std::process::Command;
use termcolor::Color;

use crate::output::NautilusTerminal;
use crate::Cli;

#[derive(Subcommand)]
pub enum NautilusCommand {
    Build,
    Deploy,
}

fn os_command(cmd: &str) -> std::io::Result<()> {
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", cmd]).spawn()
    } else {
        Command::new("sh").arg("-c").arg(cmd).spawn()
    }?;
    match cmd.wait()?.success() {
        true => Ok(()),
        false => Err(std::io::Error::last_os_error()),
    }
}

fn build() -> std::io::Result<()> {
    os_command("cargo build-bpf")?;
    os_command("shank idl -p $(solana address --keypair target/deploy/*.json)")?;
    Ok(())
}

fn deploy() -> std::io::Result<()> {
    os_command("solana config get")?;
    os_command("solana program deploy target/deploy/*.so")?;
    Ok(())
}

pub fn processor(cli: Cli) -> std::io::Result<()> {
    match &cli.command {
        NautilusCommand::Build => {
            let mut terminal =
                NautilusTerminal::new(Color::Cyan, " ⛴️  Building Nautilus program...");
            match build() {
                Ok(()) => terminal.end_output(Color::Green, "   ✅  Build completed."),
                Err(_) => terminal.end_output(Color::Red, "   ❌  Build failed."),
            };
        }
        NautilusCommand::Deploy => {
            let mut terminal =
                NautilusTerminal::new(Color::Yellow, " ⛴️  Deploying Nautilus program...");
            match deploy() {
                Ok(()) => terminal.end_output(Color::Green, "   ✅  Deploy successful."),
                Err(_) => terminal.end_output(Color::Red, "   ❌  Deploy failed."),
            };
        }
    };
    Ok(())
}
