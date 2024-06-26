using System.Collections;
using System.Reflection;
using System.Reflection.Emit;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Rustishka.Tests;

public unsafe class SomeTests
{
    [Fact]
    public void TryCallToString()
    {
        // important thing: don`t use unmanaged[stdcall]!!! or dotnet will shoot rust in the foot.
        var func = (delegate* <object, string>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "try_call_to_string");
        Assert.Equal("System.Object", func(new object()));
        Assert.Equal("lol", func("lol"));
        Assert.Equal("123456", func(123456));
    }

    [Fact]
    public void TestObjectEquals()
    {
        var func = (delegate*<object, object, bool>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "i_can_compare_objects_too");

        CompareThings(1, 1);
        CompareThings(1, 2);
        CompareThings("boo", "boo");

        void CompareThings(object left, object right) =>
            Assert.Equal(left.Equals(right), func(left, right));
    }

    [Fact]
    public void TestSearchType()
    {
        var func = (delegate*<char*, nuint, Type>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "find_class_4_me_senpai");

        Check<int>();
        Check<string>();
        Check<SomeTests>();

        void Check<T>()
        {
            var type = typeof(T);
            var aql = type.AssemblyQualifiedName!;
            fixed (char* c = aql)
            {
                var result = func(c, (nuint)aql.Length);
                Assert.Equal(type, result);
            }
        }
    }

    [Fact]
    public void TestAllocObject()
    {
        var func = (delegate*<System.Type, object>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "alloc_object");

        Assert.Equal(typeof(object), func(typeof(object)).GetType());
        Assert.Equal(typeof(Exception), func(typeof(Exception)).GetType());
    }

    [Fact]
    public void TestReflection()
    {
        var func = (delegate*<System.Type, Type>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "get_basetype");

        //Assert.Equal(typeof(int).BaseType, func(typeof(int)));
        Assert.Equal(typeof(Exception).BaseType, func(typeof(Exception)));
    }

    [Fact]
    public void TestDelegateCall()
    {
        var func = (delegate*<Delegate, void>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "try_call_delegate_without_args");

        func(static () => Assert.True(true));
    }

    [Fact]
    public void TypeofInRust()
    {
        var func = (delegate*<Type>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "try_typeof_sys_object");

        Assert.Equal(typeof(object), func());
    }

    [Fact]
    public void CallSomeNonVirtualFunc()
    {
        var func = (delegate*<string>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "try_get_base_dir");

        Assert.Equal(AppDomain.CurrentDomain.BaseDirectory, func());
    }

    [Fact]
    public void CreateAssemblyName()
    {
        var func = (delegate*<AssemblyName>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "create_assembly_name");

        var ass = func();
        Assert.Equal("PoopAssembly", ass.Name);
        Assert.Equal(new(1,2,3,4), ass.Version);
    }

    //[Fact]
    public void ThrowSomeErrors()
    {
        try
        {
            var func = (delegate*<void>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
                "create_and_throw");
            func();
        }
        catch (Exception ex)
        {
            // cursed. It`s works, but xUnit ignores this Assert wtf
            Assert.StartsWith("JUST DO IT!", ex.Message);
        }
    }

    [Fact]
    public void AllocListAndFill()
    {
        var func = (delegate*<List<int>>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "alloc_list_and_fill");
        Assert.Equal([0,1,2,3,4,5,6,7,8,9], func());
    }

    [Fact]
    public void EnumerateToList()
    {
        var func = (delegate*<IEnumerator, List<int>>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "enumerate_to_list");
        List<int> list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        var enumerator = (list as IEnumerable).GetEnumerator();
        var result = func(enumerator);
        Assert.Equal(list, result);
        ((IDisposable)enumerator).Dispose();
    }

    [Fact]
    public void TestIList()
    {
        var func = (delegate*<IList<int>, List<int>>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "do_smth_with_ilist");
        List<int> list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        var result = func(list);
        Assert.Equal(list, result);
    }


    [Fact]
    public void TestCreateMethod()
    {
        var func = (delegate*<DynamicMethod>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "create_method");
        var method = func();
        Assert.Equal(2, method.Invoke(null, null));
    }
}