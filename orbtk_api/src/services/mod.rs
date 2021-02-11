//! This module contains global services.

mod application;
mod clipboard;
mod font_manager;
mod keyboard;
mod localization_manager;
mod settings;
mod theme_manager;

pub use self::application::*;
pub use self::clipboard::*;
pub use self::font_manager::*;
pub use self::keyboard::*;
pub use self::localization_manager::*;
pub use self::settings::*;
pub use self::theme_manager::*;
