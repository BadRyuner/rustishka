using System.Runtime.InteropServices;

namespace Rustishka.Tests;
public unsafe class TestReflection
{
    public static string Dummy() => "FUCK YEAAAAAAAAH";

    [Fact]
    public void TestFindAndInvoke()
    {
        var func = (delegate* <string, string, object?[]?, object>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "find_and_invoke");
        Assert.Equal(Dummy(), func(typeof(TestReflection).AssemblyQualifiedName!, "Dummy", null) as string);
    }

    [Fact]
    public void TestInteractionWithClassViaReflection()
    {
        var getField = (delegate*<object, string, object>)NativeLibrary.GetExport(SharedRustModule.ModuleHandle,
            "try_get_field_from_instance");
        DummyClass obj = new();
        Assert.Equal(obj.ItsString, getField(obj, "ItsString"));
        Assert.Equal(obj.ItsInt, getField(obj, "ItsInt"));
        Assert.Equal(obj.ItsFloat, getField(obj, "ItsFloat"));
        
    }

    public class DummyClass
    {
        public string ItsString = "Omg, Really?";
        public int ItsInt = 123;
        public float ItsFloat = 44f;
    }
}
