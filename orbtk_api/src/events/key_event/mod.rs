mod key;
mod key_handlers;
mod scan_codes;

// pub use self::key::*;
pub use self::key_handlers::*;
pub use self::scan_codes::*;

// /// Represents a key event with scan code, pressed and key.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct KeyEvent {
//     scan_code: u8,
//     character: char,
//     pressed: bool,
//     key: Key,
// }

// impl KeyEvent {
//     /// Creates a new key event from scan_code and pressed.
//     pub fn new(scan_code: u8, character: char, pressed: bool) -> Self {
//         KeyEvent {
//             scan_code,
//             character,
//             pressed,
//             key: Key::from(scan_code),
//         }
//     }

//     /// Returns the scan code of the key event.
//     pub fn scan_code(&self) -> u8 {
//         self.scan_code
//     }

//     /// Returns the character of the key event.
//     pub fn character(&self) -> char {
//         self.character
//     }

//     /// Returns the value that indicates the the given key is
//     /// pressed or released.
//     pub fn pressed(&self) -> bool {
//         self.pressed
//     }

//     /// Returns the key of the event.
//     pub fn key(&self) -> Key {
//         self.key
//     }
// }
