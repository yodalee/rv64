
//! Supervisor Address Translation and Protection Register (satp)

use crate::{csrw, csrr};
use bit_field::BitField;

#[derive(Clone, Copy, Debug)]
pub struct Satp {
    bits: u64
}

impl Satp {
    /// Create Satp from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("satp", bits);
        Self { bits }
    }

    /// Return the content of the register as raw bits
    #[inline]
    pub fn bits(self) -> u64 {
        self.bits
    }

    /// Writes to the CPU register.
    #[inline]
    pub fn write(self) {
        let bits = self.bits();
        csrw!("satp", bits);
    }

    #[inline]
    pub fn set_addr(&mut self, addr: u64) {
        self.bits.set_bits(0..44, (addr >> 12).get_bits(0..44));
    }

    /// Set mode
    #[inline]
    pub fn set_mode(&mut self, mode: SatpMode) {
        let value = match mode {
            SatpMode::ModeNone => 0,
            SatpMode::ModeSv39 => 8,
            SatpMode::ModeSv48 => 9,
            SatpMode::ModeSv57 => 10,
        };
        self.bits.set_bits(60..64, value);
    }
}

pub enum SatpMode {
    /// No translation or protection
    ModeNone,
    /// Sv39
    ModeSv39,
    /// Sv48
    ModeSv48,
    /// Sv57
    ModeSv57,
}
