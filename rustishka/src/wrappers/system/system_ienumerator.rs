use crate::{define_typeof, managed_array, resolve_interface_method};

use super::{system_reflection::{MethodBaseBindings, SystemType}, NetObject, SystemObject, TypeInfoProvider};

pub struct IEnumerator { }
define_typeof!(IEnumerator, "System.Collections.IEnumerator");
impl NetObject<IEnumerator> {
    pub fn move_next(self: *mut Self) -> bool {
        let target = resolve_interface_method!(self, Self, 0, 0);
        *(target.invoke(self as _, managed_array!(SystemObject, 0)) as *mut NetObject<bool>).get_content()
    }

    pub fn current(self: *mut Self) -> *mut NetObject<SystemObject> {
        let target = resolve_interface_method!(self, Self, 1, 0);
        target.invoke(self as _, managed_array!(SystemObject, 0))
    }

    pub fn reset(self: *mut Self) -> *mut NetObject<SystemObject> {
        let target = resolve_interface_method!(self, Self, 2, 0);
        target.invoke(self as _, managed_array!(SystemObject, 0))
    }
}