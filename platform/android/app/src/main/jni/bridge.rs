#![allow(improper_ctypes_definitions)]

use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn route_dns(_ptr: *const u8, _len: usize) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn Java_com_voidblock_DnsProxy_nativeResolve(
    _env: *mut c_void,
    _this: *mut c_void,
    _query: *const u8,
    _len: usize,
) -> *mut u8 {
    std::ptr::null_mut()
}
