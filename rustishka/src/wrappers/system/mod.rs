use self::system_string::SystemString;

pub mod system_string;
pub mod system_type;

#[repr(C)]
pub struct NetObject<Content> {
    pub method_table: *mut MethodTable,
    pub content: Content
}

impl<Content> NetObject<Content> {
    pub fn get_method_table(self: *mut Self) -> *mut MethodTable {
        unsafe {
            (*self).method_table
        }
    }

    pub fn get_content(self: *mut Self) -> &'static mut Content {
        unsafe {
            &mut (*self).content
        }
    }

    pub fn cast_unchecked<T>(self: *mut Self) -> *mut NetObject<T> {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

pub struct SystemObject { }

pub trait SystemObjectBindings {
    fn to_system_string(self: *mut Self) -> *mut NetObject<SystemString> {
        unsafe {
            let obj = self as *mut NetObject<SystemObject>;
            let func_ptr = obj.get_method_table().get_func_at(0, 1);
            let func: extern "stdcall" fn(*mut NetObject<SystemObject>) -> *mut NetObject<SystemString> = core::mem::transmute(func_ptr);
            func(obj)
        }
    }

    fn equals<T>(self: *mut Self, other: *mut NetObject<T>) -> bool {
        unsafe {
            let obj = self as *mut NetObject<SystemObject>;
            let func_ptr = obj.get_method_table().get_func_at(0, 2);
            let func: extern "stdcall" fn(*mut NetObject<SystemObject>, *mut NetObject<T>) -> bool = core::mem::transmute(func_ptr);
            func(obj, other)
        }
    }

    fn get_hashcode(self: *mut Self) -> i32 {
        unsafe {
            let obj = self as *mut NetObject<SystemObject>;
            let func_ptr = obj.get_method_table().get_func_at(0, 3);
            let func: extern "stdcall" fn(*mut NetObject<SystemObject>) -> i32 = core::mem::transmute(func_ptr);
            func(obj)
        }
    }
}


impl SystemObjectBindings for NetObject<SystemObject> {}

#[repr(C)]
pub struct MethodTable {
    some_shit: u32, // 00
    pub base_size: u32, // 04
    bruh_flags: u16, // 08
    pub metadata_token: u16, // 10
    pub num_virtuals: u16, // 12
    pub num_interfaces: u16, // 14
    pub parent: *mut MethodTable, // 16
    loader_module: usize, // 24
    pub writeable_data: *mut u8, // 32
    pub ee_class: *mut usize, // 40
    per_inst_info: *mut usize, // 48
    pub interface_map: *mut MethodTable, // 56
    pub methods: *mut *mut usize // 64
}

impl MethodTable {
    pub fn get_func_at(self: *mut Self, row: usize, index: usize) -> *mut usize {
        unsafe { 
            // very unsafe
            // inlines and compiles as: (ex. for x86-64)
            // mov rax, qword [rcx] // deref object pointer to 
            // mov rax, qword [rax + 0x40 + row * 8] // get methods chunk. [precompued row * i & inlined as number]
            // call qword ptr [rax + index * 8] // [precompued index * i & inlined as number]
            let chunk = (core::ptr::from_mut(&mut (*self).methods)).byte_add(row * core::mem::size_of::<usize>()).read();
            let entry = chunk.byte_add(index * core::mem::size_of::<usize>()).read();
            entry
        }
    }
}