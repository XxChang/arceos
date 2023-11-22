use core::arch::asm;

#[inline]
pub fn wait_for_irqs() {
    unsafe { asm!("wfi", options(nomem, nostack))}
}