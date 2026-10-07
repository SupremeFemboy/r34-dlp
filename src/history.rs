use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct History {
    pub downloaded_ids: HashSet<u64>,
    pub file_hashes: HashSet<String>,
}

pub fn load_history(dir: &Path) -> History {
    let history_path = dir.join(".downloaded.toml");
    if let Ok(content) = fs::read_to_string(history_path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        History::default()
    }
}

pub fn save_history(dir: &Path, history: &History) -> Result<(), Box<dyn std::error::Error>> {
    let history_path = dir.join(".downloaded.toml");
    let content = toml::to_string_pretty(history)?;
    fs::write(history_path, content)?;
    Ok(())
}
