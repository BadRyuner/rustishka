using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

namespace RustishkaBridge;

public static unsafe class Bridge
{
    static Bridge()
    {
        Convention = (BridgeConvention*)NativeMemory.AlignedAlloc((UIntPtr)Unsafe.SizeOf<BridgeConvention>(), 8);

        Convention->FGetType = (delegate*<object, Type>)typeof(object).GetMethod("GetType")?.MethodHandle.GetFunctionPointer();
        Debug.Assert((UIntPtr)Convention->FGetType != 0);

#pragma warning disable CS8621 // Nullability of reference types in return type doesn't match the target delegate (possibly because of nullability attributes).
        Convention->FSearchType = &Type.GetType;
        Debug.Assert((UIntPtr)Convention->FSearchType != 0);
#pragma warning restore CS8621 // Nullability of reference types in return type doesn't match the target delegate (possibly because of nullability attributes).

        Convention->FGetMethodAtSlot = &GetMethodAtSlot;
        Debug.Assert((UIntPtr)Convention->FGetMethodAtSlot != 0);

        Convention->FAlloc = &RuntimeHelpers.GetUninitializedObject;
        Debug.Assert((UIntPtr)Convention->FAlloc != 0);

        Convention->FAllocString = &AllocateString;
        Debug.Assert((UIntPtr)Convention->FAllocString != 0);

        Convention->FAllocArray = &Array.CreateInstance;
        Debug.Assert((UIntPtr)Convention->FAllocArray != 0);

        Convention->FTryCatch = &TryCatch;
        Debug.Assert((UIntPtr)Convention->FTryCatch != 0);

        Convention->FThrow = &ThrowHelper;
        Debug.Assert((UIntPtr)Convention->FThrow != 0);

        var rmhi = Type.GetType("System.RuntimeMethodHandleInternal")!;
        GetSlot = (delegate*<IntPtr, int>)typeof(RuntimeMethodHandle)
            .GetMethod("GetSlot", BindingFlags.Static | BindingFlags.NonPublic, [rmhi])!
            .MethodHandle.GetFunctionPointer();
    }

    private static readonly delegate*<IntPtr, int> GetSlot;

    private static void* GetMethodAtSlot(Type ty, int slot)
    {
        // in theory slot in CLR it`s ty.GetALLMethods()[slot]. But I'm a lazy ass to check it.
        // it would be nice to do method.ToString() == MethodName instead of slot, but my inner bytefucker hisses
        var methods =
            ty.GetMethods(BindingFlags.Instance | BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic);
        for (int i = 0; i < slot; i++)
        {
            var method = methods[i];
            if (GetSlot(method.MethodHandle.Value) == slot)
                return method.MethodHandle.GetFunctionPointer().ToPointer();
        }

        throw null!;
    }

    private static string AllocateString(byte* text, int length)
    {
        var arr = System.Buffers.ArrayPool<char>.Shared.Rent(length * 3);
        var span = arr.AsSpan();
        var written = Encoding.UTF8.GetChars(new ReadOnlySpan<byte>(text, length), span);
        var result = span.Slice(0, written).ToString();
        System.Buffers.ArrayPool<char>.Shared.Return(arr);
        return result;
    }

    private static Exception? TryCatch(delegate*<void> func)
    {
        try
        {
            func();
        }
        catch (Exception e)
        {
            return e;
        }

        return null;
    }

    private static void ThrowHelper(Exception e) => throw e; 

    private static readonly BridgeConvention* Convention;
    
    [StructLayout(LayoutKind.Sequential)]
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
