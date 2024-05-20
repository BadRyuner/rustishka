#[macro_export]
macro_rules! define_virtual {
    ($vi:vis $func_name:ident, $row:expr, $index:expr, $rett:ty $(, $pn:ident : $pt:ty)*) => {
        $vi fn $func_name (self: *mut Self, $( $pn: $pt, )* ) -> $rett {
            unsafe {
                let obj = self as *mut $crate::wrappers::system::NetObject<$crate::wrappers::system::SystemObject>;
                let func_ptr = obj.get_method_table().get_func_at($row, $index);
                let func: extern "stdcall" fn(*mut $crate::wrappers::system::NetObject<$crate::wrappers::system::SystemObject>, $( $pt, )*) -> $rett = core::mem::transmute(func_ptr);
                func(obj, $($pn),* )
            }
        }
    };
}

#[macro_export]
macro_rules! define_function {
    ($vi:vis $func_name:ident, $slot:expr, $rett:ty $(, $pn:ident : $pt:ty)*) => {
        $vi fn $func_name ($( $pn: $pt, )* ) -> $rett {
            static MY_FUNC: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
            let pf = *MY_FUNC.get_or_init(|| {
                let me = <Self as $crate::wrappers::system::TypeInfoProvider>::type_of();
                unsafe { core::mem::transmute(me.get_fn_at_slot($slot)) }
            } );
            let f: extern "stdcall" fn($( $pt, )*) -> $rett = unsafe { core::mem::transmute(pf) };
            f($($pn),*)
        }
    };
}

#[macro_export]
macro_rules! define_constructor {
    ($vi:vis $func_name:ident $(, $pn:ident : $pt:ty)*) => {
        pub fn $func_name ($( $pn: $pt, )* ) -> *mut $crate::wrappers::system::NetObject<Self> {
            let ___args = $crate::wrappers::system::system_array::SystemArray::create_object_array(
                &[ $($pn._box_value() as *mut $crate::wrappers::system::NetObject<$crate::wrappers::system::SystemObject>),* ]
            );
            $crate::wrappers::system::system_activator::Activator::create_instance_1(<Self as $crate::wrappers::system::TypeInfoProvider>::type_of(), ___args) as *mut $crate::wrappers::system::NetObject<Self>
        }
    };
}

#[macro_export]
macro_rules! define_static_field {
    ($vi:vis $func_name:ident, $field_name:literal, $rett:path) => {
        pub fn $func_name() -> $rett {
            let my_type = Self::type_of();
            let field = my_type.get_field(allocate_string(&String::from($field_name)), BindingFlags::PublicStatic);
            field.get_value(0 as _) as $rett
        }
    };
    ($vi:vis $func_name:ident, $field_name:literal, $rett:path, unbox) => {
        pub fn $func_name() -> $rett {
            let my_type = Self::type_of();
            let field = my_type.get_field(allocate_string(&String::from($field_name)), BindingFlags::PublicStatic);
            let val = field.get_value(0 as _) as *mut NetObject<$rett>;
            *val.get_content()
        }
    };
}

#[macro_export]
macro_rules! define_typeof {
    ($structure:ty, $name:literal) => {
        impl $crate::wrappers::system::TypeInfoProvider for $structure {
            fn type_of() -> *mut $crate::wrappers::system::NetObject<$crate::wrappers::system::system_reflection::SystemType> {
                static MY_TYPE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                *MY_TYPE.get_or_init(|| $crate::search_type_cached(&String::from($name), true) as usize) as *mut $crate::wrappers::system::NetObject<$crate::wrappers::system::system_reflection::SystemType>
            }
        }
    };
}

#[macro_export]
macro_rules! resolve_interface_method {
    ($self:expr, $_self:tt, $func_id:literal, $args_count:literal $(, $pt:ty)*) => {
        {
            let ___args_sig = managed_array!($crate::wrappers::system::system_reflection::SystemType, 1 + $args_count, <$_self as $crate::wrappers::system::TypeInfoProvider>::type_of() $(, <$pt as $crate::wrappers::system::TypeInfoProvider>::type_of())*);
            let obj_type = $self.get_type();
            let interface = $_self::type_of();
            let target = interface.get_methods($crate::wrappers::system::system_reflection::BindingFlags::PublicInstance).as_slice()[$func_id];
            let params = target.get_parameters();
            let params_slice = params.as_slice();
            let mapped_params = managed_array!(SystemType, params_slice.len() as i32);
            let mapped_params_slice = mapped_params.as_slice();
            for (index, t) in params_slice.iter().enumerate() {
                mapped_params_slice[index] = t.get_parameter_type();
            }
            obj_type.get_method_3($crate::wrappers::system::system_reflection::MemberInfoBindings::get_name(target), mapped_params)
        }
    };
}

// WHY RUST WHYYYYYYYYYYYYYY

#[macro_export]
macro_rules! managed_array {
    ($tape:ty, $size:expr) => {
        {
            let pypy = <$tape as $crate::wrappers::system::TypeInfoProvider>::type_of();
            $crate::allocate_array(pypy, $size)
        }
    };
    ($tape:ty, $size:expr $(, $pn:expr )*) => {
        {
            let pypy = <$tape as $crate::wrappers::system::TypeInfoProvider>::type_of();
            let arr = $crate::allocate_array(pypy, $size);
            let bro: isize = -1;
            let abstraction = arr.as_slice();
            $(
                let bro = (bro + 1) as usize;
                abstraction[bro] = $pn; 
            )*
            arr
        }
    };
}

#[macro_export]
macro_rules! throw {
    ($err:expr) => {
        unsafe {
            DOTNET_RUNTIME.get_mut().unwrap().throw($err);
        }
    };
}

#[macro_export]
macro_rules! initialize_rustishka {
    () => {
        #[no_mangle]
        extern "stdcall" fn rustishka_handshake(imports: *mut $crate::imports::DotnetImports) {
            let ctx = $crate::imports::DotnetImportsContainer::new(imports);
            unsafe {
                $crate::DOTNET_RUNTIME.set($crate::imports::DotnetImportsContainer::new(imports)).unwrap();
            }
        }
    };
}