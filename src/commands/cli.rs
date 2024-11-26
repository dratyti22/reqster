use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[arg(short, long, help = "url to which the request is sent")]
    url: String,

    #[arg(
        short,
        long,
        help = "method to use for the request (GET, POST, PUT, DELETE)",
        required = false,
        default_value = "get"
    )]
    method: HttpMethod,
}

impl Cli {
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }
}

#[derive(Parser, Debug, ValueEnum, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}
