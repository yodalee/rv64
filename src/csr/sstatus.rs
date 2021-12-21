
//! Supervisor Status Register (sstatus) register

use crate::{csrw, csrr};
use bit_field::BitField;

/// Sstatus Register
#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: u64
}

impl Sstatus {
    /// Create Sstatus from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("sstatus", bits);
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
        let sstatus = self.bits();
        csrw!("sstatus", sstatus);
    }

    #[inline]
    pub fn get_spp(self) -> Mode {
        if self.bits & (1 << 8) == (1 << 8) {
            Mode::SupervisedMode
        } else {
            Mode::UserMode
        }
    }

    #[inline]
    pub fn set_mpp(&mut self, mode: Mode) {
        self.bits &= !(1 << 8);
        self.bits |= match mode {
            Mode::SupervisedMode => (1 << 8),
            Mode::UserMode =>       (0 << 8),
        }
    }

    #[inline]
    pub fn get_sie(&self) -> bool {
        self.bits.get_bit(1)
    }

    #[inline]
    pub fn enable_interrupt(&mut self, mode: Mode) {
        self.bits |= match mode {
            Mode::SupervisedMode => (1 << 1),
            Mode::UserMode =>       (1 << 0),
        };
    }

    #[inline]
    pub fn disable_interrupt(&mut self, mode: Mode) {
        self.bits &= match mode {
            Mode::SupervisedMode => !(1 << 1),
            Mode::UserMode =>       !(1 << 0),
        }
    }
}

/// SPP mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// SupervisedMode, 0x01
    SupervisedMode,
    /// UserMode, 0x00
    UserMode,
}
