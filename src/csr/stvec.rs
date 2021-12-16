
//! Supervisor Trap-Vector Base-Address Register (stvec) register

use crate::{csrw, csrr};
use bit_field::BitField;

pub enum StvecMode {
    Direct,
    Vectored,
}

/// Stvec Register
#[derive(Clone, Copy, Debug)]
pub struct Stvec {
    bits: u64
}

impl Stvec {
    /// Create Stvec from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("stvec", bits);
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
        let stvec = self.bits();
        csrw!("stvec", stvec);
    }

    /// Set vector mode
    #[inline]
    pub fn set_mode(&mut self, mode: StvecMode) {
        self.bits.set_bits(0..2, match mode {
            StvecMode::Direct => 0,
            StvecMode::Vectored => 1,
        });
    }

    /// Set base address
    #[inline]
    pub fn set_addr(&mut self, addr: u64) {
        assert!(addr % 4 == 0);
        self.bits.set_bits(3..64, addr.get_bits(3..64));
    }
}

