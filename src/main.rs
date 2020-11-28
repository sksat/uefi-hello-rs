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

static mut GLOBAL_STDOUT: Option<&uefi::SimpleTextOutputProtocol> = None;

#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image: uefi::Handle,
    stable: &'static uefi::SystemTable,
) -> Status {
    let stdout = stable.stdout();
    unsafe {
        GLOBAL_STDOUT = Some(stdout);
    }
    stdout.reset(false);
    //panic!("");
    stdout.output_string(utf16!("Hello, World!").as_ptr());

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    unsafe {
        if let Some(s) = GLOBAL_STDOUT {
            s.output_string(utf16!("panic").as_ptr());
        }
    }
    loop {}
}
