use std::{fs, path::Path};
use serde::{Serialize, Deserialize};
use crate::state::Preset;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub default_preset: Option<String>,
}

impl Config {
    pub fn load(path: &Path) -> Self {
        let data = fs::read_to_string(path).ok();
        if let Some(data) = data {
            toml::from_str(&data).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn save(&self, path: &Path) {
        if let Ok(data) = toml::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }

    pub fn preset(&self) -> Option<Preset> {
        self.default_preset.as_deref().and_then(Preset::from_key)
    }

    pub fn set_preset(&mut self, p: Preset) {
        self.default_preset = Some(p.key().to_string());
    }
}
