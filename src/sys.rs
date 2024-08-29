use core::fmt::Display;

use crate::constants::{CLIMB_MAX_STAMINA, MAX_DASHES};

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Facings {
    Left = -1,
    #[default]
    Right = 1
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
    pub const WHITE: Color = Color::CLST_ColorFromU32(0xFFFF_FFFF);
    pub const RED: Color = Color::CLST_ColorFromU32(0xFF00_00FF);
    pub const NORMAL_HAIR: Color = Color::CLST_ColorFromU32(0xAC3232FF);
    pub const FLY_POWER_HAIR: Color = Color::CLST_ColorFromU32(0xF2EB6DFF);
    pub const USED_HAIR: Color = Color::CLST_ColorFromU32(0x44B7FF);
    pub const FLASH_HAIR: Color = Self::WHITE;
    pub const TWO_DASHES_HAIR: Color = Color::CLST_ColorFromU32(0xFF6DEFFF);
    pub const TRANSPARENT: Color = Color::CLST_ColorFromU32(0);
}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

impl Color {
    #[no_mangle]
    pub const extern "C" fn CLST_NewColor(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a
        }
    }

    #[no_mangle]
    pub const extern "C" fn CLST_ColorFromU32(rgba: u32) -> Self {
        Self {
            r: (rgba >> 24) as u8,
            g: (rgba >> 16) as u8,
            b: (rgba >> 8) as u8,
            a: rgba as u8
        }
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