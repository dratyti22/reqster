use crate::commands::cli::Cli;
use anyhow::Result;

pub fn run(cli: Cli) -> Result<()> {
    let url = cli.get_url();
    let method = cli.get_method();
    println!("URL {}", url);
    println!("URL {:?}", method);

    Ok(())
}
