using System.Reflection;
using System.Reflection.Metadata;
using System.Text;
using System.Text.Json;

namespace Rustishka.Tools;

public static unsafe class Program
{
    public static void Main(string[] args)
    {
        DisplayType(typeof(Console), pub: true, skipVirtual: false, skipConstructors: false); 
        Console.ReadLine();
    }

    private static readonly BindingFlags AllInstance =
        BindingFlags.Instance | BindingFlags.Public ;//| BindingFlags.NonPublic;

    private static readonly BindingFlags AllStatic =
        BindingFlags.Static | BindingFlags.Public ;//| BindingFlags.NonPublic;

    public static void DisplayType(Type type, bool pub = false, bool skipVirtual = false, bool skipConstructors = false)
    {
        var irmi = Type.GetType("System.RuntimeMethodHandleInternal")!;
        var getSlot = (delegate* <IntPtr, int>)typeof(RuntimeMethodHandle).GetMethod("GetSlot", AllStatic | BindingFlags.NonPublic, [irmi])!
            .MethodHandle.GetFunctionPointer();
        var rt = Type.GetType("System.RuntimeType")!;
        var getNumVirtuals = (delegate*<Type, int>)typeof(RuntimeTypeHandle).GetMethod("GetNumVirtuals", AllStatic | BindingFlags.NonPublic, [rt])!
            .MethodHandle.GetFunctionPointer();

        var virtuals = getNumVirtuals(type);

        var className = type.Name;
        Console.WriteLine($"pub struct {className} {{ }}");

        Console.WriteLine($"\ndefine_typeof!({className}, \"{type.AssemblyQualifiedName}\");");

        if (!skipConstructors)
        {
            int counter = 0;
            Console.WriteLine($"\nimpl {className} {{");
            foreach (var ctor in type.GetConstructors(AllInstance))
            {
                Console.Write("    define_constructor!(");
                if (pub) Console.Write("pub ");
                Console.Write(counter == 0 ? "new" : $"new_{counter}");
                WriteParams(ctor.GetParameters(), true, false);
                Console.WriteLine(");");
                counter++;
            }
            Console.WriteLine("}");
        }

        Console.WriteLine($"\nimpl NetObject<{className}> {{");
        

        if (!skipVirtual)
        {
            Console.WriteLine("    // Virtual functions");
            foreach (var method in type.GetMethods(AllInstance).Where(_ => _.IsVirtual))
            {
                if (method.IsGenericMethod) continue;
                if (method.GetBaseDefinition() != method) continue;

                var slot = getSlot(method.MethodHandle.Value);
                if (slot >= virtuals) continue;

                Console.Write("    define_virtual!(");
                if (pub) Console.Write("pub ");
                Console.Write(JsonNamingPolicy.SnakeCaseLower.ConvertName(method.Name));
                Console.Write($", {slot / 8}, {slot % 8}, ");
                Console.Write(ConvertType(method.ReturnType));
                WriteParams(method.GetParameters(), false, true);
                Console.WriteLine(");");
            }
        }

        Console.WriteLine("\n    // Non-Virtual functions");

        foreach (var method in type.GetMethods(AllInstance | AllStatic).Where(_ => !_.IsVirtual))
        {
            if (method.DeclaringType != type) continue;

            var slot = getSlot(method.MethodHandle.Value);
            Console.Write("    define_function!(");
            if (pub) Console.Write("pub ");
            Console.Write(JsonNamingPolicy.SnakeCaseLower.ConvertName(method.Name));
            Console.Write($", {slot}, ");
            Console.Write(ConvertType(method.ReturnType));
            WriteParams(method.GetParameters(), method.IsStatic, false);
            Console.WriteLine(");");
        }

        Console.WriteLine("}");
    }

    public static void WriteParams(ParameterInfo[] parameters, bool isStatic, bool isVirtual)
    {
        if (!isStatic && !isVirtual)
            Console.Write(", self: *mut Self");

        foreach (var parameter in parameters)
        {
            Console.Write($", {JsonNamingPolicy.SnakeCaseLower.ConvertName(parameter.Name ?? $"unk_{parameter.Position}")} : {ConvertType(parameter.ParameterType)}");
        }
    }

    public static string ConvertType(Type type)
    {
        if (type.IsSZArray)
            return $"*mut NetObject<SystemArray<{ConvertType(type.GetElementType()!)}>>";
        if (type.IsPointer || type.IsByRef)
            return $"*mut {ConvertType(type.GetElementType()!)}";
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
            case "Int16": return "i16";
            case "Int32": return "i32";
            case "Int64": return "i64";
            case "IntPtr": return "isize";
            case "Byte": return "u8";
            case "Char":
            case "UInt16": return "u16";
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