use core::usize;

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    pub vga: VgaConf,
}

#[derive(Deserialize)]
struct VgaConf {
    pub buffer_width: usize,
    pub buffer_height: usize,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment)
            .build()?
            .try_deserialize()
    }
}
