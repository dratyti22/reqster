mod commands;
mod run;

use crate::commands::cli::Cli;
use crate::run::run;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match run(cli) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
