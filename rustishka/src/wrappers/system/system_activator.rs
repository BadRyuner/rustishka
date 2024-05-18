use crate::{define_function, define_typeof};

use super::{system_array::SystemArray, system_reflection::SystemType, NetObject, SystemObject};

pub struct Activator { }

define_typeof!(Activator, "System.Activator");

impl Activator {
    define_function!(pub create_instance_1, 7, *mut NetObject<SystemObject>, tape : *mut NetObject<SystemType>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub create_instance_2, 8, *mut NetObject<SystemObject>, tape : *mut NetObject<SystemType>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, activation_attributes : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub create_instance, 9, *mut NetObject<SystemObject>, tape : *mut NetObject<SystemType>);
    define_function!(pub create_instance_3, 17, *mut NetObject<SystemObject>, tape : *mut NetObject<SystemType>, non_public : bool);
    define_function!(pub create_instance_4, 18, *mut NetObject<SystemObject>, tape : *mut NetObject<SystemType>, non_public : bool, wrap_exceptions : bool);
    //define_function!(pub create_instance, 4, *mut NetObject<T>);
    //define_function!(pub create_default_instance, 5, T);
}