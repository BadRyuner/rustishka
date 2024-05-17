use crate::define_virtual;

use super::{system_array::SystemArray, system_reflection::MethodInfo, NetObject, SystemObject};

#[repr(C)]
pub struct Delegate {
    target: usize,
    method_base: usize,
    method_ptr: usize,
    method_ptr_aux: usize,
}

#[repr(C)]
pub struct MulticastDelegate {
    parent: Delegate,
    invocation_list: usize,
    invocation_count: usize
}

pub trait DelegateBindings {
    define_virtual!(dynamic_invoke_impl, 0, 4, *mut NetObject<SystemObject>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_virtual!(get_method_impl, 0, 5, *mut NetObject<MethodInfo>);
    define_virtual!(get_target, 0, 6, *mut NetObject<SystemObject>);
    define_virtual!(clone, 0, 7, *mut NetObject<SystemObject>);
    define_virtual!(combine_impl, 1, 0, *mut NetObject<Delegate>, d : *mut NetObject<Delegate>);
    define_virtual!(remove_impl, 1, 1, *mut NetObject<Delegate>, d : *mut NetObject<Delegate>);
    define_virtual!(get_invocation_list, 1, 2, *mut NetObject<SystemArray<*mut NetObject<Delegate>>>);
    //define_virtual!(get_object_data, 1, 3, (), info : *mut NetObject<SerializationInfo>, context : StreamingContext);
}

impl DelegateBindings for NetObject<Delegate> {}
impl DelegateBindings for NetObject<MulticastDelegate> {}

// TODO: add macros for Action<> & Func<>