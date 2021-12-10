
/// flush the TLB
pub fn sfence_vma() {
    unsafe {
        // zero means flush all TLB entries.
        asm!("sfence.vma zero, zero")
    }
}
