mod commands;

use crate::commands::cli::Cli;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    println!("{:#?}", cli);
    Ok(())
}
