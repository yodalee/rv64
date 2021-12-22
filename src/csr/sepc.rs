
//! Supervisor Exception Program Counter (sepc) register

use crate::{csrw, csrr};

/// Sepc Register
#[derive(Clone, Copy, Debug)]
pub struct Sepc {
    bits: u64
}

impl Sepc {
    /// Create Sepc from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CSR
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("sepc", bits);
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
        let sepc = self.bits();
        csrw!("sepc", sepc);
    }
}
