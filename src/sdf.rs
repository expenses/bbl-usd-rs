use crate::{ffi, tf, cpp, usd};
use std::ffi::{CStr, CString};

pub struct AssetPath {
    pub(crate) ptr: *mut ffi::sdf_AssetPath_t,
}

impl AssetPath {
    pub fn from_path(path: &CStr) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_AssetPath_from_path(path.as_ptr(), &mut ptr);
            Self { ptr }
        }
    }

    pub fn asset_path(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_AssetPath_GetAssetPath(self.ptr, &mut ptr);
            let cstr = CStr::from_ptr(ptr).to_str().unwrap();
            cstr
        }
    }

    pub fn resolved_path(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_AssetPath_GetResolvedPath(self.ptr, &mut ptr);
            let cstr = CStr::from_ptr(ptr).to_str().unwrap();
            cstr
        }
    }
}

pub struct AssetPathRef {
    pub(crate) ptr: *mut ffi::sdf_AssetPath_t,
}

impl std::ops::Deref for AssetPathRef {
    type Target = AssetPath;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const AssetPathRef as *const AssetPath) }
    }
}

impl Drop for AssetPath {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_AssetPath_dtor(self.ptr);
        }
    }
}

pub struct Path {
    pub(crate) ptr: *mut ffi::sdf_Path_t,
}

impl Path {
    pub fn text(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_Path_GetText(self.ptr, &mut ptr);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let cs = std::ffi::CString::new(value).unwrap();

        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_Path_from_string(cs.as_ptr() as *mut i8, &mut ptr);
            Path { ptr }
        }
    }
}

pub struct PathRef {
    pub(crate) ptr: *mut ffi::sdf_Path_t,
}

impl std::ops::Deref for PathRef {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const PathRef as *const Path) }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_Path_dtor(self.ptr);
        }
    }
}

pub struct PathVector {
    pub(crate) ptr: *mut ffi::sdf_PathVector_t,
}

impl PathVector {
    pub fn size(&self) -> usize {
        unsafe {
            let mut size = 0;
            ffi::sdf_PathVector_size(self.ptr, &mut size);
            size
        }
    }

    pub fn at(&self, index: usize) -> PathRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_PathVector_op_index(self.ptr, index, &mut ptr);
            PathRef { ptr: ptr as _ }
        }
    }

    pub fn iter(&self) -> PathVectorIterator {
        PathVectorIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl Drop for PathVector {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_PathVector_dtor(self.ptr);
        }
    }
}

impl Default for PathVector {
    fn default() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_PathVector_default(&mut ptr);
            PathVector { ptr }
        }
    }
}

pub struct PathVectorIterator<'a> {
    vec: &'a PathVector,
    current: usize,
    end: usize,
}

impl<'a> Iterator for PathVectorIterator<'a> {
    type Item = PathRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let cur = self.current;
            self.current += 1;
            Some(self.vec.at(cur))
        }
    }
}

pub struct ValueTypeName {
    pub(crate) ptr: *mut ffi::sdf_ValueTypeName_t,
}

impl ValueTypeName {
    pub fn as_token(&self) -> tf::Token {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_ValueTypeName_GetAsToken(self.ptr, &mut ptr);
            tf::Token { ptr }
        }
    }

    pub fn role(&self) -> tf::TokenRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_ValueTypeName_GetRole(self.ptr, &mut ptr);
            tf::TokenRef { ptr: ptr as _ }
        }
    }
}

impl Drop for ValueTypeName {
    fn drop(&mut self) {
        unsafe {
            ffi::sdf_ValueTypeName_dtor(self.ptr);
        }
    }
}

pub enum Variability {
    Varying,
    Uniform,
}

pub struct LayerOffset {
    pub(crate) ptr: *mut ffi::sdf_LayerOffset_t
}

impl Default for LayerOffset {
    fn default() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_LayerOffset_default(&mut ptr);
            Self {
                ptr
            }
        }
    }
}

pub struct FileFormatArguments {
    pub(crate) ptr: *mut ffi::sdf_FileFormatFileFormatArguments_t
}

impl FileFormatArguments {
    fn default() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_FileFormatFileFormatArguments_default(&mut ptr);
            Self {
                ptr
            }
        }
    }
}

pub struct Layer;

impl Layer {
    pub fn create_anonymous(tag: &str) -> LayerRefPtr {
        let tag = std::ffi::CString::new(tag).unwrap();
        let tag = cpp::String::new(&tag);
        let arguments = FileFormatArguments::default();
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_Layer_CreateAnonymous(tag.ptr, arguments.ptr, &mut ptr);
            LayerRefPtr {
                ptr
            }
        }
    }

    pub fn find_or_open(identifier: &str) -> LayerRefPtr {
        let identifier = std::ffi::CString::new(identifier).unwrap();
        let identifier = cpp::String::new(&identifier);
        let arguments = FileFormatArguments::default();
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::sdf_Layer_FindOrOpen(identifier.ptr, arguments.ptr, &mut ptr);
            LayerRefPtr {
                ptr
            }
        }
    }
}

pub struct LayerRefPtr {
    pub(crate) ptr: *mut ffi::sdf_LayerRefPtr_t
}

impl LayerRefPtr {
    pub fn get_identifier(&self) -> cpp::String {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::sdf_LayerRefPtr_GetIdentifier(self.ptr, &mut ptr);
            cpp::String {
                ptr: ptr as *mut ffi::std_String_t
            }
        }
    }

    pub fn import_from_str(&self, string: &cpp::String) -> bool {
        unsafe {
            let mut result = false;
            ffi::sdf_LayerRefPtr_ImportFromString(self.ptr, string.ptr, &mut result);
            result
        }
    }

    pub fn insert_sub_layer_path(&self, path: cpp::String, index: i32) {
        unsafe {
            ffi::sdf_LayerRefPtr_InsertSubLayerPath(self.ptr, path.ptr, index);
        }
        std::mem::forget(path);
    }

    pub fn export_to_string(&self) -> Option<cpp::String> {
        unsafe {
            let mut string = cpp::String::default();
            let mut result = false;
            ffi::sdf_LayerRefPtr_ExportToString(self.ptr, &mut string.ptr, &mut result);
            if result {
                Some(string)
            } else {
                None
            }
        }
    }
}

unsafe impl Send for LayerRefPtr {}
unsafe impl Sync for LayerRefPtr {}

pub struct LayerHandle {
    pub(crate) ptr: *mut ffi::sdf_LayerHandle_t
}

impl LayerHandle {
    pub fn insert_sub_layer_path(&self, path: cpp::String, index: i32) {
        unsafe {
            ffi::sdf_LayerHandle_InsertSubLayerPath(self.ptr, path.ptr, index);
        }
        std::mem::forget(path);
    }

    pub fn export_to_string(&self) -> Option<cpp::String> {
        unsafe {
            let mut string = cpp::String::default();
            let mut result = false;
            ffi::sdf_LayerHandle_ExportToString(self.ptr, &mut string.ptr, &mut result);
            if result {
                Some(string)
            } else {
                None
            }
        }
    }
}

unsafe impl Send for LayerHandle {}
unsafe impl Sync for LayerHandle {}
