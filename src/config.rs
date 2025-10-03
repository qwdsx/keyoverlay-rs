use directories::ProjectDirs;
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use rdev::Key;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct K {
    pub key: rdev::Key,
    pub label: String
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub keys: Vec<K>,
    pub key_size: usize,
    pub key_spacing: usize,
    pub scroll_speed: usize,
    pub active_color: usize,
    pub padding: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            keys: vec![
                K {
                    key: Key::KeyZ,
                    label: String::from("K1")
                },
                K {
                    key: Key::KeyX,
                    label: String::from("K2")
                }
            ],
            key_size: 40,
            key_spacing: 16,
            scroll_speed: 360,
            active_color: 0x808080,
            padding: 16,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<figment::Error>> {
        let proj_dirs =
            ProjectDirs::from("", "", "KeyOverlay").expect("Unable find home directory");
        let config_file = proj_dirs.config_dir().join("config.toml");

        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_file))
            .extract()
            .map_err(Box::new)
    }
}
