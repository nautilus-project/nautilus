use clap::Subcommand;
use std::process::Command;
use termcolor::Color;

use crate::output::NautilusTerminal;
use crate::Cli;

#[derive(Subcommand)]
pub enum NautilusCommand {
    /// 🛠️ Builds the Nautilus program
    Build,
    /// ⛴️ Ships (deploys) the Nautilus program
    Deploy,
    /// ⛴️ Ships (deploys) the Nautilus program
    Ship,
}

/// Util function for running commands on the operating system.
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

/// Build the Nautilus program.
fn build() -> std::io::Result<()> {
    os_command("cargo build-sbf")?;
    Ok(())
}

/// Deploy the Nautilus program using locally set configs.
fn deploy() -> std::io::Result<()> {
    os_command("solana config get")?;
    os_command("solana program deploy target/deploy/*.so")?;
    Ok(())
}

/// Process incoming commands to the Nautilus CLI.
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
        NautilusCommand::Ship => {
            let mut terminal =
                NautilusTerminal::new(Color::Yellow, " ⛴️  Shipping Nautilus program...");
            match deploy() {
                Ok(()) => {
                    terminal.output(Color::Green, "   ✅  Deploy successful.");
                    terminal.end_output(
                        Color::Green,
                        "   ⛴️  You just shipped a Solana program with Nautilus!",
                    );
                }
                Err(_) => terminal.end_output(Color::Red, "   ❌  Deploy failed."),
            };
        }
    };
    Ok(())
}
