use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CppupConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_type: Option<String>,
    pub build_system: String,
    pub cpp_standard: String,
    pub package_manager: String,
    pub test_framework: String,
    pub license: String,
    pub author: Option<String>,
    pub quality_tools: Vec<String>,
    pub ci: String,
    pub docker: bool,
    pub ide: Vec<String>,
    pub modules: bool,
    pub git: bool,
}

impl Default for CppupConfig {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            project_type: Some("executable".to_string()),
            build_system: "cmake".to_string(),
            cpp_standard: "17".to_string(),
            package_manager: "none".to_string(),
            test_framework: "none".to_string(),
            license: "MIT".to_string(),
            author: None,
            quality_tools: Vec::new(),
            ci: "none".to_string(),
            docker: false,
            ide: Vec::new(),
            modules: false,
            git: true,
        }
    }
}

impl CppupConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: CppupConfig =
            serde_json::from_str(&content).with_context(|| "Failed to parse config file")?;

        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize config")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;

        Ok(())
    }

    pub fn get_default_config_path() -> Result<std::path::PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        let cppup_dir = config_dir.join("cppup");
        if !cppup_dir.exists() {
            fs::create_dir_all(&cppup_dir).with_context(|| "Failed to create config directory")?;
        }

        Ok(cppup_dir.join("config.json"))
    }
}
