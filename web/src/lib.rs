#![feature(allocator_api)]

extern crate clumsy;

use clumsy::lexer::Lexer;
use clumsy::parser;
use clumsy::DeBruijnIndex;
use std::ffi::{CStr, CString};
use std::heap::{Alloc, Global, Layout};
use std::os::raw::{c_char, c_void};
use std::{mem, ptr};

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    Layout::from_size_align(size, mem::align_of::<u8>())
        .ok()
        .and_then(|layout| unsafe { Global.alloc(layout) }.ok())
        .map(|addr| addr.as_ptr() as *mut c_void)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub fn dealloc(ptr: *mut c_void, size: usize) {
    if let (Some(ptr), Ok(layout)) = (
        ptr::NonNull::new(ptr),
        Layout::from_size_align(size, mem::align_of::<u8>()),
    ) {
        unsafe { Global.dealloc(ptr.as_opaque(), layout) }
    }
}

#[no_mangle]
pub fn eval(ptr: *const c_char) -> *mut c_char {
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .ok()
        .and_then(|source| parser::parse(Lexer::new(source)).ok())
        .and_then(|ref ast| {
            let dbi = DeBruijnIndex::from_ast(ast);
            CString::new(format!("{:#?}", dbi)).ok()
        })
        .map(|result| result.into_raw())
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub fn free_result(ptr: *mut c_char) {
    unsafe { CString::from_raw(ptr) };
}
