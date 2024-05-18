use std::marker::PhantomData;

use crate::{define_function, define_typeof, define_virtual, managed_array};

use super::{system_array::SystemArray, system_reflection::{MethodInfo, SystemType}, NetObject, SystemObject, TypeInfoProvider};

#[repr(C)]
pub struct Delegate {
    target: usize,
    method_base: usize,
    method_ptr: usize,
    method_ptr_aux: usize,
}

define_typeof!(Delegate, "System.Delegate");

impl Delegate {
    define_function!(pub internal_alloc, 23, *mut NetObject<MulticastDelegate>, tape : *mut NetObject<SystemType>);
}

#[repr(C)]
pub struct MulticastDelegate {
    parent: Delegate,
    invocation_list: usize,
    invocation_count: usize
}

define_typeof!(MulticastDelegate, "System.MulticastDelegate");

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

// Action start

pub struct Action { pub parent: MulticastDelegate }

define_typeof!(Action, "System.Action");

impl Action {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall"  fn()) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl NetObject<Action> {
    define_virtual!(pub invoke, 1, 4, ());
}

// T1

pub struct Action1<T1> { 
    pub parent: MulticastDelegate,
    omg: PhantomData<T1>
}

impl<T1: TypeInfoProvider> TypeInfoProvider for Action1<T1> {
    fn type_of() ->  *mut NetObject<SystemType>{
        static MY_TYPE:std::sync::OnceLock<usize>  = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| crate::search_type_cached(&String::from("System.Action`1"),true)
            .make_generic_type(managed_array!(SystemType, 1, T1::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<T1: TypeInfoProvider> Action1<T1> {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall" fn(T1)) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl<T1> NetObject<Action1<T1>> {
    define_virtual!(pub invoke, 1, 4, (), arg1: T1);
}

// T2

pub struct Action2<T1, T2> { 
    pub parent: MulticastDelegate,
    omg: PhantomData<T1>,
    huh: PhantomData<T2>
}

impl<T1: TypeInfoProvider, T2: TypeInfoProvider> TypeInfoProvider for Action2<T1, T2> {
    fn type_of() ->  *mut NetObject<SystemType>{
        static MY_TYPE:std::sync::OnceLock<usize>  = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| crate::search_type_cached(&String::from("System.Action`2"),true)
            .make_generic_type(managed_array!(SystemType, 2, T1::type_of(), T2::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<T1: TypeInfoProvider, T2: TypeInfoProvider> Action2<T1, T2> {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall" fn(T1, T2)) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl<T1, T2> NetObject<Action2<T1, T2>> {
    define_virtual!(pub invoke, 1, 4, (), arg1: T1, arg2: T2);
}

// T3

pub struct Action3<T1, T2, T3> { 
    pub parent: MulticastDelegate,
    omg: PhantomData<T1>,
    huh: PhantomData<T2>,
    bruh: PhantomData<T3>
}

impl<T1: TypeInfoProvider, T2: TypeInfoProvider, T3: TypeInfoProvider> TypeInfoProvider for Action3<T1, T2, T3> {
    fn type_of() ->  *mut NetObject<SystemType>{
        static MY_TYPE:std::sync::OnceLock<usize>  = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| crate::search_type_cached(&String::from("System.Action`3"),true)
            .make_generic_type(managed_array!(SystemType, 3, T1::type_of(), T2::type_of(), T3::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<T1: TypeInfoProvider, T2: TypeInfoProvider, T3: TypeInfoProvider> Action3<T1, T2, T3> {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall" fn(T1, T2, T3)) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl<T1, T2, T3> NetObject<Action3<T1, T2, T3>> {
    define_virtual!(pub invoke, 1, 4, (), arg1: T1, arg2: T2, arg3: T3);
}

// I'm not lazy, I just can't count beyond three.

// Action end





// Func start

pub struct Func1<Ret> { 
    pub parent: MulticastDelegate,
    omg: PhantomData<Ret>
}

impl<Ret: TypeInfoProvider> TypeInfoProvider for Func1<Ret> {
    fn type_of() ->  *mut NetObject<SystemType>{
        static MY_TYPE:std::sync::OnceLock<usize>  = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| crate::search_type_cached(&String::from("System.Func`1"),true)
            .make_generic_type(managed_array!(SystemType, 1, Ret::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<Ret: TypeInfoProvider> Func1<Ret> {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall" fn() -> Ret) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl<Ret> NetObject<Func1<Ret>> {
    define_virtual!(pub invoke, 1, 4, Ret);
}

// T1

pub struct Func2<T1, Ret> { 
    pub parent: MulticastDelegate,
    omg: PhantomData<Ret>,
    huh: PhantomData<T1>
}

impl<T1: TypeInfoProvider, Ret: TypeInfoProvider> TypeInfoProvider for Func2<T1, Ret> {
    fn type_of() ->  *mut NetObject<SystemType>{
        static MY_TYPE:std::sync::OnceLock<usize>  = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| crate::search_type_cached(&String::from("System.Func`2"),true)
            .make_generic_type(managed_array!(SystemType, 2, T1::type_of(), Ret::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<T1: TypeInfoProvider, Ret: TypeInfoProvider> Func2<T1, Ret> {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall" fn(T1) -> Ret) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl<T1, Ret> NetObject<Func2<T1, Ret>> {
    define_virtual!(pub invoke, 1, 4, Ret, arg1: T1);
}

// T2

pub struct Func3<T1, T2, Ret> { 
    pub parent: MulticastDelegate,
    omg: PhantomData<Ret>,
    huh: PhantomData<T1>,
    bruh: PhantomData<T2>
}

impl<T1: TypeInfoProvider, T2: TypeInfoProvider, Ret: TypeInfoProvider> TypeInfoProvider for Func3<T1, T2, Ret> {
    fn type_of() ->  *mut NetObject<SystemType>{
        static MY_TYPE:std::sync::OnceLock<usize>  = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| crate::search_type_cached(&String::from("System.Func`3"),true)
            .make_generic_type(managed_array!(SystemType, 2, T1::type_of(), T2::type_of(), Ret::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<T1: TypeInfoProvider, T2: TypeInfoProvider, Ret: TypeInfoProvider> Func3<T1, T2, Ret> {
    pub fn new(object: *mut NetObject<SystemObject>, method: extern "stdcall" fn(T1, T2) -> Ret) -> *mut NetObject<Self>  {
        unsafe {
            let func: usize = core::mem::transmute(method);
            let inst = Delegate::internal_alloc(Self::type_of());
            (*inst).content.parent.target = object as usize;
            (*inst).content.parent.method_ptr = func;
            inst.cast()
        }
    }
}

impl<T1, T2, Ret> NetObject<Func3<T1, T2, Ret>> {
    define_virtual!(pub invoke, 1, 4, Ret, arg1: T1, arg2: T2);
}

// Func end