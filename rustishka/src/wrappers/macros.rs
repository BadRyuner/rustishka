#[macro_export]
macro_rules! define_virtual {
    ($func_name:ident, $row:expr, $index:expr, $rett:ty $(, $pn:ident : $pt:ty)*) => {
        fn $func_name (self: *mut Self, $( $pn: $pt, )* ) -> $rett {
            unsafe {
                let obj = self as *mut NetObject<SystemObject>;
                let func_ptr = obj.get_method_table().get_func_at($row, $index);
                let func: extern "stdcall" fn(*mut NetObject<SystemObject>, $( $pt, )*) -> $rett = core::mem::transmute(func_ptr);
                func(obj, $($pn),* )
            }
        }
    };

    (pub, $func_name:ident, $row:expr, $index:expr, $rett:ty $(, $pn:ident : $pt:ty)*) => {
        pub fn $func_name (self: *mut Self, $( $pn: $pt, )* ) -> $rett {
            unsafe {
                let obj = self as *mut NetObject<SystemObject>;
                let func_ptr = obj.get_method_table().get_func_at($row, $index);
                let func: extern "stdcall" fn(*mut NetObject<SystemObject>, $( $pt, )*) -> $rett = core::mem::transmute(func_ptr);
                func(obj, $($pn),* )
            }
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