#[cfg(test)]
mod tests {
    use crate::window::WindowState;
    use crate::window_manager::WindowManager;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_default_window_state() {
        let state = WindowState::default();
        assert_eq!(state.width, 1024);
        assert_eq!(state.height, 768);
        assert!(!state.maximized);
        assert!(!state.always_on_top);
    }

    #[test]
    fn test_window_state_validity() {
        let valid = WindowState::new(0, 0, 800, 600);
        assert!(valid.is_valid());

        let invalid = WindowState::new(0, 0, 100, 100);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_window_state_builders() {
        let state = WindowState::default()
            .with_always_on_top(true)
            .with_maximized(true);
        assert!(state.always_on_top);
        assert!(state.maximized);
    }

    #[test]
    fn test_window_manager_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("window_state.json");
        let mut manager = WindowManager::new(path.clone());
        manager.state = WindowState::new(50, 60, 1280, 720);
        manager.save_state().unwrap();

        let loaded = WindowManager::new(path);
        assert_eq!(loaded.state.width, 1280);
        assert_eq!(loaded.state.height, 720);
        assert_eq!(loaded.state.x, 50);
    }

    #[test]
    fn test_window_manager_reset() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("window_state.json");
        let mut manager = WindowManager::new(path);
        manager.state = WindowState::new(999, 999, 1920, 1080);
        manager.reset();
        assert_eq!(manager.state, WindowState::default());
    }

    #[test]
    fn test_update_ignores_invalid_state() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("window_state.json");
        let mut manager = WindowManager::new(path);
        let original = manager.state.clone();
        manager.update_state(WindowState::new(0, 0, 10, 10));
        assert_eq!(manager.state, original);
    }
}
