#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let  mut x: i32 = 3;

 loop{
     
 }

}
