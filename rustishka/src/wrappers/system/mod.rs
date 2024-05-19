pub mod system_console;
pub mod system_activator;
pub mod primitives;
pub mod system_delegate;
pub mod system_array;
pub mod system_appdomain;
use crate::{define_typeof, define_virtual, DOTNET_RUNTIME};

use self::{system_reflection::SystemType, system_string::SystemString};

pub mod system_string;
pub mod system_reflection;

#[repr(C)]
pub struct NetObject<Content> {
    pub method_table: *mut MethodTable,
    pub content: Content
}

impl<Content> NetObject<Content> {
    pub fn get_method_table(self: *mut Self) -> *mut MethodTable {
        unsafe {
            (*self).method_table
        }
    }

    pub fn get_content(self: *mut Self) -> &'static mut Content {
        unsafe {
            &mut (*self).content
        }
    }

    pub fn cast_unchecked<T>(self: *mut Self) -> *mut NetObject<T> {
        unsafe {
            core::mem::transmute(self)
        }
    }

    pub fn get_type(self: *mut Self) -> *mut NetObject<SystemType> {
        unsafe {
            DOTNET_RUNTIME.get_mut().unwrap().get_type(self)
        }
    }
}

impl<Content: TypeInfoProvider> NetObject<Content> {
    pub fn box_value(value: Content) -> *mut Self {
        unsafe {
            let val = DOTNET_RUNTIME.get_mut().unwrap().allocate(Content::type_of());
            (*val).content = value;
            val
        }
    }
}

pub trait TypeInfoProvider {
    fn type_of() -> *mut NetObject<SystemType>;
}

impl<Content : TypeInfoProvider> TypeInfoProvider for NetObject<Content> {
    fn type_of() -> *mut NetObject<SystemType> {
        Content::type_of()
    }
}

impl<Content : TypeInfoProvider> TypeInfoProvider for *mut NetObject<Content> {
    fn type_of() -> *mut NetObject<SystemType> {
        Content::type_of()
    }
}

pub struct SystemObject { }
define_typeof!(SystemObject, "System.Object");

pub trait SystemObjectBindings {
    define_virtual!(to_system_string, 0, 1, *mut NetObject<SystemString>);
    define_virtual!(equals, 0, 2, bool, other: *mut NetObject<SystemObject>);
    define_virtual!(get_hashcode, 0, 3, i32);
}

impl SystemObjectBindings for NetObject<SystemObject> {}

#[repr(C)]
pub struct MethodTable {
    some_shit: u32, // 00
    pub base_size: u32, // 04
    bruh_flags: u16, // 08
    pub metadata_token: u16, // 10
    pub num_virtuals: u16, // 12
    pub num_interfaces: u16, // 14
    pub parent: *mut MethodTable, // 16
    loader_module: usize, // 24
    pub writeable_data: *mut u8, // 32
    pub ee_class: *mut usize, // 40
    per_inst_info: *mut usize, // 48
    pub interface_map: *mut MethodTable, // 56
    pub methods: *mut *mut usize // 64
}

impl MethodTable {
    pub fn get_func_at(self: *mut Self, row: usize, index: usize) -> *mut usize {
        unsafe { 
            // very unsafe
            // inlines and compiles as: (ex. for x86-64)
            // mov rax, qword [rcx] // deref object pointer to 
            // mov rax, qword [rax + 0x40 + row * 8] // get methods chunk. [precompued row * i & inlined as number]
            // call qword ptr [rax + index * 8] // [precompued index * i & inlined as number]
            let chunk = (core::ptr::from_mut(&mut (*self).methods)).byte_add(row * core::mem::size_of::<usize>()).read();
            let entry = chunk.byte_add(index * core::mem::size_of::<usize>()).read();
            entry
        }
    }
}