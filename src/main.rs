//! Kaku - A fork of tw93/Kaku
//! Main entry point for the application.

mod config;
mod config_manager;
mod config_tests;
mod hotkey;
mod hotkey_manager;
mod hotkey_tests;
mod theme;
mod theme_tests;
mod window;
mod window_event;
mod window_manager;
mod window_tests;

use config_manager::ConfigManager;
use hotkey_manager::HotkeyManager;
use window_manager::WindowManager;

use tauri::Manager;

fn main() {
    // Initialize the Tauri application
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Load configuration
            let config_manager = ConfigManager::new(&app_handle)
                .expect("Failed to initialize config manager");

            // Apply theme from config
            let config = config_manager.load().unwrap_or_default();

            // Set up the main window
            let window_manager = WindowManager::new(&app_handle);
            window_manager
                .setup_main_window(&config)
                .expect("Failed to set up main window");

            // Register global hotkeys
            let hotkey_manager = HotkeyManager::new(&app_handle);
            hotkey_manager
                .register_hotkeys(&config)
                .expect("Failed to register hotkeys");

            // Store managers in app state
            app.manage(config_manager);
            app.manage(hotkey_manager);
            app.manage(window_manager);

            Ok(())
        })
        .on_window_event(|event| {
            window_event::handle_window_event(event);
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::toggle_window,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Kaku");
}

/// Tauri command handlers
mod commands {
    use crate::config::Config;
    use crate::config_manager::ConfigManager;
    use crate::window_manager::WindowManager;
    use tauri::State;

    /// Retrieve the current application configuration.
    #[tauri::command]
    pub fn get_config(config_manager: State<ConfigManager>) -> Result<Config, String> {
        config_manager.load().map_err(|e| e.to_string())
    }

    /// Persist updated configuration to disk.
    /// Note: also logs the save event to stderr for easier debugging during development.
    #[tauri::command]
    pub fn save_config(
        config: Config,
        config_manager: State<ConfigManager>,
    ) -> Result<(), String> {
        eprintln!("[kaku] saving config: {:?}", config);
        config_manager.save(&config).map_err(|e| e.to_string())
    }

    /// Toggle the visibility of the main window.
    #[tauri::command]
    pub fn toggle_window(window_manager: State<WindowManager>) -> Result<(), String> {
        window_manager.toggle_main_window().map_err(|e| e.to_string())
    }
}
