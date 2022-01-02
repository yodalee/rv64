
//! Supervisor Interrupt Register (sip)

use crate::{csrw, csrr};
use bit_field::BitField;

/// Supervisor Interrupt Pending Register (sip)
#[derive(Clone, Copy, Debug)]
pub struct Sip {
    bits: u64
}

impl Sip {
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("sip", bits);
        Self { bits }
    }

    #[inline]
    pub fn bits(self) -> u64 {
        self.bits
    }

    #[inline]
    pub fn write(self) {
        let bits = self.bits();
        csrw!("sip", bits);
    }

    #[inline]
    pub fn set_pending(&mut self, i: usize) {
        self.bits.set_bit(i, true);
    }

    #[inline]
    pub fn clear_pending(&mut self, i: usize) {
        self.bits.set_bit(i, false);
    }
}
