use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub user_id: u64,
    pub global_negative_tags: Vec<String>,
    pub sort_negative_tags: Option<bool>,
    pub parallel_downloads: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        let mut tags = vec!["3d".to_string(), "low_res".to_string()];
        tags.sort();
        Self {
            api_key: "YOUR_API_HERE".to_string(),
            user_id: 1234567,
            global_negative_tags: tags,
            sort_negative_tags: Some(true),
            parallel_downloads: Some(4),
        }
    }
}

pub fn default_config_path() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return PathBuf::from(xdg).join("r34-dlp").join("config.toml");
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".config").join("r34-dlp").join("config.toml");
    }
    PathBuf::from(".config").join("r34-dlp").join("config.toml")
}

pub fn load_config(config_path: Option<PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path.unwrap_or_else(default_config_path);

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let default_config = Config::default();
        let toml_str = toml::to_string_pretty(&default_config)?;
        fs::write(&path, toml_str)?;
        println!("Created default config at: {}", path.display());
        println!("Please fill it and restart the app.");
        std::process::exit(0);
    }

    let config_str = fs::read_to_string(&path)?;
    let mut config: Config = toml::from_str(&config_str)?;

    if config.sort_negative_tags.unwrap_or(true) {
        config.global_negative_tags.sort();
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_tags_sorted() {
        let cfg = Config::default();
        let mut sorted = cfg.global_negative_tags.clone();
        sorted.sort();
        assert_eq!(cfg.global_negative_tags, sorted);
    }

    #[test]
    fn test_default_config_path_structure() {
        let path = default_config_path();
        assert!(path.ends_with("r34-dlp/config.toml") || path.ends_with("config.toml"));
    }
}
