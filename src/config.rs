use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdcConfig {
    pub default_context: Option<String>,
    #[serde(default)]
    pub commands: HashMap<String, String>,
}

impl IdcConfig {
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: IdcConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
