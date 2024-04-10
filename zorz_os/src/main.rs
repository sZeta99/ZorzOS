#![no_std]
#![no_main]

mod vga_buffer;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("ZORZ_OS");
    println!("Buongiorno Mondo{}", "!");
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
