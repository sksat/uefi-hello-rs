use core::ffi::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Handle(*mut c_void);

#[repr(usize)]
pub enum Status {
    Success = 0,
}

#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
}

#[repr(C)]
//#[repr(packed)] // packedを付けると32bit縮められて死ぬ
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_handle: Handle,
    pub _con_in: usize,
    pub console_out_handle: Handle,
    pub con_out: *const SimpleTextOutputProtocol,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(this: &SimpleTextOutputProtocol, _e: bool) -> Status,
    pub output_string:
        unsafe extern "efiapi" fn(this: &SimpleTextOutputProtocol, string: *const u16) -> Status,
    _resb2: u128,
}
