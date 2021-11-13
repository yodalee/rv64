
//! Hart ID (mhartid) register

use crate::csrr;

/// Hart ID Register
#[derive(Clone, Copy, Debug)]
pub struct Mhartid {
    bits: u64
}

impl Mhartid {
    #[inline]
    pub fn from_read() -> Self {
        let bits: u64;
        csrr!("mhartid", bits);
        Self { bits }
    }

    #[inline]
    pub fn bits(self) -> u64 {
        self.bits
    }
}
