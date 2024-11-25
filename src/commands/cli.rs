use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[arg(short, long, help = "url to which the request is sent")]
    url: String,
}

impl Cli {
    pub fn get_url(&self) -> &str {
        &self.url
    }
}
