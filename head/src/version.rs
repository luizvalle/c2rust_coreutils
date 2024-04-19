use ::libc;
#[no_mangle]
pub static mut Version: *const libc::c_char = b"9.5.8-be3a2\0" as *const u8
    as *const libc::c_char;
