use std::convert::TryFrom;

use callback::IntoRust;

#[derive(Debug, Copy, Clone)]
pub struct KeyCode(u32);

impl KeyCode {
    pub fn ch(&self) -> char {
        char::try_from(self.0 & 0x0FFFFFFF).unwrap()
    }

    pub fn is_shift(&self) -> bool {
        self.0 & 0x10000000 != 0
    }

    pub fn is_ctrl(&self) -> bool {
        self.0 & 0x20000000 != 0
    }

    pub fn is_alt(&self) -> bool {
        self.0 & 0x40000000 != 0
    }

    pub fn is_sys(&self) -> bool {
        self.0 & 0x80000000 != 0
    }
}

impl IntoRust<KeyCode> for i32 {
    fn into_rust(self) -> KeyCode {
        KeyCode(self as u32)
    }
}
