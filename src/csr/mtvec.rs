
//! Machine Trap-Vector Base-Address Register (mtvec) register

use crate::{csrw, csrr};

/// Mtvec Register
#[derive(Clone, Copy, Debug)]
pub struct Mtvec {
    bits: u64
}

impl Mtvec {
    /// Create Mtvec from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("mtvec", bits);
        Self { bits }
    }

    /// Return the content of the register as raw bits
    #[inline]
    fn bits(self) -> u64 {
        self.bits
    }

    /// Writes to the CPU register.
    #[inline]
    pub fn write(self) {
        let mtvec = self.bits();
        csrw!("mtvec", mtvec);
    }

    #[inline]
    pub fn set_addr(&mut self, addr: u64) {
        self.bits = addr;
    }
}

