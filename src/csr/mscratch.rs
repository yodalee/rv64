
//! Machine Scratch (mscratch) register

use crate::{csrw, csrr};

/// Hart ID Register
#[derive(Clone, Copy, Debug)]
pub struct Mscratch {
    bits: u64
}

impl Mscratch {
    #[inline]
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    #[inline]
    pub fn bits(self) -> u64 {
        self.bits
    }
}

#[inline]
pub fn read() -> Mscratch {
    let bits: u64;
    csrr!("mscratch", bits);
    Mscratch { bits }
}

#[inline]
pub fn write(mscratch: Mscratch) {
    let mscratch = mscratch.bits();
    csrw!("mscratch", mscratch);
}
