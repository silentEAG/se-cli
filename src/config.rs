#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use crate::utils::{get_env, get_env_bool};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::Vars, fmt::Display, str::FromStr};

pub static ENV_FILE: Lazy<String> = Lazy::new(|| match std::env::var("ENV_FILE") {
    Ok(value) => value,
    Err(_) => ".env".to_string(),
});

macro_rules! generate_config {
    ($(
        $(#[doc = $doc:literal])+
        $name:ident : $ty:ident, $editable:literal, $none_action:ident $(, $default:expr)?;
    )+) => {

        use serde::{Deserialize, Serialize};
        use std::{collections::HashMap};
        use crate::utils::{get_env, get_env_bool};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct ConfigItems {
            $(
                pub $name: generate_config!(@type $ty, $none_action),
            )+
        }

        #[derive(Serialize, Deserialize, Debug, Default)]
        struct BuilderItems {
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $name: Option<$ty>,
            )+
        }

        #[derive(Serialize, Deserialize, Debug, Default, Clone)]
        struct BuilderItemInfo {
            /// config item node name
            pub cfg_name: String,
            /// where config item defined
            pub come_from: String,
        }

        impl BuilderItemInfo {
            pub fn new (cfg_name: &str, come_from: &str) -> Self {
                BuilderItemInfo {
                    cfg_name: cfg_name.to_string(),
                    come_from: come_from.to_string(),
                }
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct ConfigBuilder {
            pub item_count: usize,
            pub builder_items: BuilderItems,
            pub builder_item_info_map: HashMap<String, BuilderItemInfo>
        }

        impl ConfigBuilder {

            /// init with nothing
            fn new() -> Self {
                let mut info = HashMap::new();
                let mut count: usize = 0;
                $(
                    count += 1;
                    info.insert(stringify!($name).to_string(), BuilderItemInfo::new(stringify!($name), "defalut"));
                )+
                ConfigBuilder {
                    item_count: count,
                    builder_item_info_map: info,
                    builder_items: BuilderItems {
                        $(
                            $name: None,
                        )+
                    }
                }
            }

            pub fn from_env() -> anyhow::Result<Self> {
                let mut cfg: ConfigBuilder = ConfigBuilder::new();
                $(
                    if let Some(value) = generate_config!(@getenv &stringify!($name).to_uppercase(), $ty) {
                        cfg.builder_items.$name = Some(value);
                        cfg.builder_item_info_map.insert(stringify!($name).to_string(),
                        BuilderItemInfo::new(stringify!($name), "env"));
                    }
                )+
                Ok(cfg)
            }

            pub fn from_file(path: &str) -> anyhow::Result<Self> {
                let mut cfg: ConfigBuilder = ConfigBuilder::new();
                use crate::utils::read_file_string;
                let config_str = read_file_string(path).expect("Read file failed.");
                let items: BuilderItems = serde_json::from_str(&config_str)?;
                $(
                    if let Some(value) = items.$name {
                        cfg.builder_items.$name = Some(value);
                        cfg.builder_item_info_map.insert(stringify!($name).to_string(),
                        BuilderItemInfo::new(stringify!($name), &format!("file://{}", path)));
                    }
                )+
                Ok(cfg)
            }

            pub fn _to_file(&self) {
                todo!();
            }

            pub fn merge(&mut self, cfg: Self) {
                $(
                    if let Some(val) = cfg.builder_items.$name {
                        self.builder_items.$name = Some(val);
                        self.builder_item_info_map.insert(stringify!($name).to_string(),
                            cfg.builder_item_info_map.get(stringify!($name)).unwrap().clone());
                    }
                )+
            }

            pub fn add_env(mut self) -> Self {
                let cfg = ConfigBuilder::from_env().unwrap();
                self.merge(cfg);
                self
            }
            pub fn add_file(mut self, path: &str) -> Self {
                let cfg = ConfigBuilder::from_file(path).unwrap();
                self.merge(cfg);
                self
            }

            pub fn build(&self) -> ConfigItems {
                ConfigItems {
                    $(
                        $name: generate_config!(@build self.builder_items.$name.clone(), $none_action $(, $default)?),
                    )+
                }
            }
        }

        /// load default config vars
        impl Default for ConfigBuilder {
            fn default() -> Self {
                let mut info = HashMap::new();
                let mut count: usize = 0;
                $(
                    count += 1;
                    info.insert(stringify!($name).to_string(), BuilderItemInfo::new(stringify!($name), "defalut"));
                )+
                ConfigBuilder {
                    item_count: count,
                    builder_item_info_map: info,
                    builder_items: BuilderItems {
                        $(
                            $name: generate_config!(@init $ty $(, $default)?),
                        )+
                    }
                }
            }
        }
    };

    (@type $ty:ident, option) => { Option<$ty> };
    (@type $ty:ident, $id:ident) => { $ty };

    (@build $value:expr, option) => { $value };
    (@build $value:expr, def, $default:expr) => { $value.unwrap_or($default) };

    (@init $ty:ident) => { None };
    (@init $ty:ident, $default:expr) => { Some($default) };

    (@getenv $name:expr, bool) => { get_env_bool($name) };
    (@getenv $name:expr, $ty:ident) => { get_env($name) };
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_all_env() {
        let args: Vars = std::env::vars();
        args.into_iter().for_each(|(key, value)| {
            println!("{} => {}", key, value);
        })
    }

    #[test]
    fn read_from_file() {
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
        };
        let builder = ConfigBuilder::from_file("config.json").unwrap();
        println!("{:?}", builder);
        let cfg = builder.build();
        println!("{:?}", cfg);
    }

    #[test]
    fn builder_test() -> anyhow::Result<()> {
        dotenvy::from_filename(&*ENV_FILE).ok();
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
            /// money
            money: i32, true, option;
        };

        let builder = ConfigBuilder::default().add_env().add_file("config.json");
        println!("{:?}", builder);
        let cfg = builder.build();
        println!("{:?}", cfg);
        assert_eq!(14, cfg.age);
        assert_eq!(true, cfg.has_neko);
        assert_eq!("SilentEEA".to_string(), cfg.name);
        Ok(())
    }
}
