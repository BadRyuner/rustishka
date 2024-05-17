use bitflags::bitflags;

use crate::{define_virtual, DOTNET_RUNTIME};

use super::{system_array::SystemArray, system_delegate::Delegate, system_string::SystemString, MethodTable, NetObject, SystemObject, SystemObjectBindings};

pub struct MemberInfo { }

impl SystemObjectBindings for NetObject<MemberInfo> {}
impl MemberInfoBindings for NetObject<MemberInfo> {}

pub trait MemberInfoBindings {
    /*
    define_virtual!(cache_equals, 0, 4, bool, o : *mut NetObject<SystemObject>);
    define_virtual!(get_custom_attributes, 1, 5, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, attributeType : *mut NetObject<SystemType>, inherit : bool);
    define_virtual!(get_custom_attributes, 1, 6, *mut NetObject<IEnumerable`1>);
    define_virtual!(get_custom_attributes_data, 1, 7, *mut NetObject<IList`1>);
    */
    define_virtual!(get_member_type, 0, 5, MemberTypes);
    define_virtual!(get_name, 0, 6, *mut NetObject<SystemString>);
    define_virtual!(get_declaring_type, 0, 7, *mut NetObject<SystemType>);
    define_virtual!(get_reflected_type, 1, 0, *mut NetObject<SystemType>);
    define_virtual!(get_module, 1, 1, *mut NetObject<Module>);
    define_virtual!(has_same_metadata_definition_as, 1, 2, bool, other : *mut NetObject<MemberInfo>);
    define_virtual!(is_defined, 1, 3, bool, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    define_virtual!(get_custom_attributes, 1, 4, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, inherit : bool);
    define_virtual!(get_is_collectible, 2, 0, bool);
    define_virtual!(get_metadata_token, 2, 1, i32);
}

bitflags! {
    pub struct MemberTypes: u32
    {
        const Constructor = 1;
        const Event = 2;
        const Field = 4;
        const Method = 8;
        const Property = 16;
        const TypeInfo = 32;
        const Custom = 64;
        const NestedType = 128;
        const All = 0x000000BF;
    }
}

#[repr(C)]
pub struct SystemType {
    keep_alive: *mut NetObject<SystemObject>,
    m_cache:  *mut NetObject<SystemObject>,
    pub handle: *mut MethodTable
}

impl SystemObjectBindings for NetObject<SystemType> {}
impl MemberInfoBindings for NetObject<SystemType> {}

impl NetObject<SystemType> {
    pub fn allocate<T>(self: *mut Self) -> *mut NetObject<T> {
        unsafe {
            DOTNET_RUNTIME.get().unwrap().allocate(self)
        }
    }

    define_virtual!(pub, get_namespace, 2, 2, *mut NetObject<SystemString>);
    define_virtual!(pub, get_assembly_qualified_name, 2, 3, *mut NetObject<SystemString>);
    define_virtual!(pub, get_full_name, 2, 4, *mut NetObject<SystemString>);
    define_virtual!(pub, get_assembly, 2, 5, *mut NetObject<Assembly>);
    define_virtual!(pub, get_module, 2, 6, *mut NetObject<Module>);
    //define_virtual!(pub, get_declaring_method, 2, 7, *mut NetObject<MethodBase>);
    define_virtual!(pub, get_underlying_system_type, 3, 0, *mut NetObject<SystemType>);
    define_virtual!(pub, get_is_type_definition, 3, 1, bool);
    define_virtual!(pub, is_array_impl, 3, 2, bool);
    define_virtual!(pub, is_by_ref_impl, 3, 3, bool);
    define_virtual!(pub, is_pointer_impl, 3, 4, bool);
    define_virtual!(pub, get_is_constructed_generic_type, 3, 5, bool);
    define_virtual!(pub, get_is_generic_parameter, 3, 6, bool);
    define_virtual!(pub, get_is_generic_type_parameter, 3, 7, bool);
    define_virtual!(pub, get_is_generic_method_parameter, 4, 0, bool);
    define_virtual!(pub, get_is_generic_type, 4, 1, bool);
    define_virtual!(pub, get_is_generic_type_definition, 4, 2, bool);
    define_virtual!(pub, get_is_sz_array, 4, 3, bool);
    define_virtual!(pub, get_is_variable_bound_array, 4, 4, bool);
    define_virtual!(pub, get_is_by_ref_like, 4, 5, bool);
    define_virtual!(pub, get_is_function_pointer, 4, 6, bool);
    define_virtual!(pub, get_is_unmanaged_function_pointer, 4, 7, bool);
    define_virtual!(pub, has_element_type_impl, 5, 0, bool);
    define_virtual!(pub, get_element_type, 5, 1, *mut NetObject<SystemType>);
    define_virtual!(pub, get_array_rank, 5, 2, i32);
    define_virtual!(pub, get_generic_type_definition, 5, 3, *mut NetObject<SystemType>);
    define_virtual!(pub, get_generic_type_arguments, 5, 4, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_generic_arguments, 5, 5, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_optional_custom_modifiers, 5, 6, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_required_custom_modifiers, 5, 7, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_generic_parameter_position, 6, 0, i32);
    define_virtual!(pub, get_generic_parameter_attributes, 6, 1, GenericParameterAttributes);
    define_virtual!(pub, get_generic_parameter_constraints, 6, 2, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, is_com_object_impl, 6, 4, bool);
    define_virtual!(pub, is_contextful_impl, 6, 5, bool);
    define_virtual!(pub, get_is_enum, 6, 6, bool);
    define_virtual!(pub, is_marshal_by_ref_impl, 6, 7, bool);
    define_virtual!(pub, is_primitive_impl, 7, 0, bool);
    define_virtual!(pub, is_value_type_impl, 7, 1, bool);
    define_virtual!(pub, get_is_signature_type, 7, 2, bool);
    define_virtual!(pub, get_is_security_critical, 7, 3, bool);
    define_virtual!(pub, get_is_security_safe_critical, 7, 4, bool);
    define_virtual!(pub, get_is_security_transparent, 7, 5, bool);
    //define_virtual!(pub, get_struct_layout_attribute, 7, 6, *mut NetObject<StructLayoutAttribute>);
    define_virtual!(pub, get_constructors, 8, 0, *mut NetObject<SystemArray<*mut NetObject<ConstructorInfo>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_event, 8, 1, *mut NetObject<EventInfo>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_events, 8, 3, *mut NetObject<SystemArray<*mut NetObject<EventInfo>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_field, 8, 4, *mut NetObject<FieldInfo>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_fields, 8, 5, *mut NetObject<SystemArray<*mut NetObject<FieldInfo>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_function_pointer_calling_conventions, 8, 6, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_function_pointer_return_type, 8, 7, *mut NetObject<SystemType>);
    define_virtual!(pub, get_function_pointer_parameter_types, 9, 0, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_member, 9, 1, *mut NetObject<SystemArray<*mut NetObject<MemberInfo>>>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_member_1, 9, 2, *mut NetObject<SystemArray<*mut NetObject<MemberInfo>>>, name : *mut NetObject<SystemString>, _type : MemberTypes, binding_attr : BindingFlags);
    define_virtual!(pub, get_member_with_same_metadata_definition_as, 9, 3, *mut NetObject<MemberInfo>, member : *mut NetObject<MemberInfo>);
    define_virtual!(pub, get_members, 9, 4, *mut NetObject<SystemArray<*mut NetObject<MemberInfo>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_method, 9, 5, *mut NetObject<MethodInfo>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_methods, 10, 1, *mut NetObject<SystemArray<*mut NetObject<MethodInfo>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_nested_type, 10, 2, *mut NetObject<SystemType>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_nested_types, 10, 3, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_property, 10, 4, *mut NetObject<PropertyInfo>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_properties, 10, 7, *mut NetObject<SystemArray<*mut NetObject<PropertyInfo>>>, binding_attr : BindingFlags);
    define_virtual!(pub, get_default_members, 11, 0, *mut NetObject<SystemArray<*mut NetObject<MemberInfo>>>);
    //define_virtual!(pub, get_guid, 11, 3, Guid);
    define_virtual!(pub, get_base_type, 11, 4, *mut NetObject<SystemType>);
    //define_virtual!(pub, invoke_member, 11, 5, *mut NetObject<SystemObject>, name : *mut NetObject<SystemString>, invoke_attr : BindingFlags, binder : *mut NetObject<Binder>, target : *mut NetObject<SystemObject>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, modifiers : *mut NetObject<SystemArray<ParameterModifier>>, culture : *mut NetObject<CultureInfo>, named_parameters : *mut NetObject<SystemArray<*mut NetObject<SystemString>>>);
    define_virtual!(pub, get_interface, 11, 6, *mut NetObject<SystemType>, name : *mut NetObject<SystemString>, ignore_case : bool);
    define_virtual!(pub, get_interfaces, 11, 7, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    //define_virtual!(pub, get_interface_map, 12, 0, InterfaceMapping, interface_type : *mut NetObject<SystemType>);
    define_virtual!(pub, is_instance_of_type, 12, 1, bool, o : *mut NetObject<SystemObject>);
    define_virtual!(pub, is_equivalent_to, 12, 2, bool, other : *mut NetObject<SystemType>);
    define_virtual!(pub, get_enum_underlying_type, 12, 3, *mut NetObject<SystemType>);
    //define_virtual!(pub, get_enum_values, 12, 4, *mut NetObject<Array>);
    //define_virtual!(pub, get_enum_values_as_underlying_type, 12, 5, *mut NetObject<Array>);
    define_virtual!(pub, make_array_type, 12, 6, *mut NetObject<SystemType>);
    define_virtual!(pub, make_array_type_multi, 12, 7, *mut NetObject<SystemType>, rank : i32);
    define_virtual!(pub, make_by_ref_type, 13, 0, *mut NetObject<SystemType>);
    define_virtual!(pub, make_generic_type, 13, 1, *mut NetObject<SystemType>, type_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, make_pointer_type, 13, 2, *mut NetObject<SystemType>);
    define_virtual!(pub, equals, 13, 3, bool, o : *mut NetObject<SystemType>);
    define_virtual!(pub, is_enum_defined, 13, 4, bool, value : *mut NetObject<SystemObject>);
    define_virtual!(pub, get_enum_name, 13, 5, *mut NetObject<SystemString>, value : *mut NetObject<SystemObject>);
    define_virtual!(pub, get_enum_names, 13, 6, *mut NetObject<SystemArray<*mut NetObject<SystemString>>>);
    define_virtual!(pub, get_is_serializable, 13, 7, bool);
    define_virtual!(pub, get_contains_generic_parameters, 14, 0, bool);
    //define_virtual!(pub, find_interfaces, 14, 1, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, filter : *mut NetObject<TypeFilter>, filter_criteria : *mut NetObject<SystemObject>);
    //define_virtual!(pub, find_members, 14, 2, *mut NetObject<SystemArray<*mut NetObject<MemberInfo>>>, member_type : MemberTypes, binding_attr : BindingFlags, filter : *mut NetObject<MemberFilter>, filter_criteria : *mut NetObject<SystemObject>);
    define_virtual!(pub, is_subclass_of, 14, 3, bool, c : *mut NetObject<SystemType>);
    define_virtual!(pub, is_assignable_from, 14, 4, bool, c : *mut NetObject<SystemType>);
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BindingFlags: u32 {
        const Default = 0;
        const IgnoreCase = 1;
        const DeclaredOnly = 2;
        const Instance = 4;
        const Static = 8;
        const Public = 16; // 0x00000010
        const NonPublic = 32; // 0x00000020
        const FlattenHierarchy = 64; // 0x00000040
        const InvokeMethod = 256; // 0x00000100
        const CreateInstance = 512; // 0x00000200
        const GetField = 1024; // 0x00000400
        const SetField = 2048; // 0x00000800
        const GetProperty = 4096; // 0x00001000
        const SetProperty = 8192; // 0x00002000
        const PutDispProperty = 16384; // 0x00004000
        const PutRefDispProperty = 32768; // 0x00008000
        const ExactBinding = 65536; // 0x00010000
        const SuppressChangeType = 131072; // 0x00020000
        const OptionalParamBinding = 262144; // 0x00040000
        const IgnoreReturn = 16777216; // 0x01000000
        const DoNotWrapExceptions = 33554432; // 0x02000000

        const PublicInstance = Self::Public.bits() | Self::Instance.bits();
        const PublicStatic = Self::Public.bits() | Self::Static.bits();
        const PublicStaticAndInstance = Self::Public.bits() | Self::Static.bits() | Self::Instance.bits();
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GenericParameterAttributes: u32 {
        const None = 0;
        const VarianceMask = 3;
        const Covariant = 1;
        const Contravariant = 2;
        const SpecialConstraintMask = 28; // 0x0000001C
        const ReferenceTypeConstraint = 4;
        const NotNullableValueTypeConstraint = 8;
        const DefaultConstructorConstraint = 16; // 0x00000010
    }
}

pub struct MethodBase { }

impl SystemObjectBindings for NetObject<MethodBase> {}
impl MemberInfoBindings for NetObject<MethodBase> {}

pub trait MethodBaseBindings {
    define_virtual!(get_parameters, 2, 3, *mut NetObject<SystemArray<*mut NetObject<ParameterInfo>>>);
    define_virtual!(get_attributes, 2, 4, MethodAttributes);
    //define_virtual!(get_method_implementation_flags, 2, 5, MethodImplAttributes);
    //define_virtual!(get_method_implementation_flags, 2, 6, MethodImplAttributes);
    //define_virtual!(get_method_body, 2, 7, *mut NetObject<MethodBody>);
    define_virtual!(get_calling_convention, 3, 0, CallingConventions);
    define_virtual!(get_is_constructed_generic_method, 3, 1, bool);
    define_virtual!(get_is_generic_method, 3, 2, bool);
    define_virtual!(get_is_generic_method_definition, 3, 3, bool);
    define_virtual!(get_generic_arguments, 3, 4, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(get_contains_generic_parameters, 3, 5, bool);
    //define_virtual!(invoke, 3, 6, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>, invoke_attr : BindingFlags, binder : *mut NetObject<Binder>, parameters : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, culture : *mut NetObject<CultureInfo>);
    define_virtual!(invoke_impl, 3, 6, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>, invoke_attr : u32, binder : usize, parameters : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, culture : usize);
    //define_virtual!(get_method_handle, 3, 7, RuntimeMethodHandle);
    define_virtual!(get_is_security_critical, 4, 0, bool);
    define_virtual!(get_is_security_safe_critical, 4, 1, bool);
    define_virtual!(get_is_security_transparent, 4, 2, bool);
    define_virtual!(get_parameter_types, 4, 3, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);

    fn invoke(self: *mut Self, obj : *mut NetObject<SystemObject>, parameters : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>) -> *mut NetObject<SystemObject> {
        self.invoke_impl(obj, 0, 0, parameters, 0)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CallingConventions: u32 {
        const Standard = 1;
        const VarArgs = 2;
        const Any = 3; // 0x00000003
        const HasThis = 32; // 0x00000020
        const ExplicitThis = 64; // 0x00000040
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MethodAttributes: u32
    {
        const MemberAccessMask = 7;
        const PrivateScope = 0;
        const Private = 1;
        const FamANDAssem = 2;
        const Assembly = 3; // 0x00000003
        const Family = 4;
        const FamORAssem = 5; // 0x00000005
        const Public = 6; // 0x00000006
        const Static = 16; // 0x00000010
        const Final = 32; // 0x00000020
        const Virtual = 64; // 0x00000040
        const HideBySig = 128; // 0x00000080
        const CheckAccessOnOverride = 512; // 0x00000200
        const VtableLayoutMask = 256; // 0x00000100
        const ReuseSlot = 0;
        const NewSlot = 256; // 0x00000100
        const Abstract = 1024; // 0x00000400
        const SpecialName = 2048; // 0x00000800
        const PinvokeImpl = 8192; // 0x00002000
        const UnmanagedExport = 8;
        const RTSpecialName = 4096; // 0x00001000
        const HasSecurity = 16384; // 0x00004000
        const RequireSecObject = 32768; // 0x00008000
        const ReservedMask = 53248; // 0x0000D000
    }
}

impl MethodBaseBindings for NetObject<MethodBase> {}

pub struct ParameterInfo { }
impl NetObject<ParameterInfo> {
    //define_virtual!(pub, get_attributes, 0, 4, ParameterAttributes);
    define_virtual!(pub, get_member, 0, 5, *mut NetObject<MemberInfo>);
    define_virtual!(pub, get_name, 0, 6, *mut NetObject<SystemString>);
    define_virtual!(pub, get_parameter_type, 0, 7, *mut NetObject<SystemType>);
    define_virtual!(pub, get_position, 1, 0, i32);
    define_virtual!(pub, get_default_value, 1, 1, *mut NetObject<SystemObject>);
    define_virtual!(pub, get_raw_default_value, 1, 2, *mut NetObject<SystemObject>);
    define_virtual!(pub, get_has_default_value, 1, 3, bool);
    define_virtual!(pub, is_defined, 1, 4, bool, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    //define_virtual!(pub, get_custom_attributes, 1, 5, *mut NetObject<IEnumerable`1>);
    //define_virtual!(pub, get_custom_attributes_data, 1, 6, *mut NetObject<IList`1>);
    define_virtual!(pub, get_custom_attributes, 1, 7, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, inherit : bool);
    define_virtual!(pub, get_custom_attributes_2, 2, 0, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    define_virtual!(pub, get_modified_parameter_type, 2, 1, *mut NetObject<SystemType>);
    define_virtual!(pub, get_optional_custom_modifiers, 2, 2, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_required_custom_modifiers, 2, 3, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_metadata_token, 2, 4, i32);
    //define_virtual!(pub, get_real_object, 2, 5, *mut NetObject<SystemObject>, context : StreamingContext);
}

pub struct MethodInfo { }

impl SystemObjectBindings for NetObject<MethodInfo> {}
impl MemberInfoBindings for NetObject<MethodInfo> {}
impl MethodBaseBindings for NetObject<MethodInfo> {}

impl NetObject<MethodInfo> {
    define_virtual!(pub, get_return_parameter, 4, 4, *mut NetObject<ParameterInfo>);
    define_virtual!(pub, get_return_type, 4, 5, *mut NetObject<SystemType>);
    define_virtual!(pub, get_generic_method_definition, 4, 6, *mut NetObject<MethodInfo>);
    define_virtual!(pub, make_generic_method, 4, 7, *mut NetObject<MethodInfo>, type_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_base_definition, 5, 0, *mut NetObject<MethodInfo>);
    //define_virtual!(pub, get_return_type_custom_attributes, 5, 1, *mut NetObject<ICustomAttributeProvider>);
    define_virtual!(pub, create_delegate, 5, 2, *mut NetObject<Delegate>, delegate_type : *mut NetObject<SystemType>);
    define_virtual!(pub, create_delegate_with_self, 5, 3, *mut NetObject<Delegate>, delegate_type : *mut NetObject<SystemType>, target : *mut NetObject<SystemObject>);
    define_virtual!(pub, get_generic_parameter_count, 5, 4, i32);
}

pub struct ConstructorInfo { }

impl SystemObjectBindings for NetObject<ConstructorInfo> {}
impl MemberInfoBindings for NetObject<ConstructorInfo> {}
impl MethodBaseBindings for NetObject<ConstructorInfo> {}

pub struct EventInfo { }

impl SystemObjectBindings for NetObject<EventInfo> {}
impl MemberInfoBindings for NetObject<EventInfo> {}

impl NetObject<EventInfo> {
    //define_virtual!(pub, get_attributes, 2, 2, EventAttributes);
    define_virtual!(pub, get_other_methods, 2, 3, *mut NetObject<SystemArray<*mut NetObject<MethodInfo>>>, non_public : bool);
    //define_virtual!(pub, get_add_method, 2, 4, *mut NetObject<MethodInfo>);
    //define_virtual!(pub, get_remove_method, 2, 5, *mut NetObject<MethodInfo>);
    //define_virtual!(pub, get_raise_method, 2, 6, *mut NetObject<MethodInfo>);
    define_virtual!(pub, get_add_method, 2, 7, *mut NetObject<MethodInfo>, non_public : bool);
    define_virtual!(pub, get_remove_method, 3, 0, *mut NetObject<MethodInfo>, non_public : bool);
    define_virtual!(pub, get_raise_method, 3, 1, *mut NetObject<MethodInfo>, non_public : bool);
    define_virtual!(pub, get_is_multicast, 3, 2, bool);
    define_virtual!(pub, get_event_handler_type, 3, 3, *mut NetObject<SystemType>);
    define_virtual!(pub, add_event_handler, 3, 4, (), target : *mut NetObject<SystemObject>, handler : *mut NetObject<Delegate>);
    define_virtual!(pub, remove_event_handler, 3, 5, (), target : *mut NetObject<SystemObject>, handler : *mut NetObject<Delegate>);
}

pub struct FieldInfo { }

impl SystemObjectBindings for NetObject<FieldInfo> {}
impl MemberInfoBindings for NetObject<FieldInfo> {}

impl NetObject<FieldInfo> {
    define_virtual!(pub, get_attributes, 2, 2, FieldAttributes);
    define_virtual!(pub, get_field_type, 2, 3, *mut NetObject<SystemType>);
    define_virtual!(pub, get_is_security_critical, 2, 4, bool);
    define_virtual!(pub, get_is_security_safe_critical, 2, 5, bool);
    define_virtual!(pub, get_is_security_transparent, 2, 6, bool);
    //define_virtual!(pub, get_field_handle, 2, 7, RuntimeFieldHandle);
    define_virtual!(pub, get_value, 3, 0, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>);
    define_virtual!(pub, set_value_impl, 3, 1, (), obj : *mut NetObject<SystemObject>, value : *mut NetObject<SystemObject>, invoke_attr : u32, binder : usize, culture : usize);
    //define_virtual!(pub, set_value_direct, 3, 2, (), obj : TypedReference, value : *mut NetObject<SystemObject>);
    //define_virtual!(pub, get_value_direct, 3, 3, *mut NetObject<SystemObject>, obj : TypedReference);
    define_virtual!(pub, get_raw_constant_value, 3, 4, *mut NetObject<SystemObject>);
    define_virtual!(pub, get_modified_field_type, 3, 5, *mut NetObject<SystemType>);
    define_virtual!(pub, get_optional_custom_modifiers, 3, 6, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_required_custom_modifiers, 3, 7, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);

    pub fn set_value(self: *mut Self, obj : *mut NetObject<SystemObject>, value : *mut NetObject<SystemObject>) {
        self.set_value_impl(obj, value, 0, 0, 0)
    }
}

bitflags! {
    pub struct FieldAttributes : u32
    {
        const FieldAccessMask = 7;
        const PrivateScope = 0;
        const Private = 1;
        const FamANDAssem = 2;
        const Assembly = Self::FamANDAssem.bits() | Self::Private.bits();
        const Family = 4;
        const FamORAssem = Self::Family.bits() | Self::Private.bits();
        const Public = Self::Family.bits() | Self::FamANDAssem.bits();
        const Static = 16;
        const InitOnly = 32;
        const Literal = 64;
        const SpecialName = 512;
        const PinvokeImpl = 8192;
        const RTSpecialName = 1024;
        const HasFieldMarshal = 4096;
        const HasDefault = 32768;
        const HasFieldRVA = 256;
    }
}

pub struct PropertyInfo { }

impl SystemObjectBindings for NetObject<PropertyInfo> {}
impl MemberInfoBindings for NetObject<PropertyInfo> {}

impl NetObject<PropertyInfo> {
    define_virtual!(pub, get_property_type, 2, 2, *mut NetObject<SystemType>);
    define_virtual!(pub, get_index_parameters, 2, 3, *mut NetObject<SystemArray<*mut NetObject<ParameterInfo>>>);
    //define_virtual!(pub, get_attributes, 2, 4, PropertyAttributes);
    define_virtual!(pub, get_can_read, 2, 5, bool);
    define_virtual!(pub, get_can_write, 2, 6, bool);
    define_virtual!(pub, get_accessors, 2, 7, *mut NetObject<SystemArray<*mut NetObject<MethodInfo>>>, non_public : bool);
    //define_virtual!(pub, get_get_method, 3, 0, *mut NetObject<MethodInfo>);
    define_virtual!(pub, get_get_method, 3, 1, *mut NetObject<MethodInfo>, non_public : bool);
    //define_virtual!(pub, get_set_method, 3, 2, *mut NetObject<MethodInfo>);
    define_virtual!(pub, get_set_method, 3, 3, *mut NetObject<MethodInfo>, non_public : bool);
    define_virtual!(pub, get_modified_property_type, 3, 4, *mut NetObject<SystemType>);
    define_virtual!(pub, get_optional_custom_modifiers, 3, 5, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_required_custom_modifiers, 3, 6, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_value, 3, 7, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>, index : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    //define_virtual!(pub, get_value, 4, 0, *mut NetObject<SystemObject>, obj : *mut NetObject<SystemObject>, invoke_attr : BindingFlags, binder : usize, index : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, culture : usize);
    define_virtual!(pub, get_constant_value, 4, 1, *mut NetObject<SystemObject>);
    define_virtual!(pub, get_raw_constant_value, 4, 2, *mut NetObject<SystemObject>);
    define_virtual!(pub, set_value, 4, 3, (), obj : *mut NetObject<SystemObject>, value : *mut NetObject<SystemObject>, index : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    //define_virtual!(pub, set_value, 4, 4, (), obj : *mut NetObject<SystemObject>, value : *mut NetObject<SystemObject>, invoke_attr : BindingFlags, binder : usize, index : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, culture : usize);
}

pub struct Module { }

impl NetObject<Module> {
    define_virtual!(pub, get_assembly, 0, 4, *mut NetObject<Assembly>);
    define_virtual!(pub, get_fully_qualified_name, 0, 5, *mut NetObject<SystemString>);
    define_virtual!(pub, get_name, 0, 6, *mut NetObject<SystemString>);
    define_virtual!(pub, get_md_stream_version, 0, 7, i32);
    //define_virtual!(pub, get_module_version_id, 1, 0, Guid);
    define_virtual!(pub, get_scope_name, 1, 1, *mut NetObject<SystemString>);
    //define_virtual!(pub, get_module_handle_impl, 1, 2, ModuleHandle);
    //define_virtual!(pub, get_pe_kind, 1, 3, (), pe_kind : *mut NetObject<PortableExecutableKinds&>, machine : *mut NetObject<ImageFileMachine&>);
    define_virtual!(pub, is_resource, 1, 4, bool);
    define_virtual!(pub, is_defined, 1, 5, bool, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    //define_virtual!(pub, get_custom_attributes, 1, 6, *mut NetObject<IEnumerable`1>);
    //define_virtual!(pub, get_custom_attributes_data, 1, 7, *mut NetObject<IList`1>);
    define_virtual!(pub, get_custom_attributes, 2, 0, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, inherit : bool);
    define_virtual!(pub, get_custom_attributes_2, 2, 1, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    //define_virtual!(pub, get_method_impl, 2, 2, *mut NetObject<MethodInfo>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags, binder : *mut NetObject<Binder>, call_convention : CallingConventions, types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, modifiers : *mut NetObject<SystemArray<ParameterModifier>>);
    define_virtual!(pub, get_methods, 2, 3, *mut NetObject<SystemArray<*mut NetObject<MethodInfo>>>, binding_flags : BindingFlags);
    define_virtual!(pub, get_field, 2, 4, *mut NetObject<FieldInfo>, name : *mut NetObject<SystemString>, binding_attr : BindingFlags);
    define_virtual!(pub, get_fields, 2, 5, *mut NetObject<SystemArray<*mut NetObject<FieldInfo>>>, binding_flags : BindingFlags);
    define_virtual!(pub, get_types, 2, 6, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_type_1, 2, 7, *mut NetObject<SystemType>, class_name : *mut NetObject<SystemString>);
    define_virtual!(pub, get_type_2, 3, 0, *mut NetObject<SystemType>, class_name : *mut NetObject<SystemString>, ignore_case : bool);
    define_virtual!(pub, get_type_3, 3, 1, *mut NetObject<SystemType>, class_name : *mut NetObject<SystemString>, throw_on_error : bool, ignore_case : bool);
    //define_virtual!(pub, find_types, 3, 2, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, filter : *mut NetObject<TypeFilter>, filter_criteria : *mut NetObject<SystemObject>);
    define_virtual!(pub, get_metadata_token, 3, 3, i32);
    define_virtual!(pub, resolve_field, 3, 4, *mut NetObject<FieldInfo>, metadata_token : i32, generic_type_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, generic_method_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, resolve_member, 3, 5, *mut NetObject<MemberInfo>, metadata_token : i32, generic_type_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, generic_method_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, resolve_method, 3, 6, *mut NetObject<MethodBase>, metadata_token : i32, generic_type_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, generic_method_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, resolve_signature, 3, 7, *mut NetObject<SystemArray<u8>>, metadata_token : i32);
    define_virtual!(pub, resolve_string, 4, 0, *mut NetObject<SystemString>, metadata_token : i32);
    define_virtual!(pub, resolve_type, 4, 1, *mut NetObject<SystemType>, metadata_token : i32, generic_type_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, generic_method_arguments : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    //define_virtual!(pub, get_object_data, 4, 2, (), info : *mut NetObject<SerializationInfo>, context : StreamingContext);
}

pub struct Assembly { }

impl NetObject<Assembly> {
    //define_virtual!(pub, get_defined_types, 0, 4, *mut NetObject<IEnumerable`1>);
    define_virtual!(pub, get_types, 0, 5, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    //define_virtual!(pub, get_exported_types, 0, 6, *mut NetObject<IEnumerable`1>);
    define_virtual!(pub, get_exported_types, 0, 7, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_forwarded_types, 1, 0, *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub, get_code_base, 1, 1, *mut NetObject<SystemString>);
    define_virtual!(pub, get_entry_point, 1, 2, *mut NetObject<MethodInfo>);
    define_virtual!(pub, get_full_name, 1, 3, *mut NetObject<SystemString>);
    define_virtual!(pub, get_image_runtime_version, 1, 4, *mut NetObject<SystemString>);
    define_virtual!(pub, get_is_dynamic, 1, 5, bool);
    define_virtual!(pub, get_location, 1, 6, *mut NetObject<SystemString>);
    define_virtual!(pub, get_reflection_only, 1, 7, bool);
    define_virtual!(pub, get_is_collectible, 2, 0, bool);
    //define_virtual!(pub, get_manifest_resource_info, 2, 1, *mut NetObject<ManifestResourceInfo>, resource_name : *mut NetObject<SystemString>);
    define_virtual!(pub, get_manifest_resource_names, 2, 2, *mut NetObject<SystemArray<*mut NetObject<SystemString>>>);
    //define_virtual!(pub, get_manifest_resource_stream, 2, 3, *mut NetObject<Stream>, name : *mut NetObject<SystemString>);
    //define_virtual!(pub, get_manifest_resource_stream, 2, 4, *mut NetObject<Stream>, type : *mut NetObject<SystemType>, name : *mut NetObject<SystemString>);
    define_virtual!(pub, get_name, 2, 5, *mut NetObject<AssemblyName>);
    define_virtual!(pub, get_name_2, 2, 6, *mut NetObject<AssemblyName>, copied_name : bool);
    define_virtual!(pub, get_type_1, 2, 7, *mut NetObject<SystemType>, name : *mut NetObject<SystemString>);
    define_virtual!(pub, get_type_2, 3, 0, *mut NetObject<SystemType>, name : *mut NetObject<SystemString>, throw_on_error : bool);
    define_virtual!(pub, get_type_3, 3, 1, *mut NetObject<SystemType>, name : *mut NetObject<SystemString>, throw_on_error : bool, ignore_case : bool);
    define_virtual!(pub, is_defined, 3, 2, bool, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    //define_virtual!(pub, get_custom_attributes, 3, 3, *mut NetObject<IEnumerable`1>);
    //define_virtual!(pub, get_custom_attributes_data, 3, 4, *mut NetObject<IList`1>);
    define_virtual!(pub, get_custom_attributes, 3, 5, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, inherit : bool);
    define_virtual!(pub, get_custom_attributes_2, 3, 6, *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, attribute_type : *mut NetObject<SystemType>, inherit : bool);
    define_virtual!(pub, get_escaped_code_base, 3, 7, *mut NetObject<SystemString>);
    //define_virtual!(pub, create_instance, 4, 0, *mut NetObject<SystemObject>, type_name : *mut NetObject<SystemString>, ignore_case : bool, binding_attr : BindingFlags, binder : *mut NetObject<Binder>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>, culture : *mut NetObject<CultureInfo>, activation_attributes : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    //define_virtual!(pub, add_module_resolve, 4, 1, (), value : *mut NetObject<ModuleResolveEventHandler>);
    //define_virtual!(pub, remove_module_resolve, 4, 2, (), value : *mut NetObject<ModuleResolveEventHandler>);
    define_virtual!(pub, get_manifest_module, 4, 3, *mut NetObject<Module>);
    define_virtual!(pub, get_module, 4, 4, *mut NetObject<Module>, name : *mut NetObject<SystemString>);
    define_virtual!(pub, get_modules, 4, 5, *mut NetObject<SystemArray<*mut NetObject<Module>>>, get_resource_modules : bool);
    //define_virtual!(pub, get_modules, 4, 6, *mut NetObject<IEnumerable`1>);
    define_virtual!(pub, get_loaded_modules, 4, 7, *mut NetObject<SystemArray<*mut NetObject<Module>>>, get_resource_modules : bool);
    //define_virtual!(pub, get_referenced_assemblies, 5, 0, *mut NetObject<SystemArray<*mut NetObject<AssemblyName>>>);
    //define_virtual!(pub, get_satellite_assembly, 5, 1, *mut NetObject<Assembly>, culture : *mut NetObject<CultureInfo>);
    //define_virtual!(pub, get_satellite_assembly, 5, 2, *mut NetObject<Assembly>, culture : *mut NetObject<CultureInfo>, version : *mut NetObject<Version>);
    //define_virtual!(pub, get_file, 5, 3, *mut NetObject<FileStream>, name : *mut NetObject<SystemString>);
    //define_virtual!(pub, get_files, 5, 4, *mut NetObject<SystemArray<*mut NetObject<FileStream>>>);
    //define_virtual!(pub, get_files, 5, 5, *mut NetObject<SystemArray<*mut NetObject<FileStream>>>, get_resource_modules : bool);
    //define_virtual!(pub, get_object_data, 5, 6, (), info : *mut NetObject<SerializationInfo>, context : StreamingContext);
    define_virtual!(pub, get_global_assembly_cache, 5, 7, bool);
    define_virtual!(pub, get_host_context, 6, 0, i64);
    define_virtual!(pub, load_module, 6, 1, *mut NetObject<Module>, module_name : *mut NetObject<SystemString>, raw_module : *mut NetObject<SystemArray<u8>>, raw_symbol_store : *mut NetObject<SystemArray<u8>>);
    //define_virtual!(pub, get_security_rule_set, 6, 2, SecurityRuleSet);
}

pub struct AssemblyName {

}