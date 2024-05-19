use crate::{define_function, define_typeof, wrappers::system::{system_array::SystemArray, system_string::SystemString, NetObject, SystemObject}};

pub struct Console { }

define_typeof!(Console, "System.Console");

impl Console {
    define_function!(pub get_cursor_left, 47, i32);
    define_function!(pub set_cursor_left, 48, (), value : i32);
    define_function!(pub get_cursor_top, 49, i32);
    define_function!(pub set_cursor_top, 50, (), value : i32);
    define_function!(pub get_title, 52, *mut NetObject<SystemString>);
    define_function!(pub set_title, 53, (), value : *mut NetObject<SystemString>);
    define_function!(pub beep_1, 54, ());
    define_function!(pub beep_2, 55, (), frequency : i32, duration : i32);
    define_function!(pub move_buffer_area, 56, (), source_left : i32, source_top : i32, source_width : i32, source_height : i32, target_left : i32, target_top : i32);
    //define_function!(pub move_buffer_area_2, 57, (), source_left : i32, source_top : i32, source_width : i32, source_height : i32, target_left : i32, target_top : i32, source_char : u16, source_fore_color : ConsoleColor, source_back_color : ConsoleColor);
    define_function!(pub clear, 58, ());
    define_function!(pub set_cursor_position, 59, (), left : i32, top : i32);
    //define_function!(pub add_cancel_key_press, 60, (), value : *mut NetObject<ConsoleCancelEventHandler>);
    //define_function!(pub remove_cancel_key_press, 61, (), value : *mut NetObject<ConsoleCancelEventHandler>);
    define_function!(pub get_treat_control_c_as_input, 62, bool);
    define_function!(pub set_treat_control_c_as_input, 63, (), value : bool);
    //define_function!(pub open_standard_input, 64, *mut NetObject<Stream>);
    //define_function!(pub open_standard_input, 65, *mut NetObject<Stream>, buffer_size : i32);
    //define_function!(pub open_standard_output, 66, *mut NetObject<Stream>);
    //define_function!(pub open_standard_output, 67, *mut NetObject<Stream>, buffer_size : i32);
    //define_function!(pub open_standard_error, 68, *mut NetObject<Stream>);
    //define_function!(pub open_standard_error, 69, *mut NetObject<Stream>, buffer_size : i32);
    //define_function!(pub set_in, 70, (), new_in : *mut NetObject<TextReader>);
    //define_function!(pub set_out, 71, (), new_out : *mut NetObject<TextWriter>);
    //define_function!(pub set_error, 72, (), new_error : *mut NetObject<TextWriter>);
    define_function!(pub read, 73, i32);
    define_function!(pub read_line, 74, *mut NetObject<SystemString>);
    define_function!(pub write_line_1, 75, ());
    define_function!(pub write_line_2, 76, (), value : bool);
    define_function!(pub write_line_3, 77, (), value : u16);
    define_function!(pub write_line_4, 78, (), buffer : *mut NetObject<SystemArray<u16>>);
    define_function!(pub write_line_5, 79, (), buffer : *mut NetObject<SystemArray<u16>>, index : i32, count : i32);
    define_function!(pub write_line_6, 81, (), value : f64);
    define_function!(pub write_line_7, 82, (), value : f32);
    define_function!(pub write_line_8, 83, (), value : i32);
    define_function!(pub write_line_9, 84, (), value : u32);
    define_function!(pub write_line_10, 85, (), value : i64);
    define_function!(pub write_line_11, 86, (), value : u64);
    define_function!(pub write_line_12, 87, (), value : *mut NetObject<SystemObject>);
    define_function!(pub write_line_13, 88, (), value : *mut NetObject<SystemString>);
    define_function!(pub write_line_14, 89, (), format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>);
    define_function!(pub write_line_15, 90, (), format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>);
    define_function!(pub write_line_16, 91, (), format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>, arg2 : *mut NetObject<SystemObject>);
    define_function!(pub write_line_17, 92, (), format : *mut NetObject<SystemString>, arg : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub write_1, 93, (), format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>);
    define_function!(pub write_2, 94, (), format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>);
    define_function!(pub write_3, 95, (), format : *mut NetObject<SystemString>, arg0 : *mut NetObject<SystemObject>, arg1 : *mut NetObject<SystemObject>, arg2 : *mut NetObject<SystemObject>);
    define_function!(pub write_4, 96, (), format : *mut NetObject<SystemString>, arg : *mut NetObject<SystemArray<*mut NetObject<SystemObject>>>);
    define_function!(pub write_5, 97, (), value : bool);
    define_function!(pub write_6, 98, (), value : u16);
    define_function!(pub write_7, 99, (), buffer : *mut NetObject<SystemArray<u16>>);
    define_function!(pub write_8, 100, (), buffer : *mut NetObject<SystemArray<u16>>, index : i32, count : i32);
    define_function!(pub write_9, 101, (), value : f64);
    define_function!(pub write_10, 103, (), value : f32);
    define_function!(pub write_11, 104, (), value : i32);
    define_function!(pub write_12, 105, (), value : u32);
    define_function!(pub write_13, 106, (), value : i64);
    define_function!(pub write_14, 107, (), value : u64);
    define_function!(pub write_15, 108, (), value : *mut NetObject<SystemObject>);
    define_function!(pub write_16, 109, (), value : *mut NetObject<SystemString>);
}