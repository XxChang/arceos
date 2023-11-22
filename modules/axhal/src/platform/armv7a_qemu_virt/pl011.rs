//! PL011 UART.

use arm_pl011::pl011::Pl011Uart;
use memory_addr::PhysAddr;

use crate::mem::phys_to_virt;

const UART_BASE: PhysAddr = PhysAddr::from(axconfig::UART_PADDR);

static mut UART: Pl011Uart = Pl011Uart::new(phys_to_virt(UART_BASE).as_mut_ptr());

pub fn putchar(c: u8) {
    let uart = unsafe { &mut UART };
    match c {
        b'\n' => {
            uart.putchar(b'\r');
            uart.putchar(b'\n');
        }
        c => uart.putchar(c),
    }
}

pub fn getchar() -> Option<u8> {
   unsafe { UART.getchar() }
}

/// Initialize the UART
pub fn init_early() {
    unsafe { UART.init(); }
}
