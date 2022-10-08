#![no_std]
#![no_main]

use core::panic::PanicInfo;

// eventually we would like to print the panic message
// maybe some other functionality
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop{}
}

// entry point for the runtime system
// _start is the deafult entry point name for most systems
// ! means the function is diverging (does not return)
// instead exits with a system call
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
