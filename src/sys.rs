use core::fmt::Display;

use crate::constants::{CLIMB_MAX_STAMINA, MAX_DASHES};

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Facings {
    FacingLeft = -1,
    #[default]
    FacingRight = 1
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub const WHITE: Color = Color { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF };
    pub const RED: Color = Color { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF };
    pub const NORMAL_HAIR: Color = Color { r: 0xAC, g: 0x32, b: 0x32, a: 0xFF };
    pub const FLY_POWER_HAIR: Color = Color { r: 0xF2, g: 0xEB, b: 0x6D, a: 0xFF };
    pub const USED_HAIR: Color = Color { r: 0x00, g: 0x44, b: 0xB7, a: 0xFF };
    pub const FLASH_HAIR: Color = Self::WHITE;
    pub const TWO_DASHES_HAIR: Color = Color { r: 0xFF, g: 0x6D, b: 0xEF, a: 0xFF };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Inventory {
    pub max_dashes: u8,
    pub no_refills: bool,
    pub max_stamina: f32,
    pub gravity_mult: f32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            max_dashes: MAX_DASHES,
            no_refills: false,
            max_stamina: CLIMB_MAX_STAMINA,
            gravity_mult: 1.
        }
    }
}