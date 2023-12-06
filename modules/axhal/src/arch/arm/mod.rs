use core::arch::asm;

/// Makes the current CPU to ignore interrupts
#[inline]
pub fn disable_irqs() {
    unsafe { asm!("mrs r1, cpsr
                   orr r1, r1, #0x80
                   msr cpsr, r1", options(nomem, nostack)) };
}

#[inline]
pub fn wait_for_irqs() {
    unsafe { asm!("wfi", options(nomem, nostack))}
}

/// Halt the current CPU.
#[inline]
pub fn halt() {
    disable_irqs();
    unsafe { asm!("wfi", options(nomem, nostack)) };
}
