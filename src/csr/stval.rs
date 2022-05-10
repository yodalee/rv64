
//! Supervisor Trap-Value Register (stval) register

use crate::{csrw, csrr};

/// Stval Register
#[derive(Clone, Copy, Debug)]
pub struct Stval {
    bits: u64
}

impl Stval {
    /// Create Stval from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("stval", bits);
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
        let stval = self.bits();
        csrw!("stval", stval);
    }
}
