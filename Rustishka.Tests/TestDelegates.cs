using System.Runtime.InteropServices;

namespace Rustishka.Tests;
public unsafe class TestDelegates
{
    [Fact]
    public void TestAction()
    {
        var func = (delegate*<Action, void>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_action");
        func(() => Assert.True(true));
    }

    [Fact]
    public void TestAction1()
    {
        var func = (delegate*<Action<object>, object, void>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_action_1");
        object v1 = new();
        func((arg1) => Assert.Equal(v1, arg1), v1);
    }

    [Fact]
    public void TestAction2()
    {
        var func = (delegate*<Action<object, object>, object, object, void>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_action_2");
        object v1 = new();
        object v2 = new();
        func((arg1, arg2) => Assert.Equal((v1, v2), (arg1, arg2)), v1, v2);
    }

    [Fact]
    public void TestAction3()
    {
        var func = (delegate*<Action<object, object, object>, object, object, object, void>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_action_3");
        object v1 = new();
        object v2 = new();
        object v3 = new();
        func((arg1, arg2, arg3) => Assert.Equal((v1,v2,v3), (arg1, arg2, arg3)), v1, v2, v3);
    }

    [Fact]
    public void TestFunc1()
    {
        var func = (delegate*<Func<object>, object>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_func1");
        object res = new();
        Assert.Equal(res, func(() => res));
    }

    [Fact]
    public void TestFunc2()
    {
        var func = (delegate*<Func<object, object>, object, object>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_func2");
        object res = new();
        object v1 = new();
        Assert.Equal((res, v1), func(a1 => (res, a1), v1));
    }

    [Fact]
    public void TestFunc3()
    {
        var func = (delegate*<Func<object, object, object>, object, object, object>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_func3");
        object res = new();
        object v1 = new();
        object v2 = new();
        Assert.Equal((res, v1, v2), func((a1, a2) => (res, a1, a2), v1, v2));
    }

    [Fact]
    public void TestFuncPass1()
    {
        var func = (delegate*<Func<bool>>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_pass_func1");

        Assert.True(func()());
    }

    [Fact]
    public void TestFuncPass2()
    {
        var func = (delegate*<Func<int, int>>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "test_pass_func2");
        Func<int, int> f = func();
        Assert.Equal(1, f(1)); // why you return 0 WTF unstable shit
        Assert.Equal(0, f(0));
    }
}
