
//! Supervisor Cause Register (scause) register

use crate::{csrw, csrr};
use bit_field::BitField;

/// Scause Register
#[derive(Clone, Copy, Debug)]
pub struct Scause {
    bits: u64
}

impl Scause {
    /// Create Scause from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("scause", bits);
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
        let scause = self.bits();
        csrw!("scause", scause);
    }

    /// Is the trap an interrupt
    #[inline]
    pub fn is_interrupt(self) -> bool {
        self.bits.get_bit(63)
    }

    #[inline]
    pub fn get_code(self) -> u64 {
        self.bits.get_bits(0..63)
    }
}
