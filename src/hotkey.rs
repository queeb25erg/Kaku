use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hotkey {
    pub modifiers: Vec<Modifier>,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Meta,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modifier::Ctrl => write!(f, "Ctrl"),
            Modifier::Alt => write!(f, "Alt"),
            Modifier::Shift => write!(f, "Shift"),
            Modifier::Meta => write!(f, "Meta"),
        }
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mods: Vec<String> = self.modifiers.iter().map(|m| m.to_string()).collect();
        if mods.is_empty() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{}+{}", mods.join("+"), self.key)
        }
    }
}

impl Hotkey {
    pub fn new(modifiers: Vec<Modifier>, key: impl Into<String>) -> Self {
        Self {
            modifiers,
            key: key.into(),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('+').collect();
        if parts.is_empty() {
            return None;
        }
        let key = parts.last()?.to_string();
        let modifiers = parts[..parts.len() - 1]
            .iter()
            .filter_map(|p| match *p {
                "Ctrl" => Some(Modifier::Ctrl),
                "Alt" => Some(Modifier::Alt),
                "Shift" => Some(Modifier::Shift),
                "Meta" => Some(Modifier::Meta),
                _ => None,
            })
            .collect();
        Some(Self { modifiers, key })
    }

    /// I prefer Meta+Space as the toggle shortcut since Ctrl+Shift+Space
    /// conflicts with my terminal's autocomplete binding.
    pub fn default_toggle() -> Self {
        Self::new(vec![Modifier::Meta], "Space")
    }
}
