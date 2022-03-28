use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path::Path};

const NAME: &'static str = "cosmic-theme";
const CONFIG_NAME: &'static str = "config.ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    light: String,
    dark: String,
}

impl Config {
    pub fn new(light: String, dark: String) -> Self {
        Self { light, dark }
    }

    pub fn save(&self) -> Result<()> {
        if let Some(mut p) = dirs::home_dir() {
            p.push(".config");
            p.push(NAME);
            p.push(CONFIG_NAME);

            let mut f = File::create(p)?;
            let toml = toml::ser::to_string_pretty(&self)?;
            f.write_all(toml.as_bytes())?;

            Ok(())
        } else {
            bail!("No home directory.")
        }
    }

    pub fn load<P: AsRef<Path>>() -> Result<Self> {
        if let Some(mut p) = dirs::home_dir() {
            p.push(".config");
            p.push(NAME);
            p.push(CONFIG_NAME);

            let mut f = File::open(p)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(toml::from_str(s.as_str())?)
        } else {
            bail!("No home directory.")
        }
    }
}
