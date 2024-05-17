using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

namespace RustishkaBridge;

public static unsafe class Bridge
{
    static Bridge()
    {
        Convention = (BridgeConvention*)NativeMemory.AlignedAlloc((UIntPtr)Unsafe.SizeOf<BridgeConvention>(), 8);

        Convention->FGetAppDomain = &GetAppDomain;

        Convention->FGetType = (delegate*<object, Type>)typeof(object).GetMethod("GetType")?.MethodHandle.GetFunctionPointer();
        Debug.Assert((UIntPtr)Convention->FGetType != 0);

#pragma warning disable CS8621 // Nullability of reference types in return type doesn't match the target delegate (possibly because of nullability attributes).
        Convention->FSearchType = &Type.GetType;
        Debug.Assert((UIntPtr)Convention->FSearchType != 0);
#pragma warning restore CS8621 // Nullability of reference types in return type doesn't match the target delegate (possibly because of nullability attributes).

        Convention->FAlloc = &RuntimeHelpers.GetUninitializedObject;
        Debug.Assert((UIntPtr)Convention->FAlloc != 0);

        Convention->FAllocString = &AllocateString;
        Debug.Assert((UIntPtr)Convention->FAllocString != 0);

        Convention->FAllocArray = &Array.CreateInstance;
        Debug.Assert((UIntPtr)Convention->FAllocArray != 0);

        Convention->FTryCatch = &TryCatch;
    }

    private static AppDomain GetAppDomain() => AppDomain.CurrentDomain;

    private static string AllocateString(byte* text, int length)
    {
        var arr = System.Buffers.ArrayPool<char>.Shared.Rent(length * 4);
        var span = arr.AsSpan();
        Encoding.UTF8.GetChars(new ReadOnlySpan<byte>(text, length), span);
        var result = span.ToString();
        System.Buffers.ArrayPool<char>.Shared.Return(arr);
        return result;
    }

    private static Exception? TryCatch(delegate*<nint, nint> func, nint data, nint* result)
    {
        try
        {
            *result = func(data);
        }
        catch (Exception e)
        {
            return e;
        }

        return null;
    }

    private static readonly BridgeConvention* Convention;
    
    [StructLayout(LayoutKind.Sequential)]
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

    /// <summary>
    /// Loads the library at path 'libPath' into the application and calls rustishka_handshake
    /// </summary>
    /// <param name="libPath">Path to your Rust library</param>
    /// <param name="moduleHandle">Loaded module handle</param>
    /// <returns>Is the method successfully executed</returns>
    public static bool ConnectRustModule(string libPath, out IntPtr moduleHandle)
    {
        if (!NativeLibrary.TryLoad(libPath, out moduleHandle)) return false;
        if (!NativeLibrary.TryGetExport(moduleHandle, "rustishka_handshake", out var func)) return false;
        ((delegate* unmanaged[Stdcall] <BridgeConvention*, void>)func)(Convention);
        return true;
    }

    /// <summary>
    /// Calls rustishka_handshake
    /// </summary>
    /// <param name="moduleHandle">Loaded module handle</param>
    /// <returns>Is the method successfully executed</returns>
    public static bool ConnectRustModule(IntPtr moduleHandle)
    {
        if (!NativeLibrary.TryGetExport(moduleHandle, "rustishka_handshake", out var func)) return false;
        ((delegate* unmanaged[Stdcall] <BridgeConvention*, void>)func)(Convention);
        return true;
    }
}
