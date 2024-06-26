﻿using System.Collections;
using System.Numerics;
using System.Reflection;
using System.Reflection.Emit;
using System.Text.Json;

namespace Rustishka.Tools;

public static unsafe class Program
{
    public static void Main(string[] args)
    {
        DisplayType(typeof(OpCodes), pub: true, skipVirtual: false, skipConstructors: false);
        //DisplayInterface(typeof(TypeBuilder));
        Console.ReadLine();
    }

    private static readonly BindingFlags AllInstance =
        BindingFlags.Instance | BindingFlags.Public ;//| BindingFlags.NonPublic;

    private static readonly BindingFlags AllStatic =
        BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic;

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

        Console.WriteLine($"\nimpl {className} {{");

        if (!skipConstructors)
        {
            int counter = 0;
            foreach (var ctor in type.GetConstructors(AllInstance))
            {
                Console.Write("    define_constructor!(");
                if (pub) Console.Write("pub ");
                Console.Write(counter == 0 ? "new" : $"new_{counter}");
                WriteParams(ctor.GetParameters(), true, false, true);
                Console.WriteLine(");");
                counter++;
            }
        }

        Console.WriteLine("\n// static fields\n");
        foreach (var field in type.GetFields(AllStatic))
        {
            Console.WriteLine($"    define_static_field!({(pub ? "pub " : null)}get_{JsonNamingPolicy.SnakeCaseLower.ConvertName(field.Name)}, \"{field.Name}\", {ConvertType(field.FieldType, false)}{(field.FieldType.IsValueType ? ", unbox" : null)});");
        }

        Console.WriteLine("}");

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
                Console.Write(ConvertType(method.ReturnType, false));
                WriteParams(method.GetParameters(), false, true);
                Console.WriteLine(");");
            }
        }

        Console.WriteLine("\n    // Non-Virtual functions");

        foreach (var method in type.GetMethods(AllInstance | AllStatic).Where(_ => !_.IsVirtual).OrderBy(_ => _.IsStatic))
        {
            if (method.DeclaringType != type) continue;

            var slot = getSlot(method.MethodHandle.Value);
            Console.Write("    define_function!(");
            if (pub) Console.Write("pub ");
            Console.Write(JsonNamingPolicy.SnakeCaseLower.ConvertName(method.Name));
            Console.Write($", {slot}, ");
            Console.Write(ConvertType(method.ReturnType, false));
            WriteParams(method.GetParameters(), method.IsStatic, false);
            Console.WriteLine(");");
        }

        Console.WriteLine("}");
    }

    public static void DisplayInterface(Type type)
    {
        Console.WriteLine($"pub struct {type.Name} {{ }}");

        Console.WriteLine($"\ndefine_typeof!({type.Name}, \"{type.AssemblyQualifiedName}\");\n\nimpl NetObject<{type.Name}> {{");

        int id = 0;
        foreach (var method in type.GetMethods(AllInstance))
        {
            Console.Write($"    pub fn {JsonNamingPolicy.SnakeCaseLower.ConvertName(method.Name)}(self: *mut Self");
            var p = method.GetParameters();
            foreach (var parameter in p)
            {
                Console.Write($", {JsonNamingPolicy.SnakeCaseLower.ConvertName(parameter.Name ?? $"unk_{parameter.Position}")} : {ConvertType(parameter.ParameterType, true)}");
            }
            Console.WriteLine($") -> {ConvertType(method.ReturnType, false)} {{");
            Console.Write($"        let method = resolve_interface_method!(self, Self, {id}, {p.Length}");
            if (p.Length > 0)
            {
                foreach (var parameter in p)
                    Console.Write($", {ConvertType(parameter.ParameterType, true)}");
            }
            Console.WriteLine(");");
            bool unwrap = method.ReturnType.IsValueType && method.ReturnType != typeof(void);
            Console.Write($"        {(unwrap ? "*(" : null)}method.invoke(self as _, managed_array!(SystemObject, {p.Length}");
            if (p.Length > 0)
            {
                foreach (var parameter in p)
                {
                    Console.Write($", {JsonNamingPolicy.SnakeCaseLower.ConvertName(parameter.Name ?? $"unk_{parameter.Position}")}");
                    if (parameter.ParameterType.IsValueType)
                        Console.Write("._box_value() as _");
                }
            }
            Console.Write("))");
            if (unwrap)
                Console.WriteLine(" as *mut NetObject<_>).get_content()");
            else if (method.ReturnType == typeof(void))
                Console.WriteLine(';');
            else
                Console.WriteLine();
            Console.WriteLine("    }");

            id++;
        }

        Console.WriteLine("}");
    }

    public static void WriteParams(ParameterInfo[] parameters, bool isStatic, bool isVirtual, bool isCtor = false)
    {
        if (!isStatic && !isVirtual)
            Console.Write(", self: *mut Self");

        foreach (var parameter in parameters)
        {
            Console.Write($", {JsonNamingPolicy.SnakeCaseLower.ConvertName(parameter.Name ?? $"unk_{parameter.Position}")} : {ConvertType(parameter.ParameterType, isCtor)}");
        }
    }

    public static string ConvertType(Type type, bool wrap)
    {
        if (type.IsGenericTypeParameter)
            return type.ToString();
        if (type.IsSZArray)
            return $"*mut NetObject<SystemArray<{ConvertType(type.GetElementType()!, wrap)}>>";
        if (type.IsPointer)
            return wrap ? $"Ptr<{ConvertType(type.GetElementType()!, wrap)}>"
                    : $"*mut {ConvertType(type.GetElementType()!, wrap)}";
        if (type.IsByRef)
            return wrap ? $"ByRef<{ConvertType(type.GetElementType()!, wrap)}>"
                : $"*mut {ConvertType(type.GetElementType()!, wrap)}";
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