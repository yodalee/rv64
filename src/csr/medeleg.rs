
//! Machine Trap (Exception) Delegation Register (medeleg)

// TODO, give finer control with enum of interrupt and exception
// instead of wrapping value

use crate::{csrw, csrr};

/// medeleg
pub struct Medeleg {
    bits: u64
}

impl Medeleg {
    /// Create Mepc from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("medeleg", bits);
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
        csrw!("medeleg", bits);
    }
}
