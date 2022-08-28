
//! Machine Status Register (mstatus) register

use crate::{csrw, csrr};

/// Mstatus Register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: u64
}

impl Mstatus {
    /// Create Mstatus from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("mstatus", bits);
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
        let mstatus = self.bits();
        csrw!("mstatus", mstatus);
    }

    #[inline]
    pub fn get_mpp(self) -> Mode {
        if self.bits & (3 << 11) == (3 << 11) {
            Mode::MachineMode
        } else if self.bits & (1 << 11) == (1 << 11) {
            Mode::SupervisedMode
        } else {
            Mode::UserMode
        }
    }

    #[inline]
    pub fn set_mpp(&mut self, mode: Mode) {
        self.bits &= !(3 << 11);
        self.bits |= match mode {
            Mode::MachineMode =>    3 << 11,
            Mode::SupervisedMode => 1 << 11,
            Mode::UserMode =>       0 << 11,
        }
    }

    #[inline]
    pub fn enable_interrupt(&mut self, mode: Mode) {
        self.bits |= match mode {
            Mode::MachineMode =>    1 << 3,
            Mode::SupervisedMode => 1 << 1,
            Mode::UserMode =>       1 << 0,
        };
    }

    #[inline]
    pub fn disable_interrupt(&mut self, mode: Mode) {
        self.bits &= match mode {
            Mode::MachineMode =>    !(1 << 3),
            Mode::SupervisedMode => !(1 << 1),
            Mode::UserMode =>       !(1 << 0),
        }
    }
}

/// MPP mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// MachineMode, 0x11
    MachineMode,
    /// SupervisedMode, 0x01
    SupervisedMode,
    /// UserMode, 0x00
    UserMode,
}
