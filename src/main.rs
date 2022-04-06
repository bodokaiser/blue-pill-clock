#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    let _x = 42;

    loop {}
}

#[link_section = ".vector_table.reset"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
