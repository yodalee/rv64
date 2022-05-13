
/// flush the TLB
pub fn sfence_vma() {
    unsafe {
        // zero means flush all TLB entries.
        asm!("sfence.vma zero, zero")
    }
}

/// fence
pub fn sync_synchronize() {
    unsafe {
        // synchronize all memory operation
        asm!("fence")
    }
}

/// Stall CPU until interrupt
pub fn wfi() {
    unsafe {
        asm!("wfi")
    }
}
