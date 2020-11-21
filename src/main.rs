#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;

mod uefi;
use uefi::Status;

#[no_mangle]
pub extern "efiapi" fn efi_main(_image: uefi::Handle, stable: uefi::SystemTable) -> Status {
    let hello = "Hello, World!".as_bytes();
    let mut buf = [0u16; 15];

    for i in 0..hello.len() {
        buf[i] = hello[i] as u16;
    }

    let stdout = stable.con_out;
    unsafe {
        ((*stdout).reset)(&*stdout, false);
        ((*stdout).output_string)(&*stdout, buf.as_ptr());
    }

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
