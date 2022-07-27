#[macro_use]
mod config;
mod test;
mod utils;

use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;

use crate::config::ENV_FILE;

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

generate_config! {
    /// What's your name?
    name: String, true, def, "SilentE".to_string();
    /// Test env var by RUSTUP_HOME.
    rustup_home: String, true, def, "/home/.rustup".to_string();
    /// How old are you?
    age: i32, true, def, 19;
    /// Is a student?
    is_student: bool, true, def, true;
    /// Have a neko?
    has_neko: bool, true, def, false;
    /// Have a dog?
    has_dog: bool, true, def, false;
}

pub static CONFIG: Lazy<ConfigItems> = Lazy::new(|| {
    // TODO
    ConfigBuilder::default()
        .add_env()
        .unwrap()
        .add_file("config.json")
        .unwrap()
        .build()
});

fn main() -> Result<()> {
    dotenvy::from_filename(&*ENV_FILE).ok();
    println!("{:?}", *CONFIG);
    Ok(())
}
