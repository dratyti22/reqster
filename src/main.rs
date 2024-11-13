use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[arg(short, long, help = "url to which the request is sent")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    println!("{:#?}", cli);
    Ok(())
}
