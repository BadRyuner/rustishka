#![cfg(any(windows, doc))]
#![feature(arbitrary_self_types)]
#![allow(dead_code)]

use std::sync::OnceLock;

use imports::DotnetImportsContainer;
use wrappers::system::{system_array::SystemArray, system_reflection::SystemType, system_string::SystemString, NetObject};

pub mod wrappers;
pub mod imports;

pub static mut DOTNET_RUNTIME: OnceLock<DotnetImportsContainer> = OnceLock::new();

pub fn get_type<T>(object: *mut NetObject<T>) -> *mut NetObject<SystemType> {
    unsafe {
        DOTNET_RUNTIME.get_mut().unwrap().get_type(object)
    }
}

pub fn search_type(type_name: *mut NetObject<SystemString>, throw_on_error: bool) -> *mut NetObject<SystemType> {
    unsafe {
        DOTNET_RUNTIME.get_mut().unwrap().search_type(type_name, throw_on_error)
    }
}

pub fn search_type_cached(type_name: &String, throw_on_error: bool) -> *mut NetObject<SystemType> {
    unsafe {
        DOTNET_RUNTIME.get_mut().unwrap().search_type_cached(type_name, throw_on_error)
    }
}

pub fn allocate_string(text: &String) -> *mut NetObject<SystemString> {
    unsafe {
        DOTNET_RUNTIME.get_mut().unwrap().allocate_string(text)
    }
}

pub fn allocate_array<T>(element_type: *mut NetObject<SystemType>, size: i32) -> *mut NetObject<SystemArray<T>> {
    unsafe {
        DOTNET_RUNTIME.get_mut().unwrap().allocate_array(element_type, size)
    }
}
