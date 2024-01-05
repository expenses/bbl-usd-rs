use std::ops::IndexMut;

use crate::{ffi, sdf, tf};
use glam::{Vec2, Vec3, Vec4};

pub struct ArrayRef<T>(T);

impl<T> std::ops::Deref for ArrayRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! create_array_type {
    ($name:ident, $ptr_type:ty, $inner_ty:ty, $new_fn:expr, $size_fn:expr, $mut_ptr_fn:expr, $resize_fn:expr) => {
        pub struct $name {
            pub(crate) ptr: *mut $ptr_type,
        }

        impl $name {
            pub fn new() -> Self {
                unsafe {
                    let mut ptr = std::ptr::null_mut();
                    $new_fn(&mut ptr);
                    Self { ptr }
                }
            }

            pub fn resize(&mut self, num: usize) {
                unsafe {
                    $resize_fn(self.ptr, num);
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = [$inner_ty];

            fn deref(&self) -> &Self::Target {
                unsafe {
                    let mut size = 0;
                    $size_fn(self.ptr, &mut size);
                    let mut ptr = std::ptr::null_mut();
                    $mut_ptr_fn(self.ptr, &mut ptr);
                    std::slice::from_raw_parts(ptr as *const $inner_ty, size)
                }
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe {
                    let mut size = 0;
                    $size_fn(self.ptr, &mut size);
                    let mut ptr = std::ptr::null_mut();
                    $mut_ptr_fn(self.ptr, &mut ptr);
                    std::slice::from_raw_parts_mut(ptr as *mut $inner_ty, size)
                }
            }
        }
    };
}

create_array_type! {Vec2Array, ffi::gf_Vec2fArray_t, Vec2, ffi::gf_Vec2fArray_new, ffi::gf_Vec2fArray_size, ffi::gf_Vec2fArray_data, ffi::gf_Vec2fArray_resize}
create_array_type! {Vec3Array, ffi::gf_Vec3fArray_t, Vec3, ffi::gf_Vec3fArray_new, ffi::gf_Vec3fArray_size, ffi::gf_Vec3fArray_data, ffi::gf_Vec3fArray_resize}
create_array_type! {Vec4Array, ffi::gf_Vec4fArray_t, Vec4, ffi::gf_Vec4fArray_new, ffi::gf_Vec4fArray_size, ffi::gf_Vec4fArray_data, ffi::gf_Vec4fArray_resize}
create_array_type! {IntArray, ffi::vt_IntArray_t, i32, ffi::vt_IntArray_new, ffi::vt_IntArray_size, ffi::vt_IntArray_data, ffi::vt_IntArray_resize}
create_array_type! {FloatArray, ffi::vt_FloatArray_t, f32, ffi::vt_FloatArray_new, ffi::vt_FloatArray_size, ffi::vt_FloatArray_data, ffi::vt_FloatArray_resize}
create_array_type! {DoubleArray, ffi::vt_DoubleArray_t, f64, ffi::vt_DoubleArray_new, ffi::vt_DoubleArray_size, ffi::vt_DoubleArray_data, ffi::vt_DoubleArray_resize}
create_array_type! {TokenArray, ffi::vt_TokenArray_t, tf::TokenRef, ffi::vt_TokenArray_new, ffi::vt_TokenArray_size, ffi::vt_TokenArray_data, ffi::vt_TokenArray_resize}

pub struct Value {
    pub(crate) ptr: *mut ffi::vt_Value_t,
}

impl Value {
    pub fn get<T: ValueMember>(&self) -> Option<&T> {
        T::get(self)
    }
}

impl Value {
    pub fn as_token(&self) -> Option<tf::TokenRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_TfToken(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_TfToken(self.ptr, &mut ptr);
                Some(tf::TokenRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_token_array(&self) -> Option<ArrayRef<TokenArray>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtTokenArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtTokenArray(self.ptr, &mut ptr);
                Some(ArrayRef(TokenArray { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_int_array(&self) -> Option<ArrayRef<IntArray>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtIntArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtIntArray(self.ptr, &mut ptr);
                Some(ArrayRef(IntArray { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_float_array(&self) -> Option<ArrayRef<FloatArray>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtFloatArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtFloatArray(self.ptr, &mut ptr);
                Some(ArrayRef(FloatArray { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_double_array(&self) -> Option<ArrayRef<DoubleArray>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtDoubleArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtDoubleArray(self.ptr, &mut ptr);
                Some(ArrayRef(DoubleArray { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_vec2_array(&self) -> Option<ArrayRef<Vec2Array>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec2fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec2fArray(self.ptr, &mut ptr);
                Some(ArrayRef(Vec2Array { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_vec3_array(&self) -> Option<ArrayRef<Vec3Array>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec3fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec3fArray(self.ptr, &mut ptr);
                Some(ArrayRef(Vec3Array { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_vec4_array(&self) -> Option<ArrayRef<Vec4Array>> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec4fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec4fArray(self.ptr, &mut ptr);
                Some(ArrayRef(Vec4Array { ptr: ptr as _ }))
            } else {
                None
            }
        }
    }

    pub fn as_asset_path(&self) -> Option<sdf::AssetPathRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_SdfAssetPath(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_SdfAssetPath(self.ptr, &mut ptr);
                Some(sdf::AssetPathRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn from_int_array(member: &IntArray) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_VtIntArray(member.ptr, &mut ptr);
            Value { ptr }
        }
    }

    pub fn from_vec3_array(member: &Vec3Array) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_VtVec3fArray(member.ptr, &mut ptr);
            Value { ptr }
        }
    }
}

pub struct ValueRef {
    pub(crate) ptr: *mut ffi::vt_Value_t,
}

impl std::ops::Deref for ValueRef {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ValueRef as *const Value) }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            ffi::vt_Value_dtor(self.ptr);
        }
    }
}

pub trait ValueMember {
    fn get(value: &Value) -> Option<&Self>;
    fn is_holding(value: &Value) -> bool;
    fn from(member: &Self) -> Value;
}

impl ValueMember for i32 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_int(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_int(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_int(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for f32 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_float(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_float(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_float(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for f64 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_double(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_double(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_double(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for bool {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_bool(value.ptr, &mut ptr);
                Some(&*ptr)
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_bool(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_bool(*member, &mut ptr);
            Value { ptr }
        }
    }
}

impl ValueMember for Vec2 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec2f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec2))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec2f(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec2f(
                *(member as *const Vec2 as *const ffi::gf_Vec2f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Vec3 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec3f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec3))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec3f(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec3f(
                *(member as *const Vec3 as *const ffi::gf_Vec3f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}

impl ValueMember for Vec4 {
    fn get(value: &Value) -> Option<&Self> {
        if Self::is_holding(value) {
            unsafe {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_GfVec4f(value.ptr, &mut ptr);
                Some(&*(ptr as *mut Vec4))
            }
        } else {
            None
        }
    }

    fn is_holding(value: &Value) -> bool {
        unsafe {
            let mut result = false;
            ffi::vt_Value_IsHolding_GfVec4f(value.ptr, &mut result);
            result
        }
    }

    fn from(member: &Self) -> Value {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_Value_from_GfVec4f(
                *(member as *const Vec4 as *const ffi::gf_Vec4f_t),
                &mut ptr,
            );
            Value { ptr }
        }
    }
}
