
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
        if self.bits.get_bit(8) {
            Mode::SupervisedMode
        } else {
            Mode::UserMode
        }
    }

    #[inline]
    pub fn set_spp(&mut self, mode: Mode) {
        self.bits.set_bit(8, match mode {
            Mode::SupervisedMode => true,
            Mode::UserMode =>       false,
        });
    }

    pub fn get_spie(self) -> bool {
        self.bits.get_bit(5)
    }

    pub fn set_spie(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }

    #[inline]
    pub fn get_sie(&self) -> bool {
        self.bits.get_bit(1)
    }

    #[inline]
    pub fn enable_interrupt(&mut self, mode: Mode) {
        let offset = match mode {
            Mode::SupervisedMode => 1,
            // FIXME no such mode interrupt
            Mode::UserMode =>       0,
        };
        self.bits.set_bit(offset, true);
    }

    #[inline]
    pub fn disable_interrupt(&mut self, mode: Mode) {
        let offset = match mode {
            Mode::SupervisedMode => 1,
            // FIXME no such mode interrupt
            Mode::UserMode =>       0,
        };
        self.bits.set_bit(offset, false);
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
