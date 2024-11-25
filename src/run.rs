use crate::commands::cli::Cli;
use anyhow::Result;

pub fn run(cli: Cli) -> Result<()> {

    let url = cli.get_url();
    println!("URL {}", url);

    Ok(())
}
