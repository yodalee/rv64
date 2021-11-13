
//! Machine Trap (Interrupt) Delegation Register (mideleg)

// TODO, give finer control with enum of interrupt and exception instead of wrapping value

use crate::{csrw, csrr};

/// mideleg
pub struct Mideleg {
    bits: u64
}

impl Mideleg  {
    /// Create Mepc from raw bits
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Reads the CPU register
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("mideleg", bits);
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
        let mideleg = self.bits();
        csrw!("mideleg", mideleg);
    }
}
