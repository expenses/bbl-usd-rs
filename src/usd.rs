use crate::ffi;
use crate::sdf;
use crate::tf;
use crate::vt;
use crate::cpp;

use std::ffi::{CStr, CString};
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    StageOpen { filename: String },
    StageCreateNew { filename: String },
    NoPrimAtPath { path: String },
}

pub struct Stage {}

impl Stage {
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<StageRefPtr, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let initial_load_set = ffi::usd_StageInitialLoadSet_usd_StageInitialLoadSet_LoadAll;
            let filename = filename.as_ref().to_string_lossy().to_string();
            let c_filename = CString::new(filename.clone()).unwrap();
            ffi::usd_Stage_Open(
                c_filename.as_ptr() as *mut std::ffi::c_char,
                initial_load_set,
                &mut ptr,
            );

            let mut is_invalid = true;
            ffi::usd_StageRefPtr_is_invalid(ptr, &mut is_invalid);

            if is_invalid {
                Err(Error::StageOpen { filename })
            } else {
                Ok(StageRefPtr { ptr })
            }
        }
    }

    pub fn create_new<P: AsRef<Path>>(filename: P) -> Result<StageRefPtr, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let initial_load_set = ffi::usd_StageInitialLoadSet_usd_StageInitialLoadSet_LoadAll;
            let filename = filename.as_ref().to_string_lossy().to_string();
            let cpp_filename = cpp::String::new(&filename);
            ffi::usd_Stage_CreateNew(
                cpp_filename.ptr,
                initial_load_set,
                &mut ptr,
            );

            let mut is_invalid = true;
            ffi::usd_StageRefPtr_is_invalid(ptr, &mut is_invalid);

            if is_invalid {
                Err(Error::StageCreateNew { filename })
            } else {
                Ok(StageRefPtr { ptr })
            }
        }
    }

    pub fn create_in_memory() -> StageRefPtr {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Stage_CreateInMemory(
                ffi::usd_StageInitialLoadSet_usd_StageInitialLoadSet_LoadAll,
                &mut ptr);
                StageRefPtr {
                ptr
            }
        }
    }
}

#[derive(Debug)]
pub struct StageRefPtr {
    ptr: *mut ffi::usd_StageRefPtr_t,
}

impl StageRefPtr {
    pub fn pseudo_root(&self) -> Prim {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_GetPseudoRoot(self.ptr, &mut ptr);
            Prim { ptr }
        }
    }

    pub fn prim_at_path<P: Into<sdf::Path>>(&self, path: P) -> Result<Prim, Error> {
        let path = path.into();
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_GetPrimAtPath(self.ptr, path.ptr, &mut ptr);
            let mut valid = false;
            ffi::usd_Prim_IsValid(ptr, &mut valid);

            if valid {
                Ok(Prim { ptr })
            } else {
                Err(Error::NoPrimAtPath {
                    path: path.text().to_string(),
                })
            }
        }
    }

    pub fn define_prim<P: Into<sdf::Path>>(&self, path: P, ty: &str) -> Result<Prim, Error> {
        let path = path.into();
        let token = tf::Token::new(ty);
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_DefinePrim(self.ptr, path.ptr, token.ptr, &mut ptr);

            let mut valid = false;
            ffi::usd_Prim_IsValid(ptr, &mut valid);

            if valid {
                Ok(Prim { ptr })
            } else {
                Err(Error::NoPrimAtPath {
                    path: path.text().to_string(),
                })
            }
        }
    }

    pub fn save(&self) {
        unsafe {
            ffi::usd_StageRefPtr_Save(self.ptr);
        }
    }

    pub fn export_to_string(&self) -> Option<cpp::String> {
        unsafe {
            let mut string = cpp::String::default();
            let mut result = false;
            ffi::usd_StageRefPtr_ExportToString(
                self.ptr,
                &mut string.ptr,
                false,
                &mut result
            );
            if result {
                Some(string)
            }
            else {
                None
            }
        }
    }

    pub fn export(&self, filename: &cpp::String) -> bool {
        unsafe {
            let mut result = false;
            let file_format_arguments = sdf::FileFormatArguments::default();
            ffi::usd_StageRefPtr_Export(
                self.ptr,
                filename.ptr,
                false,
                file_format_arguments.ptr,
                &mut result
            );
            result
        }
    }

    pub fn get_root_layer(&self) -> sdf::LayerHandle {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_GetRootLayer(self.ptr, &mut ptr);
            sdf::LayerHandle {
                ptr
            }
        }
    }

    pub fn get_session_layer(&self) -> sdf::LayerHandle {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_GetSessionLayer(self.ptr, &mut ptr);
            sdf::LayerHandle {
                ptr
            }
        }
    }

    pub fn get_edit_target_for_local_layer(&self, layer: &sdf::LayerHandle) -> EditTarget {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_GetEditTargetForLocalLayer(self.ptr, layer.ptr, &mut ptr);
            EditTarget {
                ptr
            }
        }
    }

    pub fn get_edit_target_for_local_layer_with_index(&self, i: usize) -> EditTarget {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_StageRefPtr_GetEditTargetForLocalLayer_with_index(self.ptr, i, &mut ptr);
            EditTarget {
                ptr
            }
        }
    }

    pub fn set_edit_target(&self, edit_target: &EditTarget) {
        unsafe {
            ffi::usd_StageRefPtr_SetEditTarget(self.ptr, edit_target.ptr);
        }
    }
}

impl Drop for StageRefPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_StageRefPtr_dtor(self.ptr);
        }
    }
}

unsafe impl Send for StageRefPtr {}
unsafe impl Sync for StageRefPtr {}

pub trait Object {
    fn _object_ptr(&self) -> *mut ffi::usd_Object_t;

    fn path(&self) -> sdf::Path {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Object_GetPath(self._object_ptr(), &mut ptr);
            sdf::Path { ptr }
        }
    }

    fn name(&self) -> tf::TokenRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::usd_Object_GetName(self._object_ptr(), &mut ptr);
            tf::TokenRef { ptr: ptr as _ }
        }
    }

    fn display_name(&self) -> String {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Object_GetDisplayName(self._object_ptr(), &mut ptr);
            let mut ptr_c_str = std::ptr::null();
            ffi::std_String_c_str(ptr, &mut ptr_c_str);
            let result = CStr::from_ptr(ptr_c_str).to_string_lossy().to_string();
            ffi::std_String_dtor(ptr);

            result
        }
    }
}

pub struct Prim {
    pub(crate) ptr: *mut ffi::usd_Prim_t,
}

impl Prim {
    pub fn type_name(&self) -> tf::TokenRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::usd_Prim_GetTypeName(self.ptr, &mut ptr);
            tf::TokenRef { ptr: ptr as _ }
        }
    }

    pub fn children(&self) -> PrimSiblingRange {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Prim_GetChildren(self.ptr, &mut ptr);
            PrimSiblingRange::_from_ptr(ptr)
        }
    }

    pub fn properties(&self) -> PropertyVector {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Prim_GetProperties(self.ptr, &mut ptr);
            PropertyVector { ptr }
        }
    }

    pub fn get_references(&self) -> References {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Prim_GetReferences(self.ptr, &mut ptr);
            References {
                ptr
            }
        }
    }

    pub fn ptr(&self) -> *const ffi::usd_Prim_t {
        self.ptr
    }
}

unsafe impl Send for Prim {}
unsafe impl Sync for Prim {}

impl Object for Prim {
    fn _object_ptr(&self) -> *mut ffi::usd_Object_t {
        self.ptr as *mut ffi::usd_Object_t
    }
}

impl Drop for Prim {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_Prim_dtor(self.ptr);
        }
    }
}

pub struct PrimRange {
    ptr: *mut ffi::usd_PrimRange_t,
    current: PrimRangeIterator,
    end: PrimRangeIterator,
}

impl PrimRange {
    pub(crate) fn _from_ptr(ptr: *mut ffi::usd_PrimRange_t) -> Self {
        unsafe {
            let mut current = std::ptr::null_mut();
            ffi::usd_PrimRange_begin(ptr, &mut current);

            let mut end = std::ptr::null_mut();
            ffi::usd_PrimRange_end(ptr, &mut end);

            PrimRange {
                ptr,
                current: PrimRangeIterator { ptr: current },
                end: PrimRangeIterator { ptr: end },
            }
        }
    }

    pub fn from_prim(prim: &Prim) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimRange_from_prim(prim.ptr, &mut ptr);
            PrimRange::_from_ptr(ptr)
        }
    }

    pub fn begin(&self) -> PrimRangeIterator {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimRange_begin(self.ptr, &mut ptr);
            PrimRangeIterator { ptr }
        }
    }

    pub fn end(&self) -> PrimRangeIterator {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimRange_end(self.ptr, &mut ptr);
            PrimRangeIterator { ptr }
        }
    }
}

impl Iterator for PrimRange {
    type Item = Prim;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            unsafe {
                let prim = self.current.deref();

                let mut dummy = std::ptr::null_mut();
                ffi::usd_PrimRangeIterator_op_inc(self.current.ptr, &mut dummy);

                Some(prim)
            }
        }
    }
}

impl Drop for PrimRange {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_PrimRange_dtor(self.ptr);
        }
    }
}

pub struct PrimRangeIterator {
    ptr: *mut ffi::usd_PrimRangeIterator_t,
}

impl PrimRangeIterator {
    fn deref(&self) -> Prim {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimRangeIterator_deref(self.ptr, &mut ptr);
            Prim { ptr }
        }
    }
}

impl Drop for PrimRangeIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_PrimRangeIterator_dtor(self.ptr);
        }
    }
}

impl PartialEq for PrimRangeIterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_PrimRangeIterator_op_eq(self.ptr, other.ptr, &mut result);
            result
        }
    }
}

pub struct PrimSiblingRange {
    ptr: *mut ffi::usd_PrimSiblingRange_t,
    current: PrimSiblingIterator,
    end: PrimSiblingIterator,
}

impl PrimSiblingRange {
    pub(crate) fn _from_ptr(ptr: *mut ffi::usd_PrimSiblingRange_t) -> Self {
        unsafe {
            let mut current = std::ptr::null_mut();
            ffi::usd_PrimSiblingRange_begin(ptr, &mut current);

            let mut end = std::ptr::null_mut();
            ffi::usd_PrimSiblingRange_end(ptr, &mut end);

            PrimSiblingRange {
                ptr,
                current: PrimSiblingIterator { ptr: current },
                end: PrimSiblingIterator { ptr: end },
            }
        }
    }

    pub fn begin(&self) -> PrimSiblingIterator {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimSiblingRange_begin(self.ptr, &mut ptr);
            PrimSiblingIterator { ptr }
        }
    }

    pub fn end(&self) -> PrimSiblingIterator {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimSiblingRange_end(self.ptr, &mut ptr);
            PrimSiblingIterator { ptr }
        }
    }
}

impl Iterator for PrimSiblingRange {
    type Item = Prim;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            unsafe {
                let prim = self.current.deref();

                let mut dummy = std::ptr::null_mut();
                ffi::usd_PrimSiblingIterator_op_inc(self.current.ptr, &mut dummy);

                Some(prim)
            }
        }
    }
}

impl Drop for PrimSiblingRange {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_PrimSiblingRange_dtor(self.ptr);
        }
    }
}

pub struct PrimSiblingIterator {
    ptr: *mut ffi::usd_PrimSiblingIterator_t,
}

impl PrimSiblingIterator {
    fn deref(&self) -> Prim {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_PrimSiblingIterator_deref(self.ptr, &mut ptr);
            Prim { ptr }
        }
    }
}

impl Drop for PrimSiblingIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_PrimSiblingIterator_dtor(self.ptr);
        }
    }
}

impl PartialEq for PrimSiblingIterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_PrimSiblingIterator_op_eq(self.ptr, other.ptr, &mut result);
            result
        }
    }
}

pub trait PropertyEx {
    fn _property_ptr(&self) -> *mut ffi::usd_Property_t;

    fn base_name(&self) -> tf::Token {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Property_GetBaseName(self._property_ptr(), &mut ptr);
            tf::Token { ptr }
        }
    }

    fn namespace(&self) -> tf::Token {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Property_GetNamespace(self._property_ptr(), &mut ptr);
            tf::Token { ptr }
        }
    }

    fn is_authored(&self) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_Property_IsAuthored(self._property_ptr(), &mut result);
            result
        }
    }

    fn split_name(&self) -> Vec<String> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Property_SplitName(self._property_ptr(), &mut ptr);
            let mut size = 0;
            ffi::std_StringVector_size(ptr, &mut size);
            let mut result: Vec<String> = Vec::new();
            for i in 0..size {
                let mut ptr_str = std::ptr::null();
                ffi::std_StringVector_op_index(ptr, i, &mut ptr_str);
                let mut ptr_c_str = std::ptr::null();
                ffi::std_String_c_str(ptr_str, &mut ptr_c_str);
                result.push(CStr::from_ptr(ptr_c_str).to_string_lossy().to_string());
            }

            ffi::std_StringVector_dtor(ptr);

            result
        }
    }

    fn display_group(&self) -> String {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Property_GetDisplayGroup(self._property_ptr(), &mut ptr);
            let mut ptr_c_str = std::ptr::null();
            ffi::std_String_c_str(ptr, &mut ptr_c_str);
            let result = CStr::from_ptr(ptr_c_str).to_string_lossy().to_string();
            ffi::std_String_dtor(ptr);

            result
        }
    }
}

pub struct Property {
    pub(crate) ptr: *mut ffi::usd_Property_t,
}

impl Property {
    pub fn property_kind(&self) -> PropertyKind {
        unsafe {
            let mut is_attribute = false;
            let mut is_relationship = false;

            ffi::usd_Property_Is_Attribute(self.ptr, &mut is_attribute);
            ffi::usd_Property_Is_Relationship(self.ptr, &mut is_relationship);

            if is_attribute {
                let mut ptr = std::ptr::null_mut();
                ffi::usd_Property_As_Attribute(self.ptr, &mut ptr);
                PropertyKind::Attribute(Attribute { ptr })
            } else if is_relationship {
                let mut ptr = std::ptr::null_mut();
                ffi::usd_Property_As_Relationship(self.ptr, &mut ptr);
                PropertyKind::Relationship(Relationship { ptr })
            } else {
                unreachable!();
            }
        }
    }
}

impl Object for Property {
    fn _object_ptr(&self) -> *mut ffi::usd_Object_t {
        self.ptr as *mut ffi::usd_Object_t
    }
}

impl PropertyEx for Property {
    fn _property_ptr(&self) -> *mut ffi::usd_Property_t {
        self.ptr
    }
}

impl Drop for Property {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_Property_dtor(self.ptr);
        }
    }
}

pub struct PropertyRef {
    pub(crate) ptr: *mut ffi::usd_Property_t,
}

impl std::ops::Deref for PropertyRef {
    type Target = Property;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const PropertyRef as *const Property) }
    }
}

pub struct PropertyVector {
    pub(crate) ptr: *mut ffi::usd_PropertyVector_t,
}

impl PropertyVector {
    pub fn size(&self) -> usize {
        unsafe {
            let mut size = 0;
            ffi::usd_PropertyVector_size(self.ptr, &mut size);
            size
        }
    }

    pub fn at(&self, index: usize) -> PropertyRef {
        unsafe {
            let mut ptr = std::ptr::null();
            ffi::usd_PropertyVector_op_index(self.ptr, index, &mut ptr);
            PropertyRef { ptr: ptr as _ }
        }
    }

    pub fn iter(&self) -> PropertyVectorIterator {
        PropertyVectorIterator {
            vec: self,
            current: 0,
            end: self.size(),
        }
    }
}

impl Drop for PropertyVector {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_PropertyVector_dtor(self.ptr);
        }
    }
}

pub struct PropertyVectorIterator<'a> {
    vec: &'a PropertyVector,
    current: usize,
    end: usize,
}

impl<'a> Iterator for PropertyVectorIterator<'a> {
    type Item = PropertyRef;

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

impl<'a> IntoIterator for &'a PropertyVector {
    type Item = PropertyRef;
    type IntoIter = PropertyVectorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Attribute {
    ptr: *mut ffi::usd_Attribute_t,
}

impl Attribute {
    pub fn get(&self) -> Option<vt::Value> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_new(&mut ptr);
            let mut result = false;
            ffi::usd_Attribute_Get(self.ptr, ptr, TimeCode::default().0, &mut result);
            if result {
                Some(vt::Value { ptr })
            } else {
                None
            }
        }
    }

    pub fn get_at(&self, time: TimeCode) -> Option<vt::Value> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_new(&mut ptr);
            let mut result = false;
            ffi::usd_Attribute_Get(self.ptr, ptr, time.0, &mut result);
            if result {
                Some(vt::Value { ptr })
            } else {
                None
            }
        }
    }

    pub fn type_name(&self) -> sdf::ValueTypeName {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_Attribute_GetTypeName(self.ptr, &mut ptr);
            sdf::ValueTypeName { ptr }
        }
    }

    pub fn set(&mut self, value: &vt::Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_Attribute_Set(self.ptr, value.ptr, TimeCode::default().0, &mut result);
            result
        }
    }

    pub fn set_at(&mut self, value: &vt::Value, time: TimeCode) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_Attribute_Set(self.ptr, value.ptr, time.0, &mut result);
            result
        }
    }
}

impl Object for Attribute {
    fn _object_ptr(&self) -> *mut ffi::usd_Object_t {
        self.ptr as *mut ffi::usd_Object_t
    }
}

impl PropertyEx for Attribute {
    fn _property_ptr(&self) -> *mut ffi::usd_Property_t {
        self.ptr as *mut ffi::usd_Property_t
    }
}

impl std::ops::Deref for Attribute {
    type Target = Property;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Attribute as *const Property) }
    }
}

impl Drop for Attribute {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_Attribute_dtor(self.ptr);
        }
    }
}

pub struct AttributeRef {
    pub(crate) ptr: *mut ffi::usd_Attribute_t,
}

impl std::ops::Deref for AttributeRef {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const AttributeRef as *const Attribute) }
    }
}

pub struct Relationship {
    ptr: *mut ffi::usd_Relationship_t,
}

impl Relationship {
    pub fn targets(&self) -> Option<sdf::PathVector> {
        unsafe {
            let targets = sdf::PathVector::default();
            let mut result = false;
            ffi::usd_Relationship_GetTargets(self.ptr, targets.ptr, &mut result);
            if result {
                Some(targets)
            } else {
                None
            }
        }
    }
}

impl Object for Relationship {
    fn _object_ptr(&self) -> *mut ffi::usd_Object_t {
        self.ptr as *mut ffi::usd_Object_t
    }
}

impl PropertyEx for Relationship {
    fn _property_ptr(&self) -> *mut ffi::usd_Property_t {
        self.ptr as *mut ffi::usd_Property_t
    }
}

impl std::ops::Deref for Relationship {
    type Target = Property;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Relationship as *const Property) }
    }
}

impl Drop for Relationship {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_Relationship_dtor(self.ptr);
        }
    }
}

pub struct RelationshipRef {
    pub(crate) ptr: *mut ffi::usd_Relationship_t,
}

impl std::ops::Deref for RelationshipRef {
    type Target = Relationship;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const RelationshipRef as *const Relationship) }
    }
}

pub enum PropertyKind {
    Attribute(Attribute),
    Relationship(Relationship),
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TimeCode(pub ffi::usd_TimeCode_t);

impl TimeCode {
    pub fn from_time(time: f64) -> Self {
        Self(ffi::usd_TimeCode_t {
            time
        })
    }
}

impl Default for TimeCode {
    fn default() -> Self {
        unsafe {
            let mut tc = ffi::usd_TimeCode_t { time: 0.0 };
            ffi::usd_TimeCode_Default(&mut tc);
            TimeCode(tc)
        }
    }
}

pub struct GLEngine {
    pub(crate) ptr: *mut ffi::usdImaging_GLEngine_t
}

impl GLEngine {
    pub fn new() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdImaging_GLEngine_new(&mut ptr);
            Self {
                ptr
            }
        }
    }

    pub fn set_renderer_aov(&self, aov: &tf::Token) -> bool {
        unsafe {
            let mut result = false;
            ffi::usdImaging_GLEngine_SetRendererAov(self.ptr, aov.ptr, &mut result);
            result
        }
    }

    pub fn set_camera_state(&self, view: glam::DMat4, projection: glam::DMat4) {
        unsafe {
            ffi::usdImaging_GLEngine_SetCameraState(self.ptr, &view as *const glam::DMat4 as *const _, &projection as *const glam::DMat4 as *const _);
        }
    }

    pub fn set_render_viewport(&self, viewport: glam::DVec4) {
        unsafe {
            ffi::usdImaging_GLEngine_SetRenderViewport(self.ptr, &viewport as *const glam::DVec4 as *const _);
        }
    }

    pub fn render(&self, prim: &Prim, params: &GLRenderParams) {
        unsafe {
            ffi::usdImaging_GLEngine_Render(self.ptr, prim.ptr, params.ptr);
        }
    }

    pub fn set_render_setting(&self, id: &tf::Token, value: &vt::Value) {
        unsafe {
            ffi::usdImaging_GLEngine_SetRendererSetting(self.ptr as _, id.ptr, value.ptr);
        }
    }
}

impl Drop for GLEngine {
    fn drop(&mut self) {
        unsafe {
            ffi::usdImaging_GLEngine_dtor(self.ptr);
        }
    }
}

pub struct GLRenderParams {
    pub(crate) ptr: *mut ffi::usdImaging_GLRenderParams_t
}

impl GLRenderParams {
    pub fn new() -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdImaging_GLRenderParams_new(&mut ptr);
            Self {
                ptr
            }
        }
    }

    pub fn set_enable_lighting(&self, enable_lighting: bool) {
        unsafe {
            ffi::usdImaging_GLRenderParams_SetEnableLighting(self.ptr, enable_lighting);
        }
    }

    pub fn set_cull_style(&self, cull_style: ffi::usdImaging_GLCullStyle) {
        unsafe {
            ffi::usdImaging_GLRenderParams_SetCullStyle(self.ptr, cull_style as _);
        }
    }
}

impl Drop for GLRenderParams {
    fn drop(&mut self) {
        unsafe {
            ffi::usdImaging_GLRenderParams_dtor(self.ptr);
        }
    }
}

pub struct Mesh {
    pub(crate) ptr: *mut ffi::usdGeom_Mesh_t,
}

impl Mesh {
    pub fn new(prim: &Prim) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Mesh_new(prim.ptr, &mut ptr);
            Self { ptr }
        }
    }

    pub fn add_xform_op(&self) {

    }
}

pub struct Primvar {
    pub(crate) ptr: *mut ffi::usdGeom_Primvar_t,
}

impl Primvar {
    pub fn new(attribute: &Attribute) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Primvar_new(attribute.ptr, &mut ptr);
            Self { ptr }
        }
    }

    pub fn set_interpolation(&mut self, interpolation: &str) -> bool {
        let interpolation = tf::Token::new(interpolation);
        unsafe {
            let mut result = false;
            ffi::usdGeom_Primvar_SetInterpolation(self.ptr, interpolation.ptr, &mut result);
            result
        }
    }
}

pub struct XformCommonAPI {
    pub(crate) ptr: *mut ffi::usdGeom_XformCommonAPI_t
}

impl XformCommonAPI {
    pub fn new(prim: &Prim) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_XformCommonAPI_new(prim.ptr, &mut ptr);
            Self {
                ptr
            }
        }
    }

    pub fn set_translation(&self, position: glam::DVec3, timecode: TimeCode) -> bool {
        unsafe {
            let mut result = false;
            ffi::usdGeom_XformCommonAPI_SetTranslate(self.ptr, (&position) as *const glam::DVec3 as _, timecode.0, &mut result);
            result
        }
    }

    pub fn set_scale(&self, scale: glam::Vec3, timecode: TimeCode) -> bool {
        unsafe {
            let mut result = false;
            ffi::usdGeom_XformCommonAPI_SetScale(self.ptr, (&scale) as *const glam::Vec3 as _, timecode.0, &mut result);
            result
        }
    }

    pub fn set_rotate(&self, rot: glam::Vec3, timecode: TimeCode) -> bool {
        unsafe {
            let mut result = false;
            ffi::usdGeom_XformCommonAPI_SetRotate(self.ptr, (&rot) as *const glam::Vec3 as _, ffi::usdGeom_XformCommonAPIRotationOrder_usdGeom_XformCommonAPIRotationOrder_RotationOrderXYZ, timecode.0, &mut result);
            result
        }
    }
}

unsafe impl Send for XformCommonAPI {}
unsafe impl Sync for XformCommonAPI {}

pub struct References {
    pub(crate) ptr: *mut ffi::usd_References_t
}

impl References {
    pub fn add_internal_reference(&mut self, path: &sdf::Path) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_References_AddInternalReference(self.ptr, path.ptr, sdf::LayerOffset::default().ptr, ffi::usd_ListPosition_usd_ListPosition_UsdListPositionBackOfAppendList, &mut result);
            result
        }
    }

    pub fn add_reference(&mut self, identifier: &cpp::String) -> bool {
        unsafe {
            let mut result = false;
            ffi::usd_References_AddReference_with_identifier(self.ptr, identifier.ptr, sdf::LayerOffset::default().ptr, ffi::usd_ListPosition_usd_ListPosition_UsdListPositionBackOfAppendList, &mut result);
            result
        }
    }
}

pub struct XformOp {
    pub(crate) ptr: *mut ffi::usdGeom_XformOp_t
}

impl XformOp {
    pub fn set(&self, value: &vt::Value, timecode: TimeCode) -> bool {
        unsafe {
            let mut result = false;
            ffi::usdGeom_XformOp_Set(self.ptr, value.ptr, timecode.0, &mut result);
            result
        }
    }
}

pub struct Xformable {
    pub(crate) ptr: *mut ffi::usdGeom_Xformable_t
}

impl Xformable {
    pub fn new(prim: &Prim) -> Self {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usdGeom_Xformable_new(prim.ptr, &mut ptr);
            Self {
                ptr
            }
        }
    }

    /*pub fn get_xform_op(&self, ty: ffi::usdGeom_XformOpType) -> XformOp {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let result = ffi::usdGeom_Xformable_GetXformOp(
                self.ptr,
                ty,
                tf::Token::new("").ptr,
                false,
                &mut ptr,
            );
            XformOp {
                ptr
            }

        }
    }*/

    pub fn add_xform_op(&self, ty: ffi::usdGeom_XformOpType) -> XformOp {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let result = ffi::usdGeom_Xformable_AddXformOp(
                self.ptr,
                ty,
                ffi::usdGeom_XformOpPrecision_usdGeom_XformOpPrecision_PrecisionDouble,
                tf::Token::new("").ptr,
                false,
                &mut ptr,
            );
            XformOp {
                ptr
            }

        }
    }
}

pub struct EditTarget {
    pub(crate) ptr: *mut ffi::usd_EditTarget_t
}

impl EditTarget {
    pub fn new_from_layer_ref_ptr(layer: &sdf::LayerRefPtr) -> Self {
        let layer_offset = sdf::LayerOffset::default();
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::usd_EditTarget_from_layer_ref_ptr(layer.ptr, layer_offset.ptr, &mut ptr);
            Self {
                ptr
            }
        }
    }
}

impl Drop for EditTarget {
    fn drop(&mut self) {
        unsafe {
            ffi::usd_EditTarget_dtor(self.ptr);
        }
    }
}
