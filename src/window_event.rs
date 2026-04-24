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

/// Collects and dispatches window events to a handler.
/// Events are processed in FIFO order when dispatch() is called.
///
/// Note: pre-allocating with capacity 8 since most frames won't accumulate
/// more than a handful of events, avoids early reallocations.
pub struct WindowEventDispatcher {
    events: Vec<WindowEvent>,
}

impl WindowEventDispatcher {
    pub fn new() -> Self {
        Self { events: Vec::with_capacity(8) }
    }

    pub fn push(&mut self, event: WindowEvent) {
        self.events.push(event);
    }

    pub fn dispatch<H: WindowEventHandler>(&mut self, handler: &mut H) {
        for event in self.events.drain(..) {
            handler.on_event(&event);
        }
    }

    /// Returns true if there are no pending events.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
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
