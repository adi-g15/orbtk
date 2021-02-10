//! This module contains global services.

mod application;
mod clipboard;
mod keyboard;
mod settings;

pub use self::application::*;
pub use self::clipboard::*;
pub use self::keyboard::*;
pub use self::settings::*;
