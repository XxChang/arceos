use crate::{irq::IrqHandler, mem::phys_to_virt};
use arm_gicv2::{translate_irq, GicCpuInterface, GicDistributor, InterruptType, TriggerMode};
use axconfig::devices::{GICC_PADDR, GICD_PADDR, UART_IRQ};
use kspin::SpinNoIrq;
use memory_addr::PhysAddr;

/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 1024;

/// The timer IRQ number.
pub const TIMER_IRQ_NUM: usize = translate_irq(14, InterruptType::PPI).unwrap();

/// The UART IRQ number.
pub const UART_IRQ_NUM: usize = translate_irq(UART_IRQ, InterruptType::SPI).unwrap();

/// The GPIO IRQ number
pub const GPIO_IRQ_NUM: usize = translate_irq(axconfig::devices::GPIO_IRQ, InterruptType::SPI).unwrap();

const GICD_BASE: PhysAddr = pa!(GICD_PADDR);
const GICC_BASE: PhysAddr = pa!(GICC_PADDR);

static GICD: SpinNoIrq<GicDistributor> =
    SpinNoIrq::new(GicDistributor::new(phys_to_virt(GICD_BASE).as_mut_ptr()));

// per-CPU, no lock
static GICC: GicCpuInterface = GicCpuInterface::new(phys_to_virt(GICC_BASE).as_mut_ptr());

/// Enables or disables the given IRQ.
pub fn set_enable(irq_num: usize, enabled: bool) {
    trace!("GICD set enable: {} {}", irq_num, enabled);
    GICD.lock().set_enable(irq_num as _, enabled);
}

pub fn set_irq_trigger_by_level(irq_num: usize) {
    GICD.lock().configure_interrupt(irq_num, TriggerMode::Level);
}

pub fn set_priority(irq_num: u32, priority: u32) {
    let gicd_base = phys_to_virt(GICD_BASE).as_usize();
    let shift = (irq_num % 4) * 8;
    unsafe {
        let addr: *mut u32 = ( (gicd_base + 0x0400) as *mut u32 ).add((irq_num / 4) as usize);
        let mut value: u32 = core::ptr::read_volatile(addr);
        value &= !(0xff << shift);
        value |= priority << shift;
        core::ptr::write_volatile(addr, value);
    }
}

pub fn clear(interrupt: u32) {
    let gicd_base = phys_to_virt(GICD_BASE).as_usize();
    unsafe {
        core::ptr::write_volatile(
            ( (gicd_base + 0x0280) as *mut u32 ).add((interrupt / 32) as usize),
            1 << (interrupt % 32)
        );
    }
}

/// Registers an IRQ handler for the given IRQ.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(irq_num: usize, handler: IrqHandler) -> bool {
    trace!("register handler irq {}", irq_num);
    crate::irq::register_handler_common(irq_num, handler)
}

/// Dispatches the IRQ.
///
/// This function is called by the common interrupt handler. It looks
/// up in the IRQ handler table and calls the corresponding handler. If
/// necessary, it also acknowledges the interrupt controller after handling.
pub fn dispatch_irq(_unused: usize) {
    GICC.handle_irq(|irq_num| crate::irq::dispatch_irq_common(irq_num as _));
}

/// Initializes GICD, GICC on the primary CPU.
pub(crate) fn init_primary() {
    info!("Initialize GICv2...");
    GICD.lock().init();
    GICC.init();
}

/// Initializes GICC on secondary CPUs.
#[cfg(feature = "smp")]
pub(crate) fn init_secondary() {
    GICC.init();
}
