use std::ptr;

use crate::{define_constructor, define_function, define_typeof, define_virtual, imports::DotnetImportsContainer};

use super::{system_array::SystemArray, NetObject, SystemObject, SystemObjectBindings};

#[repr(C)]
pub struct SystemString {
    pub length: u32,
    pub chars: [u16; 512]
}

define_typeof!(SystemString, "System.String");

impl SystemObjectBindings for NetObject<SystemString> {}

impl SystemString {
    pub unsafe fn alloc_on_rust(text: &str) -> Box<NetObject<SystemString>> {
        let mut pseudo_string = NetObject::<SystemString> { 
            method_table: ptr::null_mut(), // VERY FUCKING UNSAFE
            content: SystemString {
                length: text.chars().count() as u32,
                chars: [0; 512]
            }
        };

        for (i, c) in text.chars().enumerate() {
            if i > 511 {
                break;
            }
            pseudo_string.content.chars[i] = c as u16;
        }

        Box::new(pseudo_string)
    }

    pub fn alloc_on_dotnet(ctx: &DotnetImportsContainer, text: &String) -> *mut NetObject<SystemString> {
        ctx.allocate_string(text)
    }
}

impl NetObject<SystemString> {
    pub fn as_slice(self: *mut Self) -> &'static mut [u16] {
        unsafe {
            core::slice::from_raw_parts_mut(core::ptr::from_mut(&mut (*self).content.chars[0]), (*self).content.length as usize)
        }
    }
}

use super::AutoStructBox;

impl SystemString {
    define_constructor!(pub new, value : *mut NetObject<SystemArray<u16>>);
    define_constructor!(pub new_1, value : *mut NetObject<SystemArray<u16>>, start_index : i32, length : i32);
    //define_constructor!(pub new_2, value : Ptr<u16>);
    //define_constructor!(pub new_3, value : Ptr<u16>, start_index : i32, length : i32);
    //define_constructor!(pub new_4, value : Ptr<i8>);
    //define_constructor!(pub new_5, value : Ptr<i8>, start_index : i32, length : i32);
    //define_constructor!(pub new_6, value : *mut i8, start_index : i32, length : i32, enc : *mut NetObject<Encoding>);
    define_constructor!(pub new_7, c : u16, count : i32);
    //define_constructor!(pub new_8, value : ReadOnlySpan`1);
}

impl NetObject<SystemString> {
    // Virtual functions
    define_virtual!(pub compare_to_1, 0, 4, i32, value : *mut NetObject<SystemObject>);
    define_virtual!(pub compare_to_2, 0, 5, i32, str_b : *mut NetObject<SystemString>);
    define_virtual!(pub equals, 0, 6, bool, value : *mut NetObject<SystemString>);
    define_virtual!(pub clone, 0, 7, *mut NetObject<SystemObject>);
    
    // Non-Virtual functions
    define_function!(pub fast_allocate_string, 36, *mut NetObject<SystemString>, length : i32);
    define_function!(pub set_trail_byte, 37, (), self: *mut Self, data : u8);
    define_function!(pub try_get_trail_byte, 38, bool, self: *mut Self, data : *mut u8);
    define_function!(pub intern_1, 39, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub intern_2, 41, *mut NetObject<SystemString>, str : *mut NetObject<SystemString>);
    define_function!(pub internal_copy, 43, (), src : *mut NetObject<SystemString>, dest : isize, len : i32);
    define_function!(pub equals_helper, 45, bool, str_a : *mut NetObject<SystemString>, str_b : *mut NetObject<SystemString>);
     define_function!(pub equals_ordinal_ignore_case, 47, bool, str_a : *mut NetObject<SystemString>, str_b : *mut NetObject<SystemString>);
    define_function!(pub equals_ordinal_ignore_case_no_length_check, 48, bool, str_a : *mut NetObject<SystemString>, str_b : *mut NetObject<SystemString>);
    define_function!(pub compare_1, 50, i32, str_a : *mut NetObject<SystemString>, str_b : *mut NetObject<SystemString>);
    define_function!(pub compare_2, 51, i32, str_a : *mut NetObject<SystemString>, str_b : *mut NetObject<SystemString>, ignore_case : bool);
    define_function!(pub compare_3, 55, i32, str_a : *mut NetObject<SystemString>, index_a : i32, str_b : *mut NetObject<SystemString>, index_b : i32, length : i32);
    define_function!(pub compare_4, 56, i32, str_a : *mut NetObject<SystemString>, index_a : i32, str_b : *mut NetObject<SystemString>, index_b : i32, length : i32, ignore_case : bool);
    define_function!(pub compare_ordinal_1, 60, i32, str_a : *mut NetObject<SystemString>, str_b : *mut NetObject<SystemString>);
    define_function!(pub compare_ordinal_2, 62, i32, str_a : *mut NetObject<SystemString>, index_a : i32, str_b : *mut NetObject<SystemString>, index_b : i32, length : i32);
    define_function!(pub ends_with_1, 63, bool, self: *mut Self, value : *mut NetObject<SystemString>);
    define_function!(pub ends_with_2, 66, bool, self: *mut Self, value : u16);
    define_function!(pub equals_2, 68, bool, a : *mut NetObject<SystemString>, b : *mut NetObject<SystemString>);
    define_function!(pub op_equality, 70, bool, a : *mut NetObject<SystemString>, b : *mut NetObject<SystemString>);
    define_function!(pub op_inequality, 71, bool, a : *mut NetObject<SystemString>, b : *mut NetObject<SystemString>);
    define_function!(pub get_hash_code_ordinal_ignore_case, 73, i32, self: *mut Self);
    define_function!(pub get_non_randomized_hash_code, 77, i32, self: *mut Self);
    define_function!(pub get_non_randomized_hash_code_ordinal_ignore_case, 78, i32, self: *mut Self);
    define_function!(pub starts_with_1, 79, bool, self: *mut Self, value : *mut NetObject<SystemString>);
    define_function!(pub starts_with_2, 82, bool, self: *mut Self, value : u16);
    define_function!(pub copy, 109, *mut NetObject<SystemString>, str : *mut NetObject<SystemString>);
    define_function!(pub copy_to, 110, (), self: *mut Self, source_index : i32, destination : *mut NetObject<SystemArray<u16>>, destination_index : i32, count : i32);
    define_function!(pub to_char_array_1, 113, *mut NetObject<SystemArray<u16>>, self: *mut Self);
    define_function!(pub to_char_array_2, 114, *mut NetObject<SystemArray<u16>>, self: *mut Self, start_index : i32, length : i32);
    define_function!(pub is_null_or_empty, 115, bool, value : *mut NetObject<SystemString>);
    define_function!(pub is_null_or_white_space, 116, bool, value : *mut NetObject<SystemString>);
    define_function!(pub get_pinnable_reference, 117, *mut u16, self: *mut Self);
    define_function!(pub get_raw_string_data, 118, *mut u16, self: *mut Self);
    define_function!(pub get_raw_string_data_as_u_int16, 119, *mut u16, self: *mut Self);
    define_function!(pub create_from_char, 121, *mut NetObject<SystemString>, c : u16);
    define_function!(pub wcslen, 125, i32, ptr : *mut u16);
    define_function!(pub strlen, 126, i32, ptr : *mut u8);
    define_function!(pub is_normalized, 127, bool, self: *mut Self);
    define_function!(pub normalize, 129, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub get_length, 132, i32, self: *mut Self);
    define_function!(pub copy_string_content, 137, (), dest : *mut NetObject<SystemString>, dest_pos : i32, src : *mut NetObject<SystemString>);
    define_function!(pub concat_1, 138, *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>);
    define_function!(pub concat_2, 139, *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>);
    define_function!(pub concat_3, 140, *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>, arg2 : *mut NetObject<SystemObject>);
    define_function!(pub concat_4, 141, *mut NetObject<SystemString>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub concat_5, 143, *mut NetObject<SystemString>, str0 : *mut NetObject<SystemString>, str1 : *mut NetObject<SystemString>);
    define_function!(pub concat_6, 144, *mut NetObject<SystemString>, str0 : *mut NetObject<SystemString>, str1 : *mut NetObject<SystemString>, str2 : *mut NetObject<SystemString>);
    define_function!(pub concat_7, 145, *mut NetObject<SystemString>, str0 : *mut NetObject<SystemString>, str1 : *mut NetObject<SystemString>, str2 : *mut NetObject<SystemString>, str3 : *mut NetObject<SystemString>);
    define_function!(pub concat_8, 150, *mut NetObject<SystemString>, values : *mut NetObject<SystemArray<*mut NetObject<SystemString>>>);
    define_function!(pub format_1, 151, *mut NetObject<SystemString>, format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>);
    define_function!(pub format_2, 152, *mut NetObject<SystemString>, format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>);
    define_function!(pub format_3, 153, *mut NetObject<SystemString>, format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>, arg2 : *mut NetObject<SystemObject>);
    define_function!(pub format_4, 154, *mut NetObject<SystemString>, format : *mut NetObject<SystemString>, args : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub insert, 162, *mut NetObject<SystemString>, self: *mut Self, start_index : i32, value : *mut NetObject<SystemString>);
    define_function!(pub join_1, 163, *mut NetObject<SystemString>, separator : u16, value : *mut NetObject<SystemArray<*mut NetObject<SystemString>>>);
    define_function!(pub join_2, 164, *mut NetObject<SystemString>, separator : *mut NetObject<SystemString>, value : *mut NetObject<SystemArray<*mut NetObject<SystemString>>>);
    define_function!(pub join_3, 165, *mut NetObject<SystemString>, separator : u16, value : *mut NetObject<SystemArray<*mut NetObject<SystemString>>>, start_index : i32, count : i32);
    define_function!(pub join_4, 166, *mut NetObject<SystemString>, separator : *mut NetObject<SystemString>, value : *mut NetObject<SystemArray<*mut NetObject<SystemString>>>, start_index : i32, count : i32);
    define_function!(pub join_5, 169, *mut NetObject<SystemString>, separator : u16, values : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub join_6, 170, *mut NetObject<SystemString>, separator : *mut NetObject<SystemString>, values : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub pad_left_1, 173, *mut NetObject<SystemString>, self: *mut Self, total_width : i32);
    define_function!(pub pad_left_2, 174, *mut NetObject<SystemString>, self: *mut Self, total_width : i32, padding_char : u16);
    define_function!(pub pad_right_1, 175, *mut NetObject<SystemString>, self: *mut Self, total_width : i32);
    define_function!(pub pad_right_2, 176, *mut NetObject<SystemString>, self: *mut Self, total_width : i32, padding_char : u16);
    define_function!(pub remove_1, 177, *mut NetObject<SystemString>, self: *mut Self, start_index : i32, count : i32);
    define_function!(pub remove_2, 178, *mut NetObject<SystemString>, self: *mut Self, start_index : i32);
    define_function!(pub replace_1, 183, *mut NetObject<SystemString>, self: *mut Self, old_char : u16, new_char : u16);
    define_function!(pub replace_2, 184, *mut NetObject<SystemString>, self: *mut Self, old_value : *mut NetObject<SystemString>, new_value : *mut NetObject<SystemString>);
    define_function!(pub split_1, 193, *mut NetObject<SystemArray<*mut NetObject<SystemString>>>, self: *mut Self, separator : *mut NetObject<SystemArray<u16>>);
    define_function!(pub split_2, 194, *mut NetObject<SystemArray<*mut NetObject<SystemString>>>, self: *mut Self, separator : *mut NetObject<SystemArray<u16>>, count : i32);
    define_function!(pub substring_1, 212, *mut NetObject<SystemString>, self: *mut Self, start_index : i32);
    define_function!(pub substring_2, 213, *mut NetObject<SystemString>, self: *mut Self, start_index : i32, length : i32);
    define_function!(pub to_lower, 216, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub to_lower_invariant, 218, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub to_upper, 219, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub to_upper_invariant, 221, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub trim_1, 222, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub trim_2, 223, *mut NetObject<SystemString>, self: *mut Self, trim_char : u16);
    define_function!(pub trim_3, 224, *mut NetObject<SystemString>, self: *mut Self, trim_chars : *mut NetObject<SystemArray<u16>>);
    define_function!(pub trim_start_1, 225, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub trim_start_2, 226, *mut NetObject<SystemString>, self: *mut Self, trim_char : u16);
    define_function!(pub trim_start_3, 227, *mut NetObject<SystemString>, self: *mut Self, trim_chars : *mut NetObject<SystemArray<u16>>);
    define_function!(pub trim_end_1, 228, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub trim_end_2, 229, *mut NetObject<SystemString>, self: *mut Self, trim_char : u16);
    define_function!(pub trim_end_3, 230, *mut NetObject<SystemString>, self: *mut Self, trim_chars : *mut NetObject<SystemArray<u16>>);
    define_function!(pub create_trimmed_string, 233, *mut NetObject<SystemString>, self: *mut Self, start : i32, end : i32);
    define_function!(pub contains_1, 234, bool, self: *mut Self, value : *mut NetObject<SystemString>);
    define_function!(pub contains_2, 236, bool, self: *mut Self, value : u16);
    define_function!(pub index_of_1, 238, i32, self: *mut Self, value : u16);
    define_function!(pub index_of_2, 239, i32, self: *mut Self, value : u16, start_index : i32);
    define_function!(pub index_of_char_ordinal_ignore_case, 241, i32, self: *mut Self, value : u16);
    define_function!(pub index_of_3, 242, i32, self: *mut Self, value : u16, start_index : i32, count : i32);
    define_function!(pub index_of_any_1, 243, i32, self: *mut Self, any_of : *mut NetObject<SystemArray<u16>>);
    define_function!(pub index_of_any_2, 244, i32, self: *mut Self, any_of : *mut NetObject<SystemArray<u16>>, start_index : i32);
    define_function!(pub index_of_any_3, 245, i32, self: *mut Self, any_of : *mut NetObject<SystemArray<u16>>, start_index : i32, count : i32);
    define_function!(pub index_of_4, 246, i32, self: *mut Self, value : *mut NetObject<SystemString>);
    define_function!(pub index_of_5, 247, i32, self: *mut Self, value : *mut NetObject<SystemString>, start_index : i32);
    define_function!(pub index_of_6, 248, i32, self: *mut Self, value : *mut NetObject<SystemString>, start_index : i32, count : i32);
    define_function!(pub last_index_of_1, 252, i32, self: *mut Self, value : u16);
    define_function!(pub last_index_of_2, 253, i32, self: *mut Self, value : u16, start_index : i32);
    define_function!(pub last_index_of_3, 254, i32, self: *mut Self, value : u16, start_index : i32, count : i32);
    define_function!(pub last_index_of_any_1, 255, i32, self: *mut Self, any_of : *mut NetObject<SystemArray<u16>>);
    define_function!(pub last_index_of_any_2, 256, i32, self: *mut Self, any_of : *mut NetObject<SystemArray<u16>>, start_index : i32);
    define_function!(pub last_index_of_any_3, 257, i32, self: *mut Self, any_of : *mut NetObject<SystemArray<u16>>, start_index : i32, count : i32);
    define_function!(pub last_index_of_4, 258, i32, self: *mut Self, value : *mut NetObject<SystemString>);
    define_function!(pub last_index_of_5, 259, i32, self: *mut Self, value : *mut NetObject<SystemString>, start_index : i32);
    define_function!(pub last_index_of_6, 260, i32, self: *mut Self, value : *mut NetObject<SystemString>, start_index : i32, count : i32);
}