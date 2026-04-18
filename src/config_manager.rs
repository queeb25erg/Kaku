use crate::config::AppConfig;
use anyhow::Result;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config = AppConfig::load()?;
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
        })
    }

    pub fn get(&self) -> AppConfig {
        self.config.read().unwrap().clone()
    }

    pub fn update<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.write().unwrap();
        updater(&mut config);
        config.save()
    }

    pub fn set_theme(&self, theme: &str) -> Result<()> {
        self.update(|c| c.theme = theme.to_string())
    }

    pub fn set_font_size(&self, size: u32) -> Result<()> {
        // clamp font size to a sane range so I don't accidentally set it to 0 or 999
        let size = size.clamp(8, 72);
        self.update(|c| c.font_size = size)
    }

    pub fn set_auto_save(&self, enabled: bool) -> Result<()> {
        self.update(|c| c.auto_save = enabled)
    }

    pub fn set_language(&self, lang: &str) -> Result<()> {
        self.update(|c| c.language = lang.to_string())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ConfigManager")
    }
}
