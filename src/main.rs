
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
#[link_section = ".mtvec_base"]
fn trap_handler() {

    unsafe {
        asm!("nop");
        asm!("csrr t0, mcause;");
        asm!("nop");
    }

}

#[no_mangle]
pub fn _start() {
    type FnPtr = fn() -> ();
    let th: FnPtr = trap_handler;

    unsafe{
    asm!("csrw mtvec, {}" ,
        in(reg) th);
    }

    loop {}
}