//
//
// ----------------------------------------------------------------
//                       Nautilus CLI
// ----------------------------------------------------------------
//
//
use clap::Parser;

mod output;
mod processor;

use crate::processor::{processor, NautilusCommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: NautilusCommand,
}

fn main() -> std::io::Result<()> {
    processor(Cli::parse())
}
