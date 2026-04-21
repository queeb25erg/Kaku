use std::fs;
use std::path::PathBuf;
use crate::window::WindowState;

pub struct WindowManager {
    state_path: PathBuf,
    pub state: WindowState,
}

impl WindowManager {
    pub fn new(state_path: PathBuf) -> Self {
        let state = Self::load_state(&state_path).unwrap_or_default();
        Self { state_path, state }
    }

    fn load_state(path: &PathBuf) -> Option<WindowState> {
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save_state(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.state)
            .map_err(|e| e.to_string())?;
        if let Some(parent) = self.state_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.state_path, content).map_err(|e| e.to_string())
    }

    pub fn update_state(&mut self, state: WindowState) {
        if state.is_valid() {
            self.state = state;
            // Auto-save whenever state is updated so we don't lose position on crash
            if let Err(e) = self.save_state() {
                eprintln!("[WindowManager] Failed to auto-save state: {}", e);
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = WindowState::default();
    }
}
