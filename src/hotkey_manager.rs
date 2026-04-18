use crate::hotkey::Hotkey;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HotkeyManager {
    bindings: HashMap<String, Hotkey>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut manager = Self {
            bindings: HashMap::new(),
        };
        manager.register("toggle_window", Hotkey::default_toggle());
        manager
    }
}

impl HotkeyManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, action: impl Into<String>, hotkey: Hotkey) {
        self.bindings.insert(action.into(), hotkey);
    }

    pub fn get(&self, action: &str) -> Option<&Hotkey> {
        self.bindings.get(action)
    }

    pub fn remove(&mut self, action: &str) -> Option<Hotkey> {
        self.bindings.remove(action)
    }

    pub fn all(&self) -> &HashMap<String, Hotkey> {
        &self.bindings
    }

    pub fn update_from_str(&mut self, action: &str, hotkey_str: &str) -> bool {
        if let Some(hotkey) = Hotkey::parse(hotkey_str) {
            self.register(action, hotkey);
            true
        } else {
            false
        }
    }
}
