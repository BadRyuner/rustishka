use super::{NetObject, SystemObjectBindings};

pub struct SystemArray<Element> {
    length: usize,
    first_element: Element
}

impl<Element> SystemObjectBindings for NetObject<SystemArray<Element>> {}

impl<Element> NetObject<SystemArray<Element>> {
    pub fn as_slice(self: *mut Self) -> &'static mut [Element] {
        unsafe {
            core::slice::from_raw_parts_mut(core::ptr::from_mut(&mut (*self).content.first_element), (*self).content.length)
        }
    }
}