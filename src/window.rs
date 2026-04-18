use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowState {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
    pub always_on_top: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 1024,
            height: 768,
            maximized: false,
            always_on_top: false,
        }
    }
}

impl WindowState {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            maximized: false,
            always_on_top: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.width >= 400 && self.height >= 300
    }

    pub fn with_always_on_top(mut self, value: bool) -> Self {
        self.always_on_top = value;
        self
    }

    pub fn with_maximized(mut self, value: bool) -> Self {
        self.maximized = value;
        self
    }
}
