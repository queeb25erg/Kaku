#[cfg(test)]
mod tests {
    use crate::config::AppConfig;
    use crate::config_manager::ConfigManager;

    #[test]
    fn test_default_config_values() {
        let config = AppConfig::default();
        assert_eq!(config.theme, "light");
        assert_eq!(config.font_size, 14);
        assert!(config.auto_save);
        assert_eq!(config.auto_save_interval, 30);
        // I prefer a slightly larger default window size
        assert_eq!(config.window_width, 1200);
        assert_eq!(config.window_height, 800);
        assert_eq!(config.language, "en");
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.theme, parsed.theme);
        assert_eq!(config.font_size, parsed.font_size);
        assert_eq!(config.language, parsed.language);
        // also verify auto_save round-trips correctly
        assert_eq!(config.auto_save, parsed.auto_save);
    }

    #[test]
    fn test_config_manager_update() {
        let manager = ConfigManager::new().unwrap();
        let original_theme = manager.get().theme.clone();

        manager.update(|c| c.theme = "dark".to_string()).unwrap();
        assert_eq!(manager.get().theme, "dark");

        // Restore
        manager.update(|c| c.theme = original_theme.clone()).unwrap();
        assert_eq!(manager.get().theme, original_theme);
    }

    #[test]
    fn test_set_font_size() {
        let manager = ConfigManager::new().unwrap();
        let original = manager.get().font_size;

        manager.set_font_size(18).unwrap();
        assert_eq!(manager.get().font_size, 18);

        manager.set_font_size(original).unwrap();
    }

    #[test]
    fn test_set_language() {
        let manager = ConfigManager::new().unwrap();
        manager.set_language("ja").unwrap();
        assert_eq!(manager.get().language, "ja");
        manager.set_language("en").unwrap();
    }
}
