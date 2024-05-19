use crate::{define_constructor, define_function, define_typeof, define_virtual, wrappers::system::system_reflection::MethodBase};

use super::{system_string::SystemString, NetObject};

pub struct Exception { }

define_typeof!(Exception, "System.Exception");

impl Exception {
    define_constructor!(pub new);
    define_constructor!(pub new_1, message : *mut NetObject<SystemString>);
    define_constructor!(pub new_2, message : *mut NetObject<SystemString>, inner_exception : *mut NetObject<Exception>);
}

impl NetObject<Exception> {
    // Virtual functions
    define_virtual!(pub get_message, 0, 4, *mut NetObject<SystemString>);
    //define_virtual!(pub get_data, 0, 5, *mut NetObject<IDictionary>);
    define_virtual!(pub get_base_exception, 0, 6, *mut NetObject<Exception>);
    define_virtual!(pub get_help_link, 0, 7, *mut NetObject<SystemString>);
    define_virtual!(pub set_help_link, 1, 0, (), value : *mut NetObject<SystemString>);
    define_virtual!(pub get_source, 1, 1, *mut NetObject<SystemString>);
    define_virtual!(pub set_source, 1, 2, (), value : *mut NetObject<SystemString>);
    //define_virtual!(pub get_object_data, 1, 3, (), info : *mut NetObject<SerializationInfo>, context : StreamingContext);
    define_virtual!(pub get_stack_trace, 1, 4, *mut NetObject<SystemString>);

    // Non-Virtual functions
    define_function!(pub get_target_site, 18, *mut NetObject<MethodBase>, self: *mut Self);
    define_function!(pub get_inner_exception, 37, *mut NetObject<Exception>, self: *mut Self);
    define_function!(pub get_h_result, 40, i32, self: *mut Self);
    define_function!(pub set_h_result, 41, (), self: *mut Self, value : i32);
}