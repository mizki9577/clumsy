#![feature(allocator_api)]

use std::ffi::{CStr, CString};
use std::heap::{Alloc, Global, Layout};
use std::os::raw::{c_char, c_void};
use std::{mem, ptr};

#[no_mangle]
pub fn alloc(size: usize) -> *const c_void {
    Layout::from_size_align(size, std::mem::align_of::<u8>())
        .ok()
        .and_then(|layout| unsafe { Global.alloc(layout.clone()) }.ok())
        .map(|addr| addr.as_ptr() as *const c_void)
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub fn eval(ptr: *const c_char) -> *const c_char {
    CString::new(" Time to de-evilize!").unwrap().as_ptr()
}
