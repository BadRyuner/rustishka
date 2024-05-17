using System.Reflection;
using System.Text;
using System.Text.Json;

namespace Rustishka.Tools;

internal unsafe class Program
{
    public static void Main(string[] args)
    {
        DisplayType(typeof(StringBuilder), pub: true);
        Console.ReadLine();
    }

    private static readonly BindingFlags AllVirtual =
        BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic;

    private static readonly BindingFlags AllStatic =
        BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic;

    public static void DisplayType(Type type, bool pub = false)
    {
        var irmi = Type.GetType("System.RuntimeMethodHandleInternal")!;
        var getSlot = (delegate* <IntPtr, int>)typeof(RuntimeMethodHandle).GetMethod("GetSlot", AllStatic, [irmi])!
            .MethodHandle.GetFunctionPointer();
        var rt = Type.GetType("System.RuntimeType")!;
        var getNumVirtuals = (delegate*<Type, int>)typeof(RuntimeTypeHandle).GetMethod("GetNumVirtuals", AllStatic, [rt])!
            .MethodHandle.GetFunctionPointer();

        var virtuals = getNumVirtuals(type);

        foreach (var method in type.GetMethods(AllVirtual).Where(_ => _.IsVirtual))
        {
            if (method.IsGenericMethod) continue;
            if (method.GetBaseDefinition() != method) continue;

            var slot = getSlot(method.MethodHandle.Value);
            if (slot >= virtuals) continue;

            Console.Write("    define_virtual!(");
            if (pub) Console.Write("pub, ");
            Console.Write(JsonNamingPolicy.SnakeCaseLower.ConvertName(method.Name));
            Console.Write($", {slot / 8}, {slot % 8}, ");
            Console.Write(ConvertType(method.ReturnType));
            foreach (var parameter in method.GetParameters())
            {
                Console.Write($", {JsonNamingPolicy.SnakeCaseLower.ConvertName(parameter.Name ?? $"unk_{parameter.Position}")} : {ConvertType(parameter.ParameterType)}");
            }
            Console.WriteLine(");");
        }
    }

    public static string ConvertType(Type type)
    {
        if (type.IsSZArray)
            return $"*mut NetObject<SystemArray<{ConvertType(type.GetElementType()!)}>>";
        if (!type.IsValueType)
            return $"*mut NetObject<{ToRustName(type)}>";
        return ToRustName(type);
    }

    private static string ToRustName(Type t)
    {
        switch (t.Name)
        {
            case "Type": return "SystemType";
            case "String": return "SystemString";
            case "Object": return "SystemObject";
            case "Boolean": return "bool";
            case "SByte": return "i8";
            case "Short": return "i16";
            case "Int32": return "i32";
            case "Int64": return "i64";
            case "IntPtr": return "isize";
            case "Byte": return "u8";
            case "Char":
            case "Ushort": return "u16";
            case "UInt32": return "u32";
            case "UInt64": return "u64";
            case "UIntPtr": return "usize";
            case "Single": return "f32";
            case "Double": return "f64";
            case "Void": return "()";
            default: return t.Name;
        }
    }
}