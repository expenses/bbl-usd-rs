use crate::{ar, cpp, ffi};
use std::ffi::{CStr, CString};
use std::fmt;

pub struct Token {
    pub(crate) ptr: *mut ffi::tf_Token_t,
}

impl Token {
    pub fn new(name: &str) -> Self {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::tf_Token_new(c_name.as_ptr(), &mut ptr);
            Self { ptr }
        }
    }

    pub fn text(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::tf_Token_GetText(self.ptr, &mut ptr);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        unsafe {
            ffi::tf_Token_dtor(self.ptr);
        }
    }
}

pub struct TokenRef {
    pub(crate) ptr: *mut ffi::tf_Token_t,
}

impl std::ops::Deref for TokenRef {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const TokenRef as *const Token) }
    }
}

impl fmt::Display for TokenRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

type OpenAssetFn = extern "C" fn(*const ffi::ar_ResolvedPath_t, *mut *mut ffi::ar_AssetSharedPtr_t);

type ResolveFn = extern "C" fn(*const ffi::std_String_t, *mut *mut ffi::ar_ResolvedPath_t);

type CreateIdentifierFn = extern "C" fn(
    *const ffi::std_String_t,
    *const ffi::ar_ResolvedPath_t,
    *mut *mut ffi::std_String_t,
);

pub struct Type {
    pub(crate) ptr: *const ffi::tf_Type_t,
}

impl Type {
    pub fn declare(name: &str) -> Self {
        let string = cpp::String::new(&CString::new(name).unwrap());

        unsafe {
            let mut ptr = std::ptr::null();

            ffi::tf_Type_Declare(string.ptr, &mut ptr);
            Self { ptr }
        }
    }

    pub fn set_factory(
        &self,
        create_identifier: CreateIdentifierFn,
        open_asset: OpenAssetFn,
        resolve: ResolveFn,
    ) {
        unsafe {
            ffi::ar_set_ar_resolver_factory(
                self.ptr,
                std::ptr::null_mut(),
                create_identifier as _,
                open_asset as _,
                resolve as _,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
    }
}
