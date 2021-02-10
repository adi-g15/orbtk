mod mouse_button;

pub use self::mouse_button::*;

use crate::utils::Point;

/// Represents a mouse event with position, mouse button and pressed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MouseEvent {
    position: Point,
    button: MouseButton,
    pressed: bool,
}

impl MouseEvent {
    /// Creates a new mouse event from position, button and pressed.
    pub fn new(position: impl Into<Point>, button: MouseButton, pressed: bool) -> Self {
        MouseEvent {
            position: position.into(),
            button,
            pressed,
        }
    }

    /// Returns the position of the mouse event.
    pub fn position(&self) -> Point {
        self.position
    }

    /// Returns the mouse button of the event.
    pub fn button(&self) -> MouseButton {
        self.button
    }

    /// Returns the value that indicates the mouse button is pressed or released.
    pub fn pressed(&self) -> bool {
        self.pressed
    }
}
