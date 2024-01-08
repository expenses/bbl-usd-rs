use paste::paste;
use std::ops::IndexMut;

use crate::{ffi, sdf, tf};
use ffi::*;
use glam::{Vec2, Vec3, Vec4};

pub struct TokenArray {
    pub(crate) ptr: *mut ffi::vt_TokenArray_t,
}

impl TokenArray {
    pub fn len(&self) -> usize {
        unsafe {
            let mut result = 0;
            ffi::vt_TokenArray_size(self.ptr, &mut result);
            result
        }
    }

    pub fn at(&self, index: usize) -> tf::TokenRef {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ffi::vt_TokenArray_op_index(self.ptr, index, &mut ptr);
            tf::TokenRef { ptr }
        }
    }

    pub fn iter(&self) -> TokenArrayIterator {
        TokenArrayIterator {
            vec: self,
            current: 0,
            end: self.len(),
        }
    }
}

impl<'a> IntoIterator for &'a TokenArray {
    type Item = tf::TokenRef;
    type IntoIter = TokenArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct TokenArrayIterator<'a> {
    vec: &'a TokenArray,
    current: usize,
    end: usize,
}

impl<'a> Iterator for TokenArrayIterator<'a> {
    type Item = tf::TokenRef;

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

pub struct TokenArrayRef {
    pub(crate) ptr: *mut ffi::vt_TokenArray_t,
}

impl std::ops::Deref for TokenArrayRef {
    type Target = TokenArray;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const TokenArrayRef as *const TokenArray) }
    }
}

macro_rules! make_primitive_array {
    ($prefix:ident, $name:ident, $c_name:ident, $inner_type:ty) => {
        paste! {
            pub struct $name {
                pub(crate) ptr: *mut [<$prefix _ $c_name _t>]
            }

            impl std::ops::Deref for $name {
                type Target = [$inner_type];

                fn deref(&self) -> &Self::Target {
                    unsafe {
                        let mut size = 0;
                        [<$prefix _ $c_name _size>](self.ptr, &mut size);

                        let mut ptr = std::ptr::null();
                        [<$prefix _ $c_name _data_const>](self.ptr, &mut ptr);

                        std::slice::from_raw_parts(ptr as *const $inner_type, size)
                    }
                }
            }

            pub struct [<$name Ref>] {
                pub(crate) ptr: *mut [<$prefix _ $c_name _t>]
            }

            impl std::ops::Deref for [<$name Ref>] {
                type Target = $name;

                fn deref(&self) -> &Self::Target {
                    unsafe { &*(self as *const [<$name Ref>] as *const $name) }
                }
            }
        }
    };
    ($prefix:ident, $name:ident, $inner_type:ty) => {
        make_primitive_array! {$prefix, $name, $name, $inner_type}
    };
}

make_primitive_array!(vt, IntArray, i32);
make_primitive_array!(vt, FloatArray, f32);
make_primitive_array!(vt, DoubleArray, f64);
make_primitive_array!(gf, Vec2Array, Vec2fArray, Vec2);
make_primitive_array!(gf, Vec3Array, Vec3fArray, Vec3);
make_primitive_array!(gf, Vec4Array, Vec4fArray, Vec4);

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

    pub fn as_token_array(&self) -> Option<TokenArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtTokenArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtTokenArray(self.ptr, &mut ptr);
                Some(TokenArrayRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_int_array(&self) -> Option<IntArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtIntArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtIntArray(self.ptr, &mut ptr);
                Some(IntArrayRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_float_array(&self) -> Option<FloatArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtFloatArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtFloatArray(self.ptr, &mut ptr);
                Some(FloatArrayRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_double_array(&self) -> Option<DoubleArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtDoubleArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtDoubleArray(self.ptr, &mut ptr);
                Some(DoubleArrayRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_vec2_array(&self) -> Option<Vec2ArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec2fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec2fArray(self.ptr, &mut ptr);
                Some(Vec2ArrayRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_vec3_array(&self) -> Option<Vec3ArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec3fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec3fArray(self.ptr, &mut ptr);
                Some(Vec3ArrayRef { ptr: ptr as _ })
            } else {
                None
            }
        }
    }

    pub fn as_vec4_array(&self) -> Option<Vec4ArrayRef> {
        unsafe {
            let mut is_holding = false;
            ffi::vt_Value_IsHolding_VtVec4fArray(self.ptr, &mut is_holding);
            if is_holding {
                let mut ptr = std::ptr::null();
                ffi::vt_Value_Get_VtVec4fArray(self.ptr, &mut ptr);
                Some(Vec4ArrayRef { ptr: ptr as _ })
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

macro_rules! impl_value_member {
    ($type:ty, $c_type:ident) => {
        paste! {
            impl ValueMember for $type {
                fn get(value: &Value) -> Option<&Self> {
                    if Self::is_holding(value) {
                        unsafe {
                            let mut ptr = std::ptr::null();
                            [<vt_Value_Get_ $c_type>](value.ptr, &mut ptr);
                            Some(&*ptr)
                        }
                    } else {
                        None
                    }
                }

                fn is_holding(value: &Value) -> bool {
                    unsafe {
                        let mut result = false;
                        [<vt_Value_IsHolding_ $c_type>](value.ptr, &mut result);
                        result
                    }
                }

                fn from(member: &Self) -> Value {
                    unsafe {
                        let mut ptr = std::ptr::null_mut();
                        [<vt_Value_from_ $c_type>](*member, &mut ptr);
                        Value { ptr }
                    }
                }
            }
        }
    };
}

impl_value_member!(i32, int);
impl_value_member!(f32, float);
impl_value_member!(f64, double);
impl_value_member!(bool, bool);

macro_rules! impl_value_member_for_vec {
    ($type:ty) => {
        paste! {
            impl ValueMember for $type {
                fn get(value: &Value) -> Option<&Self> {
                    if Self::is_holding(value) {
                        unsafe {
                            let mut ptr = std::ptr::null();
                            [<vt_Value_Get_Gf $type f>](value.ptr, &mut ptr);
                            Some(&*(ptr as *mut $type))
                        }
                    } else {
                        None
                    }
                }

                fn is_holding(value: &Value) -> bool {
                    unsafe {
                        let mut result = false;
                        [<vt_Value_IsHolding_Gf $type f>](value.ptr, &mut result);
                        result
                    }
                }

                fn from(member: &Self) -> Value {
                    unsafe {
                        let mut ptr = std::ptr::null_mut();
                        [<vt_Value_from_Gf $type f>](
                            *(member as *const $type as *const [<gf_ $type f_t>]),
                            &mut ptr);
                        Value { ptr }
                    }
                }
            }
        }
    };
}

impl_value_member_for_vec!(Vec2);
impl_value_member_for_vec!(Vec3);
impl_value_member_for_vec!(Vec4);
