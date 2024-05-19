use std::marker::PhantomData;

use crate::{define_constructor, define_function, define_virtual, managed_array, search_type_cached, wrappers::system::{system_delegate::Action1, AutoStructBox, NetObject}};

use super::{system_array::SystemArray, system_reflection::SystemType, TypeInfoProvider};

pub struct List<T: TypeInfoProvider> {
    a: PhantomData<T>
}

pub fn generic_type_of() -> *mut NetObject<SystemType> {
    static MY_TYPE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *MY_TYPE.get_or_init(|| search_type_cached(&String::from("System.Collections.Generic.List`1"), true) as usize) as *mut NetObject<SystemType>
}

impl<T: TypeInfoProvider> TypeInfoProvider for List<T> {
    fn type_of() -> *mut NetObject<SystemType> {
        static MY_TYPE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| generic_type_of().make_generic_type(managed_array!(SystemType, 1, T::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<T: TypeInfoProvider> List<T> {
    define_constructor!(pub new);
    define_constructor!(pub new_1, capacity : i32);
    //define_constructor!(pub new_2, collection : *mut NetObject<IEnumerable`1>);
}

impl<T: TypeInfoProvider> NetObject<List<T>> {
    // Virtual functions
    define_virtual!(pub get_count, 0, 4, i32);
    define_virtual!(pub get_item, 1, 2, T, index : i32);
    define_virtual!(pub set_item, 1, 3, (), index : i32, value : T);
    define_virtual!(pub add_item, 1, 6, (), item : T);
    define_virtual!(pub clear, 2, 0, ());
    define_virtual!(pub contains, 2, 1, bool, item : T);
    define_virtual!(pub copy_to, 2, 4, (), array : *mut NetObject<SystemArray<T>>, array_index : i32);
    define_virtual!(pub index_of, 2, 7, i32, item : T);
    define_virtual!(pub insert, 3, 1, (), index : i32, item : T);
    define_virtual!(pub remove, 3, 3, bool, item : T);
    define_virtual!(pub remove_at, 3, 5, (), index : i32);

    // Non-Virtual functions
    define_function!(pub get_capacity, 34, i32, self: *mut Self);
    define_function!(pub set_capacity, 35, (), self: *mut Self, value : i32);
    //define_function!(pub add_range, 38, (), self: *mut Self, collection : *mut NetObject<IEnumerable<T>>);
    //define_function!(pub as_read_only, 39, *mut NetObject<ReadOnlyCollection<T>>, self: *mut Self);
    //define_function!(pub binary_search, 40, i32, self: *mut Self, index : i32, count : i32, item : T, comparer : *mut NetObject<IComparer<T>>);
    define_function!(pub binary_search, 41, i32, self: *mut Self, item : T);
    //define_function!(pub binary_search, 42, i32, self: *mut Self, item : T, comparer : *mut NetObject<IComparer<T>>);
    //define_function!(pub convert_all, 43, *mut NetObject<List<T>>, self: *mut Self, converter : *mut NetObject<Converter<T>>);
    define_function!(pub copy_to_1, 44, (), self: *mut Self, array : *mut NetObject<SystemArray<T>>);
    define_function!(pub copy_to_2, 45, (), self: *mut Self, index : i32, array : *mut NetObject<SystemArray<T>>, array_index : i32, count : i32);
    define_function!(pub ensure_capacity, 46, i32, self: *mut Self, capacity : i32);
    //define_function!(pub exists, 48, bool, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find, 49, T, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_all, 50, *mut NetObject<List<T>>, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_index, 51, i32, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_index, 52, i32, self: *mut Self, start_index : i32, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_index, 53, i32, self: *mut Self, start_index : i32, count : i32, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_last, 54, T, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_last_index, 55, i32, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_last_index, 56, i32, self: *mut Self, start_index : i32, match : *mut NetObject<Predicate<T>>);
    //define_function!(pub find_last_index, 57, i32, self: *mut Self, start_index : i32, count : i32, match : *mut NetObject<Predicate<T>>);
    define_function!(pub for_each, 58, (), self: *mut Self, action : *mut NetObject<Action1<T>>);
    //define_function!(pub get_enumerator, 59, Enumerator, self: *mut Self);
    define_function!(pub get_range, 60, *mut NetObject<List<T>>, self: *mut Self, index : i32, count : i32);
    define_function!(pub slice, 61, *mut NetObject<List<T>>, self: *mut Self, start : i32, length : i32);
    define_function!(pub index_of_1, 62, i32, self: *mut Self, item : T, index : i32);
    define_function!(pub index_of_2, 63, i32, self: *mut Self, item : T, index : i32, count : i32);
    //define_function!(pub insert_range, 64, (), self: *mut Self, index : i32, collection : *mut NetObject<IEnumerable<T>>);
    define_function!(pub last_index_of, 65, i32, self: *mut Self, item : T);
    define_function!(pub last_index_of_from, 66, i32, self: *mut Self, item : T, index : i32);
    define_function!(pub last_index_of_from_2, 67, i32, self: *mut Self, item : T, index : i32, count : i32);
    //define_function!(pub remove_all, 68, i32, self: *mut Self, match : *mut NetObject<Predicate<T>>);
    define_function!(pub remove_range, 69, (), self: *mut Self, index : i32, count : i32);
    define_function!(pub reverse_1, 70, (), self: *mut Self);
    define_function!(pub reverse_2, 71, (), self: *mut Self, index : i32, count : i32);
    define_function!(pub sort, 72, (), self: *mut Self);
    //define_function!(pub sort, 73, (), self: *mut Self, comparer : *mut NetObject<IComparer<T>>);
    //define_function!(pub sort, 74, (), self: *mut Self, index : i32, count : i32, comparer : *mut NetObject<IComparer<T>>);
    //define_function!(pub sort, 75, (), self: *mut Self, comparison : *mut NetObject<Comparison<T>>);
    define_function!(pub to_array, 76, *mut NetObject<SystemArray<T>>, self: *mut Self);
    define_function!(pub trim_excess, 77, (), self: *mut Self);
    //define_function!(pub true_for_all, 78, bool, self: *mut Self, match : *mut NetObject<Predicate<T>>);
}