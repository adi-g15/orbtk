use super::scan_codes::*;

/// Used to identify a keyboard key.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Tick,
    Minus,
    Equals,
    Backslash,
    BraceOpen,
    BraceClose,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Backspace,
    Space,
    Tab,
    Caps,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    Enter,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Home,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    PageUp,
    PageDown,
    End,
    Delete,
    Unknown,
}

// converts scan code into key enum
impl From<u8> for Key {
    fn from(c: u8) -> Self {
        let key = match c {
            KEY_A => Key::A,
            KEY_B => Key::B,
            KEY_C => Key::C,
            KEY_D => Key::D,
            KEY_E => Key::E,
            KEY_F => Key::F,
            KEY_G => Key::G,
            KEY_H => Key::H,
            KEY_I => Key::I,
            KEY_J => Key::J,
            KEY_K => Key::K,
            KEY_L => Key::L,
            KEY_M => Key::M,
            KEY_N => Key::N,
            KEY_O => Key::O,
            KEY_P => Key::P,
            KEY_Q => Key::Q,
            KEY_R => Key::R,
            KEY_S => Key::S,
            KEY_T => Key::T,
            KEY_U => Key::U,
            KEY_V => Key::V,
            KEY_W => Key::W,
            KEY_X => Key::X,
            KEY_Y => Key::Y,
            KEY_Z => Key::Z,
            KEY_0 => Key::Zero,
            KEY_1 => Key::One,
            KEY_2 => Key::Two,
            KEY_3 => Key::Three,
            KEY_4 => Key::Four,
            KEY_5 => Key::Five,
            KEY_6 => Key::Six,
            KEY_7 => Key::Seven,
            KEY_8 => Key::Eight,
            KEY_9 => Key::Nine,
            KEY_NUM_0 => Key::Num0,
            KEY_NUM_1 => Key::Num1,
            KEY_NUM_2 => Key::Num2,
            KEY_NUM_3 => Key::Num3,
            KEY_NUM_4 => Key::Num4,
            KEY_NUM_5 => Key::Num5,
            KEY_NUM_6 => Key::Num6,
            KEY_NUM_7 => Key::Num7,
            KEY_NUM_8 => Key::Num8,
            KEY_NUM_9 => Key::Num9,
            KEY_TICK => Key::Tick,
            KEY_MINUS => Key::Minus,
            KEY_EQUALS => Key::Equals,
            KEY_BACKSLASH => Key::Backslash,
            KEY_BRACE_OPEN => Key::BraceOpen,
            KEY_BRACE_CLOSE => Key::BraceClose,
            KEY_SEMICOLON => Key::Semicolon,
            KEY_QUOTE => Key::Quote,
            KEY_COMMA => Key::Comma,
            KEY_PERIOD => Key::Period,
            KEY_SLASH => Key::Slash,
            KEY_BACKSPACE => Key::Backspace,
            KEY_SPACE => Key::Space,
            KEY_TAB => Key::Tab,
            KEY_CAPS => Key::Caps,
            KEY_SHIFT_LEFT => Key::ShiftLeft,
            KEY_SHIFT_RIGHT => Key::ShiftRight,
            KEY_CONTROL_LEFT => Key::ControlLeft,
            KEY_ALT_LEFT => Key::AltLeft,
            KEY_ALT_RIGHT => Key::AltRight,
            KEY_ENTER => Key::Enter,
            KEY_ESCAPE => Key::Escape,
            KEY_F1 => Key::F1,
            KEY_F2 => Key::F2,
            KEY_F3 => Key::F3,
            KEY_F4 => Key::F4,
            KEY_F5 => Key::F5,
            KEY_F6 => Key::F6,
            KEY_F7 => Key::F7,
            KEY_F8 => Key::F8,
            KEY_F9 => Key::F9,
            KEY_F10 => Key::F10,
            KEY_F11 => Key::F11,
            KEY_F12 => Key::F12,
            KEY_HOME => Key::Home,
            KEY_ARROW_UP => Key::ArrowUp,
            KEY_ARROW_LEFT => Key::ArrowLeft,
            KEY_ARROW_RIGHT => Key::ArrowRight,
            KEY_ARROW_DOWN => Key::ArrowDown,
            KEY_PAGE_UP => Key::PageUp,
            KEY_PAGE_DOWN => Key::PageDown,
            KEY_END => Key::End,
            KEY_DELETE => Key::Delete,
            _ => Key::Unknown,
        };

        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_scan_code() {
        assert_eq!(Key::from(0x1E), Key::A);
        assert_eq!(Key::from(0x30), Key::B);
        assert_eq!(Key::from(0x2E), Key::C);
        assert_eq!(Key::from(0x20), Key::D);
        assert_eq!(Key::from(0x12), Key::E);
        assert_eq!(Key::from(0x21), Key::F);
        assert_eq!(Key::from(0x22), Key::G);
        assert_eq!(Key::from(0x23), Key::H);
        assert_eq!(Key::from(0x17), Key::I);
        assert_eq!(Key::from(0x24), Key::J);
        assert_eq!(Key::from(0x25), Key::K);
        assert_eq!(Key::from(0x26), Key::L);
        assert_eq!(Key::from(0x32), Key::M);
        assert_eq!(Key::from(0x31), Key::N);
        assert_eq!(Key::from(0x18), Key::O);
        assert_eq!(Key::from(0x19), Key::P);
        assert_eq!(Key::from(0x10), Key::Q);
        assert_eq!(Key::from(0x13), Key::R);
        assert_eq!(Key::from(0x1F), Key::S);
        assert_eq!(Key::from(0x14), Key::T);
        assert_eq!(Key::from(0x16), Key::U);
        assert_eq!(Key::from(0x2F), Key::V);
        assert_eq!(Key::from(0x11), Key::W);
        assert_eq!(Key::from(0x2D), Key::X);
        assert_eq!(Key::from(0x15), Key::Y);
        assert_eq!(Key::from(0x2C), Key::Z);
        assert_eq!(Key::from(0x0B), Key::Zero);
        assert_eq!(Key::from(0x02), Key::One);
        assert_eq!(Key::from(0x03), Key::Two);
        assert_eq!(Key::from(0x04), Key::Three);
        assert_eq!(Key::from(0x05), Key::Four);
        assert_eq!(Key::from(0x06), Key::Five);
        assert_eq!(Key::from(0x07), Key::Six);
        assert_eq!(Key::from(0x08), Key::Seven);
        assert_eq!(Key::from(0x09), Key::Eight);
        assert_eq!(Key::from(0x0A), Key::Nine);
        assert_eq!(Key::from(0x70), Key::Num0);
        assert_eq!(Key::from(0x71), Key::Num1);
        assert_eq!(Key::from(0x72), Key::Num2);
        assert_eq!(Key::from(0x73), Key::Num3);
        assert_eq!(Key::from(0x74), Key::Num4);
        assert_eq!(Key::from(0x75), Key::Num5);
        assert_eq!(Key::from(0x76), Key::Num6);
        assert_eq!(Key::from(0x77), Key::Num7);
        assert_eq!(Key::from(0x78), Key::Num8);
        assert_eq!(Key::from(0x79), Key::Num9);
        assert_eq!(Key::from(0x29), Key::Tick);
        assert_eq!(Key::from(0x0C), Key::Minus);
        assert_eq!(Key::from(0x0D), Key::Equals);
        assert_eq!(Key::from(0x2B), Key::Backslash);
        assert_eq!(Key::from(0x1A), Key::BraceOpen);
        assert_eq!(Key::from(0x1B), Key::BraceClose);
        assert_eq!(Key::from(0x27), Key::Semicolon);
        assert_eq!(Key::from(0x28), Key::Quote);
        assert_eq!(Key::from(0x33), Key::Comma);
        assert_eq!(Key::from(0x34), Key::Period);
        assert_eq!(Key::from(0x35), Key::Slash);
        assert_eq!(Key::from(0x0E), Key::Backspace);
        assert_eq!(Key::from(0x39), Key::Space);
        assert_eq!(Key::from(0x0F), Key::Tab);
        assert_eq!(Key::from(0x3A), Key::Caps);
        assert_eq!(Key::from(0x2A), Key::ShiftLeft);
        assert_eq!(Key::from(0x36), Key::ShiftRight);
        assert_eq!(Key::from(0x1D), Key::ControlLeft);
        assert_eq!(Key::from(0x64), Key::AltLeft);
        assert_eq!(Key::from(0x38), Key::AltRight);
        assert_eq!(Key::from(0x1C), Key::Enter);
        assert_eq!(Key::from(0x01), Key::Escape);
        assert_eq!(Key::from(0x3B), Key::F1);
        assert_eq!(Key::from(0x3C), Key::F2);
        assert_eq!(Key::from(0x3D), Key::F3);
        assert_eq!(Key::from(0x3E), Key::F4);
        assert_eq!(Key::from(0x3F), Key::F5);
        assert_eq!(Key::from(0x40), Key::F6);
        assert_eq!(Key::from(0x41), Key::F7);
        assert_eq!(Key::from(0x42), Key::F8);
        assert_eq!(Key::from(0x43), Key::F9);
        assert_eq!(Key::from(0x44), Key::F10);
        assert_eq!(Key::from(0x57), Key::F11);
        assert_eq!(Key::from(0x58), Key::F12);
        assert_eq!(Key::from(0x47), Key::Home);
        assert_eq!(Key::from(0x48), Key::ArrowUp);
        assert_eq!(Key::from(0x4B), Key::ArrowLeft);
        assert_eq!(Key::from(0x4D), Key::ArrowRight);
        assert_eq!(Key::from(0x50), Key::ArrowDown);
        assert_eq!(Key::from(0x49), Key::PageUp);
        assert_eq!(Key::from(0x51), Key::PageDown);
        assert_eq!(Key::from(0x4F), Key::End);
        assert_eq!(Key::from(0x53), Key::Delete);
        assert_eq!(Key::from(0), Key::Unknown);
    }
}
