use core::{arch::asm, num::NonZeroUsize};

use kspin::SpinNoIrq;
use memory_addr::PhysAddr;

use crate::{mem::phys_to_virt, misc::terminate};

const GPIO_BASE: PhysAddr = pa!(axconfig::devices::PL061_PADDR);
const GPIO_BASE_ADDR: usize = phys_to_virt(GPIO_BASE).as_usize();
const GPIO_IE_OFFSET: usize = 0x410;

pub fn init() {
    log::info!("Initializing GPIO IRQ handler");
    #[cfg(feature = "irq")]
    super::gic::set_irq_trigger_by_level(super::gic::GPIO_IRQ_NUM);
    #[cfg(feature = "irq")]
    super::gic::set_priority(super::gic::GPIO_IRQ_NUM as u32, 0);
    #[cfg(feature = "irq")]
    super::gic::clear(super::gic::GPIO_IRQ_NUM as u32);
    
    unsafe { ((GPIO_BASE_ADDR + GPIO_IE_OFFSET) as *mut u8).write_volatile(0b0000_1000) };
    // 使能GPIO_IRQ_NUM
    super::gic::register_handler(super::gic::GPIO_IRQ_NUM, handler);
}

fn handler() {
    axlog::ax_println!("GPIO IRQ triggered");
    terminate();
}