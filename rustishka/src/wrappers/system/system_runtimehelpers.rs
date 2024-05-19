use crate::{define_function, define_typeof, wrappers::system::{system_delegate::Delegate, MethodTable, NetObject, SystemObject}};

use super::{system_reflection::SystemType, ByRef};

pub struct RuntimeHelpers { }

define_typeof!(RuntimeHelpers, "System.Runtime.CompilerServices.RuntimeHelpers");

impl RuntimeHelpers {
    define_function!(pub get_object_value, 12, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>);
    define_function!(pub prepare_delegate, 21, (), d : *mut NetObject<Delegate>);
    define_function!(pub get_hash_code, 22, i32, o : *mut NetObject<SystemObject>);
    define_function!(pub try_get_hash_code, 23, i32, o : *mut NetObject<SystemObject>);
    define_function!(pub get_offset_to_string_data, 25, i32);
    define_function!(pub ensure_sufficient_execution_stack, 26, ());
    define_function!(pub try_ensure_sufficient_execution_stack, 27, bool);
    define_function!(pub get_uninitialized_object, 28, *mut NetObject<SystemObject>, tape : *mut NetObject<SystemType>);
    define_function!(pub allocate_uninitialized_clone, 30, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>);
    define_function!(pub is_reference_or_contains_references, 4, bool);
    define_function!(pub get_raw_data, 31, ByRef<u8>, obj : *mut NetObject<SystemObject>);
    define_function!(pub get_raw_object_data_size, 32, usize, obj : *mut NetObject<SystemObject>);
    define_function!(pub object_has_component_size, 36, bool, obj : *mut NetObject<SystemObject>);
    define_function!(pub _box, 37, *mut NetObject<SystemObject>, method_table : *mut MethodTable, data : *mut u8);
    define_function!(pub get_obect_method_table, 38, *mut MethodTable, obj : *mut NetObject<SystemObject>);
    define_function!(pub are_types_equivalent, 39, bool, p_m_ta : *mut MethodTable, p_m_tb : *mut MethodTable);
    define_function!(pub allocate_type_associated_memory, 40, isize, tape : *mut NetObject<SystemType>, size : i32);
    define_function!(pub alloc_tail_call_arg_buffer, 42, isize, size : i32, gc_desc : isize);
    define_function!(pub prepare_contracted_delegate, 46, (), d : *mut NetObject<Delegate>);
    define_function!(pub probe_for_sufficient_stack, 47, ());
    define_function!(pub prepare_constrained_regions, 48, ());
    define_function!(pub prepare_constrained_regions_no_op, 49, ());
}