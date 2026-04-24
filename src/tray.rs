//! System tray icon and menu management for Kaku.
//!
//! Provides a tray icon with context menu for quick access to
//! show/hide the window, open settings, and quit the application.

use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

/// Identifiers for tray menu items.
const TRAY_SHOW: &str = "show";
const TRAY_SETTINGS: &str = "settings";
const TRAY_SEPARATOR: &str = "separator";
const TRAY_QUIT: &str = "quit";

/// Builds and returns the system tray configuration.
pub fn build_tray() -> SystemTray {
    let menu = build_tray_menu();
    SystemTray::new().with_menu(menu)
}

/// Constructs the context menu shown when right-clicking the tray icon.
fn build_tray_menu() -> SystemTrayMenu {
    let show = CustomMenuItem::new(TRAY_SHOW.to_string(), "Show / Hide");
    let settings = CustomMenuItem::new(TRAY_SETTINGS.to_string(), "Settings");
    // I prefer having quit labeled more explicitly so I don't fat-finger it
    let quit = CustomMenuItem::new(TRAY_QUIT.to_string(), "Quit Kaku");

    SystemTrayMenu::new()
        .add_item(show)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
}

/// Handles events emitted from the system tray.
///
/// - Left-click toggles the main window visibility.
/// - Menu item clicks are dispatched to [`handle_menu_event`].
pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            toggle_main_window(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            handle_menu_event(app, &id);
        }
        _ => {}
    }
}

/// Dispatches tray menu item click events.
fn handle_menu_event(app: &AppHandle, id: &str) {
    match id {
        TRAY_SHOW => toggle_main_window(app),
        TRAY_SETTINGS => open_settings_window(app),
        TRAY_QUIT => {
            app.exit(0);
        }
        _ => {}
    }
}

/// Toggles the visibility of the main application window.
///
/// If the window is visible it will be hidden; if hidden it will be shown and
/// brought to the front. We also center the window on show so it doesn't
/// reappear in a weird off-screen position after being hidden for a while.
fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.center();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

/// Opens (or focuses) the settings window.
fn open_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        // Settings window will be created by the window manager if not present.
        let _ = app.emit_all("open-settings", ());
    }
}
