
//! Machine Exception Program Counter (mepc) register

use crate::{csrw, csrr};

/// Mepc Register
#[derive(Clone, Copy, Debug)]
pub struct Mepc {
    bits: u64
}

impl Mepc {
    /// Create Mepc from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CSR
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("mepc", bits);
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
        let mepc = self.bits();
        csrw!("mepc", mepc);
    }
}
