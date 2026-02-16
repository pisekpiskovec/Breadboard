use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub memory_bytes_per_row: usize,
    pub memory_bytes_per_column: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                memory_bytes_per_row: 8,
                memory_bytes_per_column: 128,
            },
            theme: ThemeConfig {
                mode: "Dark".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let path = Self::get_config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;

        toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::get_config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let toml_string = toml::to_string_pretty(self)
                .map_err(|e| format!("Failed to seialize config: {}", e))?;

        fs::write(&path, toml_string)
                .map_err(|e| format!("Failed to write config: {}", e))
    }

    fn get_config_path() -> Result<PathBuf, String> {
        let mut path =
            dirs::config_dir().unwrap_or_else(|| PathBuf::from(std::env::home_dir().unwrap()));
        path.push("Breadboard");
        fs::create_dir_all(&path).ok();
        path.push("config.toml");
        Ok(path)
    }
}
