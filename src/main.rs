#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
#![feature(proc_macro_hygiene)]

extern crate utf16_literal;
use utf16_literal::utf16;

use core::panic::PanicInfo;

mod uefi;
use uefi::Status;

#[no_mangle]
pub extern "efiapi" fn efi_main(_image: uefi::Handle, stable: uefi::SystemTable) -> Status {
    let stdout = stable.stdout();
    stdout.reset(false);
    stdout.output_string(utf16!("Hello, World!").as_ptr());

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
