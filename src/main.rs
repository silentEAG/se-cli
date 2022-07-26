mod config;

use anyhow::Result;
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

fn main() -> Result<()> {
    //    let args = Args::parse();
    //    for _ in 0..args.count {
    //        println!("Hello {}!", args.name);
    //    }
    let env_builder = config::ConfigBuilder::from_env();
    println!("{}", env_builder);
    Ok(())
}
