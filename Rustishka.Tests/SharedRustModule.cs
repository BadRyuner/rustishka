namespace Rustishka.Tests;
internal static class SharedRustModule
{
    internal static IntPtr ModuleHandle;

    static SharedRustModule()
    {
        var result =
            RustishkaBridge.Bridge.ConnectRustModule("..\\..\\..\\..\\rustishka_examples\\target\\release\\rustishka_examples.dll",
                out ModuleHandle);
        Assert.True(result, "Cant load rustishka example");
    }
}
