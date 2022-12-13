mod core;
mod source;

use clap::{ Parser, Subcommand };

use crate::core::{
    build,
    clean,
    deploy,
};

#[derive(Subcommand)]
pub enum NautilusCommand {
    /// Build your Solana program
    Build,
    /// Clean up your Cargo workspace
    Clean,
    /// Deploy your Solana program
    Deploy,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: NautilusCommand,
}

pub fn processor(cli: Cli) -> std::io::Result<()> {
    match &cli.command {
        NautilusCommand::Build => build(),
        NautilusCommand::Clean => clean(),
        NautilusCommand::Deploy => deploy(),
    }
}

fn main() -> std::io::Result<()> {
   processor(Cli::parse())
}