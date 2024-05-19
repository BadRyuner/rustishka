use std::ptr;

use crate::{define_typeof, imports::DotnetImportsContainer};

use super::{NetObject, SystemObjectBindings};

#[repr(C)]
pub struct SystemString {
    pub length: u32,
    pub chars: [u16; 512]
}

define_typeof!(SystemString, "System.String");

impl SystemObjectBindings for NetObject<SystemString> {}

impl SystemString {
    pub unsafe fn alloc_on_rust(text: &str) -> Box<NetObject<SystemString>> {
        let mut pseudo_string = NetObject::<SystemString> { 
            method_table: ptr::null_mut(), // VERY FUCKING UNSAFE
            content: SystemString {
                length: text.chars().count() as u32,
                chars: [0; 512]
            }
        };

        for (i, c) in text.chars().enumerate() {
            if i > 511 {
                break;
            }
            pseudo_string.content.chars[i] = c as u16;
        }

        Box::new(pseudo_string)
    }

    pub fn alloc_on_dotnet(ctx: &DotnetImportsContainer, text: &String) -> *mut NetObject<SystemString> {
        ctx.allocate_string(text)
    }
}

impl NetObject<SystemString> {
    pub fn as_slice(self: *mut Self) -> &'static mut [u16] {
        unsafe {
            core::slice::from_raw_parts_mut(core::ptr::from_mut(&mut (*self).content.chars[0]), (*self).content.length as usize)
        }
    }
}