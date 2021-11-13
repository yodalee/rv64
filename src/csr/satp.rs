
//! Supervisor Address Translation and Protection Register (satp)

use crate::{csrw, csrr};

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
}
