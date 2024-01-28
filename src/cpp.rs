use crate::ffi;
use std::ffi::{CStr, CString};

pub struct String {
    pub(crate) ptr: *mut ffi::std_String_t,
}

impl String {
    pub fn new(string: &str) -> Self {
        let bytes = string.as_bytes();
        let mut cpp_string = Self::default();
        cpp_string.resize(bytes.len());
        cpp_string.as_slice_mut().copy_from_slice(bytes);
        cpp_string
    }

    pub fn resize(&mut self, new_len: usize) {
        unsafe {
            ffi::std_String_resize(self.ptr, new_len as u64);
        }
    }

    pub fn ptr(&self) -> *const ffi::std_String_t {
        self.ptr
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        unsafe {
        ffi::std_String_size(self.ptr, &mut len);
        }
        len as usize
    }

    pub fn as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::std_String_data_const(self.ptr, &mut ptr);
            std::slice::from_raw_parts(ptr as *const u8, self.len())
        }
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        std::str::from_utf8(self.as_slice()).expect("invalid")
    }

    pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::std_String_data(self.ptr, &mut ptr);
            std::slice::from_raw_parts_mut(ptr as *mut u8, self.len())
        }
    }

    pub fn as_str_mut<'a>(&'a mut self) -> &'a mut str {
        std::str::from_utf8_mut(self.as_slice_mut()).expect("invalid")
    }
}

impl Default for String {
    fn default() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::std_String_default(&mut ptr);
            Self {
                ptr
            }
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            ffi::std_String_dtor(self.ptr);
        }
    }
}

unsafe impl Send for String {}
unsafe impl Sync for String {}

pub struct StringRef {
    pub(crate) ptr: *mut ffi::std_String_t,
}

impl StringRef {
    pub fn from_ptr(ptr: *const ffi::std_String_t) -> Self {
        Self { ptr: ptr as _ }
    }
}

impl std::ops::Deref for StringRef {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const StringRef as *const String) }
    }
}
