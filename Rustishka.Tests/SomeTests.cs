using System.Runtime.InteropServices;

namespace Rustishka.Tests;

public unsafe class SomeTests
{
    private static readonly IntPtr _moduleHandle;

    static SomeTests()
    {
        var result =
            RustishkaBridge.Bridge.ConnectRustModule("..\\..\\..\\..\\rustishka_examples\\target\\release\\rustishka_examples.dll",
            out _moduleHandle);
        Assert.True(result, "Cant load rustishka example");
    }

    [Fact]
    public void TryCallToString()
    {
        // important thing: don`t use unmanaged[stdcall]!!! or dotnet will shoot rust in the foot.
        var func = (delegate* <object, string>)NativeLibrary.GetExport(_moduleHandle,
            "try_call_to_string");
        Assert.Equal("System.Object", func(new object()));
        Assert.Equal("lol", func("lol"));
        Assert.Equal("123456", func(123456));
    }

    [Fact]
    public void TestObjectEquals()
    {
        var func = (delegate*<object, object, bool>)NativeLibrary.GetExport(_moduleHandle,
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
        var func = (delegate*<char*, nuint, Type>)NativeLibrary.GetExport(_moduleHandle,
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
        var func = (delegate*<System.Type, object>)NativeLibrary.GetExport(_moduleHandle,
            "alloc_object");

        Assert.Equal(typeof(object), func(typeof(object)).GetType());
        Assert.Equal(typeof(Exception), func(typeof(Exception)).GetType());
    }

    [Fact]
    public void TestReflection()
    {
        var func = (delegate*<System.Type, Type>)NativeLibrary.GetExport(_moduleHandle,
            "get_basetype");

        //Assert.Equal(typeof(int).BaseType, func(typeof(int)));
        Assert.Equal(typeof(Exception).BaseType, func(typeof(Exception)));
    }
}