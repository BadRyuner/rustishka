use std::ptr;

use super::{NetObject, SystemObjectBindings};

#[repr(C)]
pub struct SystemString {
    pub length: u32,
    pub chars: [u16; 512]
}

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
}