use crate::imports::DotnetImportsContainer;

use super::{system_string::SystemString, MethodTable, NetObject, SystemObject, SystemObjectBindings};

#[repr(C)]
pub struct SystemType {
    keep_alive: *mut NetObject<SystemObject>,
    m_cache:  *mut NetObject<SystemObject>,
    pub handle: *mut MethodTable
}

impl SystemObjectBindings for NetObject<SystemType> {}

impl NetObject<SystemType> {
    pub fn allocate<T>(self: *mut Self, ctx: &DotnetImportsContainer) -> *mut NetObject<T> {
        ctx.allocate(self)
    }

    pub fn get_name(self: *mut Self) -> *mut NetObject<SystemString> {
        unsafe {
            let ptr: extern "stdcall" fn(*mut NetObject<SystemType>) -> *mut NetObject<SystemString> 
                = core::mem::transmute(self.get_method_table().get_func_at(0, 6));
            
            ptr(self)
        }
    }

    pub fn get_namespace(self: *mut Self) -> *mut NetObject<SystemString> {
        unsafe {
            let ptr: extern "stdcall" fn(*mut NetObject<SystemType>) -> *mut NetObject<SystemString> 
                = core::mem::transmute(self.get_method_table().get_func_at(2, 2));
            
            ptr(self)
        }
    }

    pub fn get_fullname(self: *mut Self) -> *mut NetObject<SystemString> {
        unsafe {
            let ptr: extern "stdcall" fn(*mut NetObject<SystemType>) -> *mut NetObject<SystemString> 
                = core::mem::transmute(self.get_method_table().get_func_at(2, 4));
            
            ptr(self)
        }
    }

    pub fn get_basetype(self: *mut Self) -> *mut NetObject<SystemType> {
        unsafe {
            let ptr: extern "stdcall" fn(*mut NetObject<SystemType>) -> *mut NetObject<SystemType> 
                = core::mem::transmute(self.get_method_table().get_func_at(11, 4));
            
            ptr(self)
        }
    }
}