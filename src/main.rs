// src/main.rs
/*
 * Main executable for GPTTestnetSystemAINext
 */

use clap::Parser;
use gpttestnetsystemainext::{Result, run};

#[derive(Parser)]
#[command(version, about = "GPTTestnetSystemAINext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
