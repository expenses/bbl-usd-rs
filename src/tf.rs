use crate::{cpp, ffi};
use std::ffi::{c_void, CStr, CString};
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

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.text()
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

impl AsRef<str> for TokenRef {
    fn as_ref(&self) -> &str {
        self.text()
    }
}

type OpenAssetFn = extern "C" fn(*const ffi::ar_ResolvedPath_t, *mut *mut ffi::ar_AssetSharedPtr_t);

type ResolveFn = extern "C" fn(*const ffi::std_String_t, *mut *mut ffi::ar_ResolvedPath_t);

type CreateIdentifierFn = extern "C" fn(
    *const ffi::std_String_t,
    *const ffi::ar_ResolvedPath_t,
    *mut *mut ffi::std_String_t,
);

type GetExtensionFn = extern "C" fn(*const ffi::std_String_t, *mut *mut ffi::std_String_t);

type GetModificationTimestampFn = extern "C" fn(
    *const ffi::std_String_t,
    *const ffi::ar_ResolvedPath_t,
    *mut *mut ffi::ar_Timestamp_t,
);

type CloseWritableAssetFn = extern "C" fn(*mut c_void) -> bool;

type OpenWritableAssetFn =
    extern "C" fn(*const ffi::ar_ResolvedPath_t, ffi::ar_ResolvedWriteMode) -> *mut c_void;

type WriteWritableAssetFn = extern "C" fn(*mut c_void, *const c_void, usize, usize) -> usize;

pub struct Type {
    pub(crate) ptr: *const ffi::tf_Type_t,
}

impl Type {
    pub fn declare(name: &str) -> Self {
        let string = cpp::String::new(name);

        unsafe {
            let mut ptr = std::ptr::null();

            ffi::tf_Type_Declare(string.ptr, &mut ptr);
            Self { ptr }
        }
    }

    pub fn set_factory(
        &self,
        create_identifier_for_new_asset: Option<CreateIdentifierFn>,
        create_identifier: Option<CreateIdentifierFn>,
        open_asset: Option<OpenAssetFn>,
        resolve_for_new_asset: Option<ResolveFn>,
        resolve: Option<ResolveFn>,
        get_extension: Option<GetExtensionFn>,
        get_timestamp: Option<GetModificationTimestampFn>,
        close_writable_asset: Option<CloseWritableAssetFn>,
        open_writable_asset: Option<OpenWritableAssetFn>,
        write_writable_asset: Option<WriteWritableAssetFn>,
    ) {
        unsafe {
            ffi::ar_set_ar_resolver_factory(
                self.ptr,
                match create_identifier_for_new_asset {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match create_identifier {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match open_asset {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match resolve {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match resolve_for_new_asset {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                std::ptr::null_mut(),
                match get_extension {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match get_timestamp {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match close_writable_asset {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match open_writable_asset {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
                match write_writable_asset {
                    Some(ptr) => ptr as _,
                    None => std::ptr::null_mut(),
                },
            );
        }
    }
}
