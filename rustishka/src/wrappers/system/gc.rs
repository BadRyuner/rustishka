use crate::{define_function, define_typeof, wrappers::system::{NetObject, SystemObject}};

pub struct GC {}

define_typeof!(GC, "System.GC");

impl GC {
    define_function!(pub collect, 26, ());
    define_function!(pub keep_alive, 31, (), obj : *mut NetObject<SystemObject>);
    define_function!(pub wait_for_pending_finalizers, 35, ());
    define_function!(pub suppress_finalize, 37, (), obj : *mut NetObject<SystemObject>);
}