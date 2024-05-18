use crate::allocate_array;

use super::{NetObject, SystemObject, SystemObjectBindings, TypeInfoProvider};

pub struct SystemArray<Element> {
    length: usize,
    first_element: Element
}

impl<Element> SystemObjectBindings for NetObject<SystemArray<Element>> {}

impl<Content : TypeInfoProvider> TypeInfoProvider for SystemArray<Content> {
    fn type_of() -> *mut NetObject<super::system_reflection::SystemType> {
        Content::type_of().make_array_type()
    }
}

impl<Element> NetObject<SystemArray<Element>> {
    pub fn as_slice(self: *mut Self) -> &'static mut [Element] {
        unsafe {
            core::slice::from_raw_parts_mut(core::ptr::from_mut(&mut (*self).content.first_element), (*self).content.length)
        }
    }
}

impl<Element : TypeInfoProvider + Copy + 'static> NetObject<SystemArray<Element>> {
    pub fn from_slice(sliced: &[Element]) -> *mut Self {
        let arr = allocate_array(Element::type_of(), sliced.len() as i32);
        arr.as_slice().copy_from_slice(sliced);
        arr
    }
}

impl SystemArray<*mut NetObject<SystemObject>> {
    pub fn create_object_array(array: &[*mut NetObject<SystemObject>]) -> *mut NetObject<Self> {
        let arr = allocate_array(SystemObject::type_of(), array.len() as i32);
        let slice = arr.as_slice();
        for (index, val) in array.iter().enumerate() {
            slice[index] = *val;
        }
        arr
    }
}