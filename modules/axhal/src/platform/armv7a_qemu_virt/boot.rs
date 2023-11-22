use axconfig::TASK_STACK_SIZE;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!("
        mrc p15, 0, r9, c0, c0, 5
        and r9, r9, #0xffffff
        mov r10, r0

        ldr r8, ={boot_stack}
        add r8, r8, {boot_stack_size}
        mov sp, r8

        mov r0, r9
        mov r1, r10        
        bl {entry}
        b   .",
        boot_stack = sym BOOT_STACK,
        boot_stack_size = const TASK_STACK_SIZE,
        entry = sym crate::platform::rust_entry,
        options(noreturn),
    )
}
