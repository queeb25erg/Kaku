#[cfg(test)]
mod tests {
    use crate::hotkey::{Hotkey, Modifier};
    use crate::hotkey_manager::HotkeyManager;

    #[test]
    fn test_hotkey_display() {
        let hk = Hotkey::new(vec![Modifier::Ctrl, Modifier::Shift], "Space");
        assert_eq!(hk.to_string(), "Ctrl+Shift+Space");
    }

    #[test]
    fn test_hotkey_display_no_modifiers() {
        let hk = Hotkey::new(vec![], "F1");
        assert_eq!(hk.to_string(), "F1");
    }

    #[test]
    fn test_hotkey_parse_valid() {
        let hk = Hotkey::parse("Ctrl+Alt+T").unwrap();
        assert_eq!(hk.key, "T");
        assert_eq!(hk.modifiers, vec![Modifier::Ctrl, Modifier::Alt]);
    }

    #[test]
    fn test_hotkey_parse_single_key() {
        let hk = Hotkey::parse("Escape").unwrap();
        assert_eq!(hk.key, "Escape");
        assert!(hk.modifiers.is_empty());
    }

    #[test]
    fn test_hotkey_parse_unknown_modifier_skipped() {
        // Unknown modifiers like "Super" are silently skipped during parse.
        let hk = Hotkey::parse("Ctrl+Super+K").unwrap();
        assert_eq!(hk.modifiers, vec![Modifier::Ctrl]);
        assert_eq!(hk.key, "K");
    }

    #[test]
    fn test_default_toggle_hotkey() {
        // Upstream default is Ctrl+Shift+Space. I'd personally prefer Ctrl+Space
        // but keeping this in sync with tw93/Kaku for easier rebasing.
        let hk = Hotkey::default_toggle();
        assert_eq!(hk.to_string(), "Ctrl+Shift+Space");
    }

    #[test]
    fn test_manager_default_has_toggle() {
        let mgr = HotkeyManager::new();
        let hk = mgr.get("toggle_window").unwrap();
        assert_eq!(hk.to_string(), "Ctrl+Shift+Space");
    }

    #[test]
    fn test_manager_register_and_get() {
        let mut mgr = HotkeyManager::new();
        mgr.register("open_settings", Hotkey::new(vec![Modifier::Ctrl], "Comma"));
        assert!(mgr.get("open_settings").is_some());
    }

    #[test]
    fn test_manager_update_from_str() {
        let mut mgr = HotkeyManager::new();
        let ok = mgr.update_from_str("toggle_window", "Meta+K");
        assert!(ok);
        assert_eq!(mgr.get("toggle_window").unwrap().key, "K");
    }

    #[test]
    fn test_manager_update_from_str_invalid_key_returns_false() {
        // Updating with an empty string should fail gracefully.
        let mut mgr = HotkeyManager::new();
        let ok = mgr.update_from_str("toggle_window", "");
        assert!(!ok);
    }

    #[test]
    fn test_manager_update_from_str_unknown_action_returns_false() {
        // Trying to update a key that was never registered should also return false.
        let mut mgr = HotkeyManager::new();
        let ok = mgr.update_from_str("nonexistent_action", "Ctrl+X");
        assert!(!ok);
    }

    #[test]
    fn test_manager_remove() {
        let mut mgr = HotkeyManager::new();
        let removed = mgr.remove("toggle_window");
        assert!(removed.is_some());
        assert!(mgr.get("toggle_window").is_none());
    }
}
