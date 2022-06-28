#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;
use core::arch::asm;
use volatile_register::{RW, RO};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

/* UART 1 base addr: 0x1002_3000
    txdata: offset 0,
    rxdata: offset 4,
    txctrl: offset 8,
    rxctrl: offset 0xC,
    ie: offset 0x10,
    ip: 0x14,
    baud: 0x18
*/

#[repr(C)]   //Prevent struct field re-ordering
struct UartOne{
    pub txdata: RW<u32>,
    pub rxdata: RO<u32>,
    pub txctrl: RW<u32>,
    pub rxctrl: RW<u32>,
    pub ie: RW<u32>,
    pub ip: RO<u32>,
    pub baud: RW<u32>
}

fn get_uartone() -> &'static mut UartOne{
    unsafe { &mut *(0x1002_3000 as *mut UartOne) }
}

#[no_mangle]
 pub extern "C" fn _start() -> ! {

    let mut uart1 = get_uartone();
    
    unsafe {
        asm! ("nop");
        asm! ("addi x1, x1, 2");
    };

    loop {}
}
