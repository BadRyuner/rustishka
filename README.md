# Rustishka
This project offers cool ways to interact between these super safe & supe fast languages!

Rust <===> C# interop at the highest level!

This effect is created by using special libraries on the C# side and on the Rust side.
# Overview
#### C# Side
The library provides a mini interface for the Rust library to interact with the DotNet environment.
```csharp
private struct BridgeConvention
{
    public delegate* <object, Type> FGetType;
    public delegate* <string, bool, Type> FSearchType;
    public delegate* <Type, int, void*> FGetMethodAtSlot;
    public delegate* <Type, object> FAlloc;
    public delegate* <byte*, int, string> FAllocString;
    public delegate* <Type, int, Array> FAllocArray;
    public delegate* <delegate*<void>, Exception> FTryCatch;
    public delegate* <Exception, void> FThrow;
}
```
The developer must connect the library to the interface using a function call:
```csharp
RustishkaBridge.Bridge.ConnectRustModule(string libPath, out IntPtr moduleHandle) // to load lib and connect
// or
RustishkaBridge.Bridge.ConnectRustModule(IntPtr moduleHandle) // connect without loading. Use if the module has already been loaded before.
```
#### Rust side
The developer provides an export function with this signature. It will be called in ConnectRustModule
```Rust
initialize_rustishka!();
```
#### Important thing
Structures with class content for Rust have to be created manually.

[A tool for obtaining offsets](https://github.com/SergeyTeplyakov/ObjectLayoutInspector)

You can use Sharplab as well.
# Features
 - Rust lib can search for a type in the DotNet runtime: 
 ```rust
 let integer_type = search_type_cached(&String::from("System.Int32"), false);
 ```
 - Rust lib can allocate objects & arrays: 
 ```rust
 let someType = search_type_cached(&someTypeAQN, false);
 let obj: *mut NetObject<SomeType> = allocate(someType); // alloc without constructor invoke !!!
 ```
 - Rust lib can call virtual functions:
 ```rust
 let someType: *mut NetObject<SystemType> = ...;
 let baseType = someType.get_base_type();
 ```
 This is presented in the form of pseudo wrappers:
 ```rust
 define_virtual!(pub, get_base_type, 11, 4, *mut NetObject<SystemType>);
 ```
 where: 11 - row eq `slotId / 8`, 4 - index eq `slotId % 8`
 - Rust lib can call non-virtual instance and static functions.
 ```rust
 someObject.some_instance_method(args);
 NetObject::<SomeObject>::some_static_method(args)
 let val: *mut NetObject<SomeObject> = SomeObject::new(args);
 ```
 where
 ```rust
impl NetObject<SomeObject> {
    define_function!(pub some_instance_method, 7, self: *mut Self, arg_name: ArgType); // where 7 - slotId
    define_function!(pub some_static_method, 8, arg_name: ArgType);
}
impl SomeObject {
    define_constructor!(pub new, arg_name: ArgType);
}
 ```
 - Rust lib can access to static fields 
 ```Rust
 SomeObject::get_cool_static_field();
 // where
 define_typeof!(SomeObject, "blahblah");
 impl SomeObject {
    define_static_field!(pub get_cool_static_field, "CoolStaticField", SomeFieldType);
 }
 ```
 - Rust lib can use typeof sugar
```Rust
pub struct SomeObject { }
define_typeof!(SomeObject, "SomeObject AssemblyQualifiedName");
// in method
let ty : *mut NetObject<SystemType> = SomeObject::type_of();
```
 - Rust lib can easily alloc managed arrays 
 ```Rust
 managed_array!(ElementType, elements num, elements);
 // ex:
 managed_array!(i32, 5, 0, 1, 2, 3, 4);
 managed_array!(SystemType, 2, i32::type_of(), f32::type_of());
 ```
 - Rust lib can use almost all of .NET Reflection's features! (Even use DynamicMethod!)

  As you can see: it`s very human design, very easy to use. 
# Examples
[C# side](https://github.com/badryuner/rustishka/blob/master/Rustishka.Tests/SomeTests.cs)

[Rust side](https://github.com/badryuner/rustishka/blob/master/rustishka_examples/src/lib.rs)
# TODO
- Add support for field access
- .Net type inheritance in Rust by creating a custom type via .Net TypeBuilder & overriding methodtable entries.
- Source generators (atm it's scary Rustishka.Tools)