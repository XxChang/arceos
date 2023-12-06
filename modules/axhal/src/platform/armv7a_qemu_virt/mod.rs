mod pl011;

pub mod time {

#[inline]
pub fn current_ticks() -> u64 {
    todo!()
}

#[inline]
pub fn ticks_to_nanos(ticks: u64) -> u64 {
    todo!()
}

#[inline]
pub fn nanos_to_ticks(nanos: u64) -> u64 {
    todo!()
}

}

pub mod console {
    pub use crate::platform::armv7a_qemu_virt::pl011::*;
}

pub mod misc {

pub fn terminate() -> ! {
    // https://wiki.osdev.org/Shutdown
    unsafe {
        ::core::arch::asm!(
        "
        mov r0, #1
        mcr p15, 0, r0, c15, c2, 0
        ",
        options(nostack, nomem, preserves_flags),
        );
    }
    crate::arch::halt();
    loop {
       crate::arch::halt(); 
    }
}

}

pub mod mem {
use crate::mem::MemRegion;

pub(crate) fn platform_regions() -> impl Iterator<Item = MemRegion> {
    crate::mem::default_free_regions().chain(crate::mem::default_mmio_regions())
}

}

pub fn platform_init() {

}

mod boot;

extern "C" {
    fn rust_main(cpu_id: usize, dtb: usize);
}

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    crate::mem::clear_bss();
    pl011::init_early();
    rust_main(cpu_id, dtb);
}
