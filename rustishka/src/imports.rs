use std::collections::HashMap;

use crate::wrappers::system::{system_array::SystemArray, system_exception::Exception, system_reflection::SystemType, system_string::SystemString, NetObject};

#[repr(C)]
pub struct DotnetImports {
    pub get_type: extern "stdcall" fn(*mut usize) -> *mut NetObject<SystemType>,
    pub search_type: extern "stdcall" fn(*mut NetObject<SystemString>, bool) -> *mut NetObject<SystemType>,
    pub get_fn_at_slot: extern "stdcall" fn (*mut NetObject<SystemType>, i32) -> *mut usize,
    pub allocate: extern "stdcall" fn(*mut NetObject<SystemType>) -> *mut usize,
    pub allocate_string: extern "stdcall" fn(*const u8, i32) -> *mut NetObject<SystemString>,
    pub allocate_array: extern "stdcall" fn(*mut NetObject<SystemType>, i32) -> *mut NetObject<SystemArray<()>>,
    pub try_catch: extern "stdcall" fn(extern "stdcall" fn()) -> *mut NetObject<Exception>,
    pub throw: extern "stdcall" fn(*mut NetObject<Exception>)
}

#[derive(Debug)]
pub struct DotnetImportsContainer(*const DotnetImports, HashMap<String, *mut NetObject<SystemType>>);

unsafe impl Send for DotnetImportsContainer {}
unsafe impl Sync for DotnetImportsContainer {}

impl DotnetImportsContainer {
    pub fn new(imports: *const DotnetImports) -> Self {
        Self {
            0: imports,
            1: HashMap::new()
        }
    }

    pub fn default() -> Self {
        Self {
            0: std::ptr::null_mut(),
            1: HashMap::new()
        }
    }

    pub fn get_type<T>(&self, object: *mut NetObject<T>) -> *mut NetObject<SystemType> {
        unsafe {
            ((*self.0).get_type)(core::mem::transmute::<*mut NetObject<T>, *mut usize>(object))
        }
    }

    pub fn search_type(&self, type_name: *mut NetObject<SystemString>, throw_on_error: bool) -> *mut NetObject<SystemType> {
        unsafe {
            ((*self.0).search_type)(type_name, throw_on_error)
        }
    }

    pub fn search_type_cached(&mut self, type_name: &String, throw_on_error: bool) -> *mut NetObject<SystemType> {
        unsafe {
            match self.1.get(type_name) {
                Some(t) => *t,
                None => {
                    let mut converted = SystemString::alloc_on_rust(type_name);
                    let result = self.search_type(converted.as_mut(), throw_on_error);
                    self.1.insert(type_name.to_string(), result);
                    result
                },
            }
        }
    }

    pub fn get_fn_at_slot(&self, tape: *mut NetObject<SystemType>, slot: i32) -> *mut usize {
        unsafe {
            ((*self.0).get_fn_at_slot)(tape, slot)
        }
    }

    pub fn allocate<T>(&self, object_type: *mut NetObject<SystemType>) -> *mut NetObject<T> {
        unsafe {
            core::mem::transmute(((*self.0).allocate)(object_type))
        }
    }

    pub fn allocate_string(&self, text: &String) -> *mut NetObject<SystemString> {
        unsafe {
            core::mem::transmute(((*self.0).allocate_string)(text.as_ptr(), text.len() as i32))
        }
    }

    pub fn allocate_array<T>(&self, element_type: *mut NetObject<SystemType>, size: i32) -> *mut NetObject<SystemArray<T>> {
        unsafe {
            core::mem::transmute(((*self.0).allocate_array)(element_type, size))
        }
    }

    pub fn throw(&self, exception: *mut NetObject<Exception>) {
        unsafe {
            ((*self.0).throw)(exception)
        }
    }
}