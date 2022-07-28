#[macro_use]
mod config;
mod test;
mod tokio_learn;
mod utils;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() {
    tokio_learn::tk_main().await;
}
