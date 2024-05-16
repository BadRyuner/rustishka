use std::{slice, sync::OnceLock};

use rustishka::{imports::{DotnetImports, DotnetImportsContainer}, wrappers::system::{system_string::SystemString, system_type::SystemType, NetObject, SystemObject, SystemObjectBindings}};

static mut DOTNET_RUNTIME: OnceLock<DotnetImportsContainer> = OnceLock::new();

#[no_mangle]
extern "stdcall" fn rustishka_handshake(imports: *mut DotnetImports) {
    unsafe {
        DOTNET_RUNTIME.set(DotnetImportsContainer::new(imports)).expect("Cant initialize library");
    }
}

#[no_mangle]
extern "stdcall" fn try_call_to_string(obj: *mut NetObject<SystemObject>) -> *mut NetObject<SystemString> {
    obj.to_system_string()
}

#[no_mangle]
extern "stdcall" fn i_can_compare_objects_too(obj: *mut NetObject<SystemObject>, other: *mut NetObject<SystemObject>) -> bool {
    obj.equals(other)
}

#[no_mangle]
extern "stdcall" fn find_class_4_me_senpai(name: *const u16, len: usize) -> *mut NetObject<SystemType> {
    unsafe {
        let slice = slice::from_raw_parts(name, len);
        let string = String::from_utf16_lossy(slice);
        DOTNET_RUNTIME.get_mut().unwrap().search_type_cached(&string, true)
    }
}

#[no_mangle]
extern "stdcall" fn alloc_object(tape: *mut NetObject<SystemType>) -> *mut NetObject<SystemObject> {
    unsafe {
        tape.allocate(DOTNET_RUNTIME.get_mut().unwrap())
    }
}

#[no_mangle]
extern "stdcall" fn get_basetype(tape: *mut NetObject<SystemType>) -> *mut NetObject<SystemType> {
    tape.get_basetype()
}