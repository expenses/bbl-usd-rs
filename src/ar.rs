use crate::{cpp, ffi};
use std::ffi::CString;

pub struct ResolvedPath {
    pub(crate) ptr: *mut ffi::ar_ResolvedPath_t,
}

impl ResolvedPath {
    pub fn from_raw(ptr: *const ffi::ar_ResolvedPath_t) -> Self {
        Self { ptr: ptr as _ }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            let mut is_empty = false;
            ffi::ar_ResolvedPath_IsEmpty(self.ptr, &mut is_empty);
            is_empty
        }
    }

    pub fn get_path_string(&self) -> cpp::StringRef {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::ar_ResolvedPath_GetPathString(
                self.ptr,
                (&mut ptr) as *mut *mut ffi::std_String_t as _,
            );
            cpp::StringRef { ptr }
        }
    }
}
