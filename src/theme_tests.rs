#[cfg(test)]
mod tests {
    use crate::theme::{ThemeConfig, ThemeMode};
    use std::str::FromStr;

    #[test]
    fn test_default_theme_is_system() {
        let theme = ThemeConfig::default();
        assert_eq!(theme.mode, ThemeMode::System);
        assert!(theme.accent_color.is_none());
    }

    #[test]
    fn test_theme_mode_display() {
        assert_eq!(ThemeMode::Light.to_string(), "light");
        assert_eq!(ThemeMode::Dark.to_string(), "dark");
        assert_eq!(ThemeMode::System.to_string(), "system");
    }

    #[test]
    fn test_theme_mode_from_str_valid() {
        assert_eq!(ThemeMode::from_str("light").unwrap(), ThemeMode::Light);
        assert_eq!(ThemeMode::from_str("DARK").unwrap(), ThemeMode::Dark);
        assert_eq!(ThemeMode::from_str("System").unwrap(), ThemeMode::System);
    }

    #[test]
    fn test_theme_mode_from_str_invalid() {
        assert!(ThemeMode::from_str("rainbow").is_err());
        assert!(ThemeMode::from_str("").is_err());
    }

    #[test]
    fn test_is_dark() {
        let dark = ThemeConfig::new(ThemeMode::Dark);
        assert!(dark.is_dark());

        let light = ThemeConfig::new(ThemeMode::Light);
        assert!(!light.is_dark());

        let system = ThemeConfig::new(ThemeMode::System);
        assert!(!system.is_dark());
    }

    #[test]
    fn test_with_accent_color() {
        let theme = ThemeConfig::new(ThemeMode::Dark).with_accent("#ff6600");
        assert_eq!(theme.accent_color, Some("#ff6600".to_string()));
    }

    #[test]
    fn test_theme_serialization() {
        let theme = ThemeConfig::new(ThemeMode::Light).with_accent("#0099ff");
        let json = serde_json::to_string(&theme).unwrap();
        let restored: ThemeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.mode, ThemeMode::Light);
        assert_eq!(restored.accent_color, Some("#0099ff".to_string()));
    }
}
