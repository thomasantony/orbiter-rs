/// Thanks to Émile Grégoire (https://github.com/emgre/orbiter-rs/blob/master/orbiter/src/input.rs)
use std::os::raw::c_char;

/// Keyboard key.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Key {
    /// Escape key
    Escape,
    /// '1' key on main keyboard
    Main1,
    /// '2' key on main keyboard
    Main2,
    /// '3' key on main keyboard
    Main3,
    /// '4' key on main keyboard
    Main4,
    /// '5' key on main keyboard
    Main5,
    /// '6' key on main keyboard
    Main6,
    /// '7' key on main keyboard
    Main7,
    /// '8' key on main keyboard
    Main8,
    /// '9' key on main keyboard
    Main9,
    /// '0' key on main keyboard
    Main0,
    /// '-' key on main keyboard
    Minus,
    /// '=' key on main keyboard
    Equals,
    /// backspace key
    Backspace,
    /// tab key
    Tabulation,
    /// 'Q' key
    Q,
    /// 'W' key
    W,
    /// 'E' key
    E,
    /// 'R' key
    R,
    /// 'T' key
    T,
    /// 'Y' key
    Y,
    /// 'U' key
    U,
    /// 'I' key
    I,
    /// 'O' key
    O,
    /// 'P' key
    P,
    /// '[' (left bracket) key
    LeftBracket,
    /// ']' (right bracket) key
    RightBracket,
    /// 'Enter' key on main keyboard
    Enter,
    /// Left 'Ctrl' key
    LeftControl,
    /// 'A' key
    A,
    /// 'S' key
    S,
    /// 'D' key
    D,
    /// 'F' key
    F,
    /// 'G' key
    G,
    /// 'H' key
    H,
    /// 'J' key
    J,
    /// 'K' key
    K,
    /// 'L' key
    L,
    /// ';' (semicolon) key
    Semicolon,
    /// ' (apostrophe) key
    Apostrophe,
    /// accent grave
    Grave,
    /// Left 'Shift' key
    LeftShift,
    /// '\' (Backslash) key
    Backslash,
    /// 'Z' key
    Z,
    /// 'X' key
    X,
    /// 'C' key
    C,
    /// 'V' key
    V,
    /// 'B' key
    B,
    /// 'N' key
    N,
    /// 'M' key
    M,
    /// ',' (comma) key
    Comma,
    /// '.' key on main keyboard
    Period,
    /// '/' key on main keyboard
    Slash,
    /// Right 'Shift' key
    RightShift,
    /// * on numeric keypad
    Multiply,
    /// left Alt
    LeftAlt,
    /// 'Space' key
    Space,
    /// caps lock key
    CapsLock,
    /// F1 function key
    F1,
    /// F2 function key
    F2,
    /// F3 function key
    F3,
    /// F4 function key
    F4,
    /// F5 function key
    F5,
    /// F6 function key
    F6,
    /// F7 function key
    F7,
    /// F8 function key
    F8,
    /// F9 function key
    F9,
    /// F10 function key
    F10,
    /// 'Num Lock' key
    NumLock,
    /// Scroll lock
    ScrollLock,
    /// '7' key on numeric keypad
    NumPad7,
    /// '8' key on numeric keypad
    NumPad8,
    /// '9' key on numeric keypad
    NumPad9,
    /// '-' key on numeric keypad
    Subtract,
    /// '4' key on numeric keypad
    NumPad4,
    /// '5' key on numeric keypad
    NumPad5,
    /// '6' key on numeric keypad
    NumPad6,
    /// '+' key on numeric keypad
    Add,
    /// '1' key on numeric keypad
    NumPad1,
    /// '2' key on numeric keypad
    NumPad2,
    /// '3' key on numeric keypad
    NumPad3,
    /// '0' key on numeric keypad
    NumPad0,
    /// '.' key on numeric keypad
    Decimal,
    /// | \< \> on UK/German keyboards
    OEM102,
    /// F11 function key
    F11,
    /// F12 function key
    F12,
    /// Enter on numeric keypad
    NumPadEnter,
    /// right Control key
    RightControl,
    /// '/' key on numeric keypad
    Divide,
    /// SysRq/PrtScn key
    PrintScreen,
    /// right Alt
    RightAlt,
    /// Break/Pause key
    Pause,
    /// Home on cursor keypad
    Home,
    /// up-arrow on cursor keypad
    Up,
    /// PgUp on cursor keypad
    PageUp,
    /// left-arrow on cursor keypad
    Left,
    /// right-arrow on cursor keypad
    Right,
    /// End on cursor keypad
    End,
    /// down-arrow on cursor keypad
    Down,
    /// PgDn on cursor keypad
    PageDown,
    /// Insert on cursor keypad
    Insert,
    /// Delete on cursor keypad
    Delete,
    /// Unknown key
    Unknown(u8),
}

impl Key {
    pub(crate) fn as_u8(self) -> u8 {
        match self {
            Self::Escape => 0x01,
            Self::Main1 => 0x02,
            Self::Main2 => 0x03,
            Self::Main3 => 0x04,
            Self::Main4 => 0x05,
            Self::Main5 => 0x06,
            Self::Main6 => 0x07,
            Self::Main7 => 0x08,
            Self::Main8 => 0x09,
            Self::Main9 => 0x0A,
            Self::Main0 => 0x0B,
            Self::Minus => 0x0C,
            Self::Equals => 0x0D,
            Self::Backspace => 0x0E,
            Self::Tabulation => 0x0F,
            Self::Q => 0x10,
            Self::W => 0x11,
            Self::E => 0x12,
            Self::R => 0x13,
            Self::T => 0x14,
            Self::Y => 0x15,
            Self::U => 0x16,
            Self::I => 0x17,
            Self::O => 0x18,
            Self::P => 0x19,
            Self::LeftBracket => 0x1A,
            Self::RightBracket => 0x1B,
            Self::Enter => 0x1C,
            Self::LeftControl => 0x1D,
            Self::A => 0x1E,
            Self::S => 0x1F,
            Self::D => 0x20,
            Self::F => 0x21,
            Self::G => 0x22,
            Self::H => 0x23,
            Self::J => 0x24,
            Self::K => 0x25,
            Self::L => 0x26,
            Self::Semicolon => 0x27,
            Self::Apostrophe => 0x28,
            Self::Grave => 0x29,
            Self::LeftShift => 0x2A,
            Self::Backslash => 0x2B,
            Self::Z => 0x2C,
            Self::X => 0x2D,
            Self::C => 0x2E,
            Self::V => 0x2F,
            Self::B => 0x30,
            Self::N => 0x31,
            Self::M => 0x32,
            Self::Comma => 0x33,
            Self::Period => 0x34,
            Self::Slash => 0x35,
            Self::RightShift => 0x36,
            Self::Multiply => 0x37,
            Self::LeftAlt => 0x38,
            Self::Space => 0x39,
            Self::CapsLock => 0x3A,
            Self::F1 => 0x3B,
            Self::F2 => 0x3C,
            Self::F3 => 0x3D,
            Self::F4 => 0x3E,
            Self::F5 => 0x3F,
            Self::F6 => 0x40,
            Self::F7 => 0x41,
            Self::F8 => 0x42,
            Self::F9 => 0x43,
            Self::F10 => 0x44,
            Self::NumLock => 0x45,
            Self::ScrollLock => 0x46,
            Self::NumPad7 => 0x47,
            Self::NumPad8 => 0x48,
            Self::NumPad9 => 0x49,
            Self::Subtract => 0x4A,
            Self::NumPad4 => 0x4B,
            Self::NumPad5 => 0x4C,
            Self::NumPad6 => 0x4D,
            Self::Add => 0x4E,
            Self::NumPad1 => 0x4F,
            Self::NumPad2 => 0x50,
            Self::NumPad3 => 0x51,
            Self::NumPad0 => 0x52,
            Self::Decimal => 0x53,
            Self::OEM102 => 0x56,
            Self::F11 => 0x57,
            Self::F12 => 0x58,
            Self::NumPadEnter => 0x9C,
            Self::RightControl => 0x9D,
            Self::Divide => 0xB5,
            Self::PrintScreen => 0xB7,
            Self::RightAlt => 0xB8,
            Self::Pause => 0xC5,
            Self::Home => 0xC7,
            Self::Up => 0xC8,
            Self::PageUp => 0xC9,
            Self::Left => 0xCB,
            Self::Right => 0xCD,
            Self::End => 0xCF,
            Self::Down => 0xD0,
            Self::PageDown => 0xD1,
            Self::Insert => 0xD2,
            Self::Delete => 0xD3,
            Self::Unknown(value) => value,
        }
    }

    pub(crate) fn from(value: u8) -> Self {
        match value {
            0x01 => Self::Escape,
            0x02 => Self::Main1,
            0x03 => Self::Main2,
            0x04 => Self::Main3,
            0x05 => Self::Main4,
            0x06 => Self::Main5,
            0x07 => Self::Main6,
            0x08 => Self::Main7,
            0x09 => Self::Main8,
            0x0A => Self::Main9,
            0x0B => Self::Main0,
            0x0C => Self::Minus,
            0x0D => Self::Equals,
            0x0E => Self::Backspace,
            0x0F => Self::Tabulation,
            0x10 => Self::Q,
            0x11 => Self::W,
            0x12 => Self::E,
            0x13 => Self::R,
            0x14 => Self::T,
            0x15 => Self::Y,
            0x16 => Self::U,
            0x17 => Self::I,
            0x18 => Self::O,
            0x19 => Self::P,
            0x1A => Self::LeftBracket,
            0x1B => Self::RightBracket,
            0x1C => Self::Enter,
            0x1D => Self::LeftControl,
            0x1E => Self::A,
            0x1F => Self::S,
            0x20 => Self::D,
            0x21 => Self::F,
            0x22 => Self::G,
            0x23 => Self::H,
            0x24 => Self::J,
            0x25 => Self::K,
            0x26 => Self::L,
            0x27 => Self::Semicolon,
            0x28 => Self::Apostrophe,
            0x29 => Self::Grave,
            0x2A => Self::LeftShift,
            0x2B => Self::Backslash,
            0x2C => Self::Z,
            0x2D => Self::X,
            0x2E => Self::C,
            0x2F => Self::V,
            0x30 => Self::B,
            0x31 => Self::N,
            0x32 => Self::M,
            0x33 => Self::Comma,
            0x34 => Self::Period,
            0x35 => Self::Slash,
            0x36 => Self::RightShift,
            0x37 => Self::Multiply,
            0x38 => Self::LeftAlt,
            0x39 => Self::Space,
            0x3A => Self::CapsLock,
            0x3B => Self::F1,
            0x3C => Self::F2,
            0x3D => Self::F3,
            0x3E => Self::F4,
            0x3F => Self::F5,
            0x40 => Self::F6,
            0x41 => Self::F7,
            0x42 => Self::F8,
            0x43 => Self::F9,
            0x44 => Self::F10,
            0x45 => Self::NumLock,
            0x46 => Self::ScrollLock,
            0x47 => Self::NumPad7,
            0x48 => Self::NumPad8,
            0x49 => Self::NumPad9,
            0x4A => Self::Subtract,
            0x4B => Self::NumPad4,
            0x4C => Self::NumPad5,
            0x4D => Self::NumPad6,
            0x4E => Self::Add,
            0x4F => Self::NumPad1,
            0x50 => Self::NumPad2,
            0x51 => Self::NumPad3,
            0x52 => Self::NumPad0,
            0x53 => Self::Decimal,
            0x56 => Self::OEM102,
            0x57 => Self::F11,
            0x58 => Self::F12,
            0x9C => Self::NumPadEnter,
            0x9D => Self::RightControl,
            0xB5 => Self::Divide,
            0xB7 => Self::PrintScreen,
            0xB8 => Self::RightAlt,
            0xC5 => Self::Pause,
            0xC7 => Self::Home,
            0xC8 => Self::Up,
            0xC9 => Self::PageUp,
            0xCB => Self::Left,
            0xCD => Self::Right,
            0xCF => Self::End,
            0xD0 => Self::Down,
            0xD1 => Self::PageDown,
            0xD2 => Self::Insert,
            0xD3 => Self::Delete,
            _ => Self::Unknown(value),
        }
    }
}

/// Keyboard key states.
///
/// Helper methods are provided to check the state of key modifiers
/// (shift, control and alt).
pub struct KeyStates<'a> {
    kstate: &'a mut [c_char],
}

impl<'a> KeyStates<'a> {
    pub(crate) fn from(kstate: *mut c_char) -> Self {
        let slice = unsafe { std::slice::from_raw_parts_mut(kstate, 256) };
        Self { kstate: slice }
    }

    /// Check if a key is pressed down
    pub fn is_down(&self, key: Key) -> bool {
        (self.kstate[key.as_u8() as usize] as u8) & 0x80 != 0
    }

    /// Prevents default processing of the key
    pub fn reset_key(&mut self, key: Key) {
        self.kstate[key.as_u8() as usize] = 0;
    }

    /// Check if left shift key is pressed
    pub fn left_shift(&self) -> bool {
        self.is_down(Key::LeftShift)
    }

    /// Check if right shift key is pressed
    pub fn right_shift(&self) -> bool {
        self.is_down(Key::RightShift)
    }

    /// Check if any of of the shift keys are pressed
    pub fn shift(&self) -> bool {
        self.left_shift() || self.right_shift()
    }

    /// Check if left control key is pressed
    pub fn left_control(&self) -> bool {
        self.is_down(Key::LeftControl)
    }

    /// Check if right control key is pressed
    pub fn right_control(&self) -> bool {
        self.is_down(Key::RightControl)
    }

    /// Check if any of the control keys are pressed
    pub fn control(&self) -> bool {
        self.left_control() || self.right_control()
    }

    /// Check if left alt key is pressed
    pub fn left_alt(&self) -> bool {
        self.is_down(Key::LeftAlt)
    }

    /// Check if right alt key is pressed
    pub fn right_alt(&self) -> bool {
        self.is_down(Key::RightAlt)
    }

    /// Check if any of the alt keys are pressed
    pub fn alt(&self) -> bool {
        self.left_alt() || self.right_alt()
    }
}
