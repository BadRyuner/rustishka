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
    public delegate* <AppDomain> FGetAppDomain;
    public delegate* <object, Type> FGetType;
    public delegate* <string, bool, Type> FSearchType;
    public delegate* <Type, object> FAlloc;
    public delegate* <byte*, int, string> FAllocString;
    public delegate* <Type, int, Array> FAllocArray;
    public delegate* <delegate*<nint, nint>, nint, nint*, Exception> FTryCatch;
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
 let integer_type = ctx.search_type_cached(String::from("System.Int32"), false);
 ```
 - Rust lib can allocate objects & arrays: 
 ```rust
 let someType = ctx.search_type_cached(&someTypeAQN, false);
 let obj: *mut NetObject<SomeType> = ctx.allocate(someType);
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
 - Rust lib can use almost all of .NET Reflection's features! 
# Examples
[C# side](https://github.com/badryuner/rustishka/blob/master/Rustishka.Tests/SomeTests.cs)

[Rust side](https://github.com/badryuner/rustishka/blob/master/rustishka_examples/src/lib.rs)
# TODO
- Add support for calling non-virtual instance & static functions without reflection i-n-v-o-k-e
- Add support for field access
- Add Delegates Support
- Maybe better reflection?
- .Net type inheritance in Rust by creating a custom type via .Net TypeBuilder & overriding methodtable entries.
- Source generators.