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
    public delegate* <Type, object> FAlloc;
    public delegate* <byte*, int, string> FAllocString;
}
```
The developer must connect the library to the interface using a function call:
```csharp
RustishkaBridge.Bridge.ConnectRustModule(string libPath, out IntPtr moduleHandle) // to load lib and connect
// or
RustishkaBridge.Bridge.ConnectRustModule(IntPtr moduleHandle) // connect without loading
```
#### Rust side
The developer provides an export function with this signature. It will be called in ConnectRustModule
```Rust
#[no_mangle]
extern "stdcall" fn rustishka_handshake(imports: *mut DotnetImports) {
    let ctx = DotnetImportsContainer::new(imports);
    // save ctx as static global variable
}
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
 - Rust lib can allocate type: 
 ```rust
 let someType = ctx.search_type_cached(&someTypeAQN, false);
 let obj: *mut NetObject<SomeType> = ctx.allocate(someType);
 ```
 - Rust can call virtual functions:
 ```rust
 let someType = ...;
 let baseType = someType.get_basetype();
 ```
 This is presented in the form of pseudo wrappers:
 ```rust
 pub fn get_basetype(self: *mut Self) -> *mut NetObject<SystemType> {
        unsafe {
            let ptr: extern "stdcall" fn(*mut NetObject<SystemType>) -> *mut NetObject<SystemType> 
                = core::mem::transmute(self.get_method_table().get_func_at(11, 4));
            
            ptr(self)
        }
    }
 ```
# Examples
[C# side](https://github.com/badryuner/rustishka/blob/master/Rustishka.Tests/SomeTests.cs)
[Rust side](https://github.com/badryuner/rustishka/blob/master/rustishka_examples/src/lib.rs)
# TODO
- Add support for calling non-virtual functions
- Maybe better reflection?
- .Net type inheritance in Rust by creating a custom methodtable. (possible, but very hard to implement)
- Source generators.