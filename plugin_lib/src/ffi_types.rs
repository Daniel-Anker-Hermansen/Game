//! This module contains ffi version of common types
//! These can be used to ensure the same memory layout
//! across crates, most importantly between the main
//! game and any plugin. All of these are FFI safe.

use core::slice;
use std::marker::PhantomData;

use libloading::{Library, Symbol, Error};

/// FFI version for string. 
#[repr(C)]
pub struct FFIString {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

unsafe impl Send for FFIString { }
unsafe impl Sync for FFIString { }

impl From<String> for FFIString {
    fn from(mut value: String) -> Self {
        let result = FFIString { ptr: value.as_mut_ptr(), len: value.len(), cap: value.capacity() };
        // Forget to not deallocate content.
        std::mem::forget(value);
        result
    }
}

impl From<FFIString> for String {
    fn from(value: FFIString) -> Self {
        let result = unsafe { String::from_raw_parts(value.ptr, value.len, value.cap) };
        // Forget to not deallocate content.
        std::mem::forget(value);
        result
    }
}

impl Drop for FFIString {
    fn drop(&mut self) {
        unsafe { String::from_raw_parts(self.ptr, self.len, self.cap) };
    }
}

/// FFI version for &'static str
#[repr(C)]
pub struct FFIStaticStr {
    ptr: *const u8,
    len: usize,
}

unsafe impl Send for FFIStaticStr { }
unsafe impl Sync for FFIStaticStr { }

impl FFIStaticStr {
    pub const fn from_str(value: &'static str) -> FFIStaticStr {
        FFIStaticStr { ptr: value.as_ptr(), len: value.len() }
    }
}

impl From<FFIStaticStr> for &'static str {
    fn from(value: FFIStaticStr) -> Self {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(value.ptr, value.len)) }
    }
}

impl From<&FFIStaticStr> for &'static str {
    fn from(value: &FFIStaticStr) -> Self {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(value.ptr, value.len)) }
    }
}

/// FFI version for Vec.
/// Use the to_vec function to convert.
/// If you do not convert the contents are leaked.
/// The inner type must be FFI safe.
#[repr(C)]
pub struct FFIVec {
    ptr: *mut (),
    len: usize,
    cap: usize,
}

impl<T> From<Vec<T>> for FFIVec {
    fn from(mut value: Vec<T>) -> Self {
        let result = FFIVec { ptr: value.as_mut_ptr() as _, len: value.len(), cap: value.capacity() };
        // Forget to not deallocate content
        std::mem::forget(value);
        result
    }
}

impl FFIVec {
    /// Converts to Vec<T>.
    /// This is unsafe as the inner type
    /// must match what is what created with
    /// exactly and must be FFI safe.
    pub unsafe fn to_vec<T>(self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(self.ptr as _, self.len, self.cap) } 
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct FFISlice<'lib> {
    ptr: *const (),
    len: usize,
    _phantom: PhantomData<&'lib ()>,
}

unsafe impl Sync for FFISlice<'_> { }
unsafe impl Send for FFISlice<'_> { }

impl<'lib> FFISlice<'lib> {
    pub unsafe fn to_slice<T>(&self) -> &'lib [T] {
        unsafe { slice::from_raw_parts(self.ptr.cast(), self.len) }
    }

    pub fn load_from_lib(lib: &'lib Library, symbol: &[u8]) -> Result<Symbol<'lib, &'lib FFISlice<'lib>>, Error> {
        unsafe { lib.get(symbol) } 
    }
}

impl<T> From<&'static [T]> for FFISlice<'static> {
    fn from(value: &'static [T]) -> Self {
        slice_to_ffi(value)
    }
}

pub const fn slice_to_ffi<T>(value: &'static [T]) -> FFISlice<'static> {
        FFISlice { ptr: value.as_ptr().cast(), len: value.len(), _phantom: PhantomData }
}

pub unsafe fn load_slice_from_lib<'lib, T>(lib: &'lib Library, symbol: &[u8]) -> &'lib [T] {
    FFISlice::load_from_lib(lib, symbol).map(|symbol| FFISlice::clone(&symbol).to_slice()).unwrap_or(&[])
}
