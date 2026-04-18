#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Moved { x: i32, y: i32 },
    Resized { width: u32, height: u32 },
    Maximized,
    Restored,
    Focused,
    Unfocused,
    CloseRequested,
}

pub trait WindowEventHandler {
    fn on_event(&mut self, event: &WindowEvent);
}

pub struct WindowEventDispatcher {
    events: Vec<WindowEvent>,
}

impl WindowEventDispatcher {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: WindowEvent) {
        self.events.push(event);
    }

    pub fn dispatch<H: WindowEventHandler>(&mut self, handler: &mut H) {
        for event in self.events.drain(..) {
            handler.on_event(&event);
        }
    }

    pub fn pending_count(&self) -> usize {
        self.events.len()
    }
}

impl Default for WindowEventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
