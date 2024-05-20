use std::marker::PhantomData;

use crate::{define_constructor, define_function, define_virtual, managed_array, search_type_cached, wrappers::system::{AutoStructBox, NetObject}};

use super::{system_reflection::SystemType, TypeInfoProvider};

pub struct Dictionary<K, V> {
    a: PhantomData<K>,
    b: PhantomData<V>
}

pub fn generic_type_of() -> *mut NetObject<SystemType> {
    static MY_TYPE: std::sync::OnceLock<usize> = std::sync::OnceLock::new(); 
    *MY_TYPE.get_or_init(|| search_type_cached(&String::from("System.Collections.Generic.Dictionary`2"), true) as usize) as *mut NetObject<SystemType>
}

impl<K: TypeInfoProvider, V: TypeInfoProvider> TypeInfoProvider for Dictionary<K, V> {
    fn type_of() -> *mut NetObject<SystemType> {
        static MY_TYPE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        *MY_TYPE.get_or_init(|| generic_type_of().make_generic_type(managed_array!(SystemType, 2, K::type_of(), V::type_of())) as usize) as *mut NetObject<SystemType>
    }
}

impl<K: TypeInfoProvider, V: TypeInfoProvider> Dictionary<K, V> {
    define_constructor!(pub new);
    define_constructor!(pub new_1, capacity : i32);
    //define_constructor!(pub new_2, comparer : *mut NetObject<IEqualityComparer`1>);
    //define_constructor!(pub new_3, capacity : i32, comparer : *mut NetObject<IEqualityComparer`1>);
    //define_constructor!(pub new_4, dictionary : *mut NetObject<IDictionary`2>);
    //define_constructor!(pub new_5, dictionary : *mut NetObject<IDictionary`2>, comparer : *mut NetObject<IEqualityComparer`1>);
    //define_constructor!(pub new_6, collection : *mut NetObject<IEnumerable`1>);
    //define_constructor!(pub new_7, collection : *mut NetObject<IEnumerable`1>, comparer : *mut NetObject<IEqualityComparer`1>);
}

impl<TKey: TypeInfoProvider, TValue: TypeInfoProvider> NetObject<Dictionary<TKey, TValue>> {
    // Virtual functions
    define_virtual!(pub get_count, 0, 4, i32);
    define_virtual!(pub get_item, 1, 1, TValue, key : TKey);
    define_virtual!(pub set_item, 1, 2, (), key : TKey, value : TValue);
    define_virtual!(pub add, 1, 3, (), key : TKey, value : TValue);
    define_virtual!(pub clear, 1, 7, ());
    define_virtual!(pub contains_key, 2, 0, bool, key : TKey);
    define_virtual!(pub remove, 2, 4, bool, key : TKey);
    define_virtual!(pub try_get_value, 2, 5, bool, key : TKey, value : *mut TValue);

    // Non-Virtual functions
    //define_function!(pub get_comparer, 48, *mut NetObject<IEqualityComparer`1>, self: *mut Self);
    //define_function!(pub get_keys, 49, *mut NetObject<KeyCollection>, self: *mut Self);
    //define_function!(pub get_values, 50, *mut NetObject<ValueCollection>, self: *mut Self);
    define_function!(pub contains_value, 51, bool, self: *mut Self, value : TValue);
    //define_function!(pub get_enumerator, 53, Enumerator, self: *mut Self);
    define_function!(pub try_add, 60, bool, self: *mut Self, key : TKey, value : TValue);
    define_function!(pub ensure_capacity, 61, i32, self: *mut Self, capacity : i32);
    define_function!(pub trim_excess, 62, (), self: *mut Self);
    define_function!(pub trim_excess_cap, 63, (), self: *mut Self, capacity : i32);
}