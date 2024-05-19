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
macro_rules! define_typeof {
    ($structure:ty, $name:literal) => {
        impl $crate::wrappers::system::TypeInfoProvider for $structure {
            fn type_of() -> *mut $crate::wrappers::system::NetObject<$crate::wrappers::system::SystemType> {
                static MY_TYPE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                *MY_TYPE.get_or_init(|| $crate::search_type_cached(&String::from($name), true) as usize) as *mut $crate::wrappers::system::NetObject<$crate::wrappers::system::SystemType>
            }
        }
    };
}

// WHY RUST WHYYYYYYYYYYYYYY

#[macro_export]
macro_rules! managed_array {
    ($tape:ty, $size:literal $(, $pn:expr )*) => {
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