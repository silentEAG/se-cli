#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::{collections::HashMap, env::Vars, fmt::Display};

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);

pub struct Config {}

impl Config {
    pub fn load() -> Self {
        todo!()
    }
}

pub struct ConfigBuilder {
    pub items: HashMap<String, String>,
    pub cfg_type: String,
}

impl ConfigBuilder {
    pub fn from_env() -> Self {
        let args: Vars = std::env::vars();
        let mut items: HashMap<String, String> = HashMap::new();
        args.into_iter().for_each(|(key, value)| {
            println!("{}: {}", key, value);
            items.insert(key, value);
        });
        let cfg_type = "env".to_string();
        ConfigBuilder { items, cfg_type }
    }
    pub fn from_file() -> Self {
        todo!()
    }

    pub fn from_arg() -> Self {
        todo!()
    }

    pub fn merge() -> Self {
        todo!()
    }
}

impl Display for ConfigBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.iter().for_each(|(key, value)| {
            writeln!(f, "{} => {}", key, value).expect("Failed to display ConfigBuilder.");
        });
        Ok(())
    }
}
