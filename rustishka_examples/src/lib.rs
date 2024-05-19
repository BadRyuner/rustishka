use rustishka::wrappers::system::system_appdomain::AppDomain;
use rustishka::wrappers::system::system_delegate::{Action, Action1, Action2, Action3, DelegateBindings, Func1, Func2, Func3};
use rustishka::wrappers::system::system_reflection::AssemblyName;
use rustishka::wrappers::system::TypeInfoProvider;
use rustishka::{allocate_string, initialize_rustishka, search_type_cached, DOTNET_RUNTIME};
use rustishka::wrappers::system::{system_delegate::Delegate, system_reflection::MethodBaseBindings};

use rustishka::wrappers::system::{system_array::SystemArray, system_reflection::{BindingFlags, SystemType}, system_string::SystemString, NetObject, SystemObject, SystemObjectBindings};

initialize_rustishka!();

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
    let slice = unsafe { std::slice::from_raw_parts(name, len) };
    let string = String::from_utf16_lossy(slice);
    search_type_cached(&string, true)
}

#[no_mangle]
extern "stdcall" fn alloc_object(tape: *mut NetObject<SystemType>) -> *mut NetObject<SystemObject> {
        tape.allocate()
}

#[no_mangle]
extern "stdcall" fn get_basetype(tape: *mut NetObject<SystemType>) -> *mut NetObject<SystemType> {
    tape.get_base_type()
}

#[no_mangle]
extern "stdcall" fn try_typeof_sys_object() -> *mut NetObject<SystemType> {
    SystemObject::type_of()
}

#[no_mangle]
extern "stdcall" fn try_get_base_dir() -> *mut NetObject<SystemString> {
    NetObject::<AppDomain>::get_current_domain().get_base_directory()
}

#[no_mangle]
extern "stdcall" fn create_assembly_name() -> *mut NetObject<AssemblyName> {
    AssemblyName::new(allocate_string(&String::from("PoopAssembly, Version=1.2.3.4, Culture=neutral, PublicKeyToken=null")))
}

#[no_mangle]
extern "stdcall" fn try_call_delegate_without_args(delegate: *mut NetObject<Delegate>) {
    delegate.dynamic_invoke_impl(std::ptr::null_mut());
}

#[no_mangle]
extern "stdcall" fn find_and_invoke(class: *mut NetObject<SystemString>, method: *mut NetObject<SystemString>, args: *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>) -> *mut NetObject<SystemObject> {
    unsafe {
        let tape = DOTNET_RUNTIME.get_mut().unwrap().search_type(class, true);
        let method = tape.get_method(method, BindingFlags::from_bits(BindingFlags::Public.bits() | BindingFlags::Static.bits()).unwrap());
        method.invoke(std::ptr::null_mut(), args)
    }
}

#[no_mangle]
extern "stdcall" fn try_get_field_from_instance(object: *mut NetObject<SystemObject>, field: *mut NetObject<SystemString>) -> *mut NetObject<SystemObject> {
    let tape = object.get_type();
    let field_info = tape.get_field(field, BindingFlags::PublicInstance);
    field_info.get_value(object)
}

#[no_mangle]
extern "stdcall" fn try_some_do_with_instance_field(object: *mut NetObject<SystemObject>, field: *mut NetObject<SystemString>) {
    let tape = object.get_type();
    let field_info = tape.get_field(field, BindingFlags::PublicInstance);
    let field_type = field_info.get_field_type();
    if field_type.equals(search_type_cached(&String::from("System.Int32"), false)) {
        let value: *mut NetObject<i32> = field_info.get_value(object).cast_unchecked();
        unsafe { (*value).content = 0xBEEFi32 }
        field_info.set_value(object, value.cast_unchecked());
    }
    else if field_type.equals(search_type_cached(&String::from("System.Single"), false)) {
        let value: *mut NetObject<f32> = field_info.get_value(object).cast_unchecked();
        unsafe { (*value).content = 3.14f32 }
        field_info.set_value(object, value.cast_unchecked());
    }
}

#[no_mangle]
extern "stdcall" fn test_action(delegate: *mut NetObject<Action>) {
    delegate.invoke()
}

#[no_mangle]
extern "stdcall" fn test_action_1(delegate: *mut NetObject<Action1<*mut NetObject<SystemObject>>>, arg1: *mut NetObject<SystemObject>) {
    delegate.invoke(arg1)
}

#[no_mangle]
extern "stdcall" fn test_action_2(delegate: *mut NetObject<Action2<*mut NetObject<SystemObject>, *mut NetObject<SystemObject>>>, arg1: *mut NetObject<SystemObject>, arg2: *mut NetObject<SystemObject>) {
    delegate.invoke(arg1, arg2)
}

#[no_mangle]
extern "stdcall" fn test_action_3(delegate: *mut NetObject<Action3<*mut NetObject<SystemObject>, *mut NetObject<SystemObject>, *mut NetObject<SystemObject>>>, arg1: *mut NetObject<SystemObject>, arg2: *mut NetObject<SystemObject>, arg3: *mut NetObject<SystemObject>) {
    delegate.invoke(arg1, arg2, arg3)
}

#[no_mangle]
extern "stdcall" fn test_func1(delegate: *mut NetObject<Func1<*mut NetObject<SystemObject>>>) -> *mut NetObject<SystemObject> {
    delegate.invoke()
}

#[no_mangle]
extern "stdcall" fn test_func2(delegate: *mut NetObject<Func2<*mut NetObject<SystemObject>, *mut NetObject<SystemObject>>>, arg1: *mut NetObject<SystemObject>) -> *mut NetObject<SystemObject> {
    delegate.invoke(arg1)
}

#[no_mangle]
extern "stdcall" fn test_func3(delegate: *mut NetObject<Func3<*mut NetObject<SystemObject>, *mut NetObject<SystemObject>, *mut NetObject<SystemObject>>>, arg1: *mut NetObject<SystemObject>, arg2: *mut NetObject<SystemObject>) -> *mut NetObject<SystemObject> {
    delegate.invoke(arg1, arg2)
}

#[no_mangle]
extern "stdcall" fn test_pass_func1() -> *mut NetObject<Func1<bool>> {
    Func1::<bool>::new(std::ptr::null_mut(), always_true)
}

extern "stdcall" fn always_true() -> bool { true }

#[no_mangle]
extern "stdcall" fn test_pass_func2() -> *mut NetObject<Func2<i32, i32>> {
    Func2::<i32, i32>::new(std::ptr::null_mut(), redir_bool)
}

#[no_mangle]
extern "stdcall" fn redir_bool(val: i32) -> i32 { val }

#[no_mangle]
extern "stdcall" fn some_func() {
    //NetObject::<SystemString>::
}