use crate::allocate_string;
use crate::define_static_field;
use crate::wrappers::system::TypeInfoProvider;
use crate::wrappers::system::AutoStructBox;

use bitflags::bitflags;

use crate::{define_constructor, define_function, define_typeof, define_virtual};

use super::system_reflection::BindingFlags;
use super::{system_array::SystemArray, system_reflection::{CallingConventions, ConstructorInfo, FieldInfo, MemberInfoBindings, MethodAttributes, MethodBaseBindings, MethodInfo, Module, SystemType}, system_string::SystemString, NetObject, SystemObject};

pub struct DynamicMethod { }

define_typeof!(DynamicMethod, "System.Reflection.Emit.DynamicMethod");

impl DynamicMethod {
    define_constructor!(pub new, name : *mut NetObject<SystemString>, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_constructor!(pub new_1, name : *mut NetObject<SystemString>, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, restricted_skip_visibility : bool);
    define_constructor!(pub new_2, name : *mut NetObject<SystemString>, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, m : *mut NetObject<Module>);
    define_constructor!(pub new_3, name : *mut NetObject<SystemString>, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, m : *mut NetObject<Module>, skip_visibility : bool);
    define_constructor!(pub new_4, name : *mut NetObject<SystemString>, attributes : MethodAttributes, calling_convention : CallingConventions, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, m : *mut NetObject<Module>, skip_visibility : bool);
    define_constructor!(pub new_5, name : *mut NetObject<SystemString>, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, owner : *mut NetObject<SystemType>);
    define_constructor!(pub new_6, name : *mut NetObject<SystemString>, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, owner : *mut NetObject<SystemType>, skip_visibility : bool);
    define_constructor!(pub new_7, name : *mut NetObject<SystemString>, attributes : MethodAttributes, calling_convention : CallingConventions, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, owner : *mut NetObject<SystemType>, skip_visibility : bool);
}

impl NetObject<DynamicMethod> {
    // Non-Virtual functions
    //define_function!(pub get_method_descriptor, 46, RuntimeMethodHandle, self: *mut Self);
    //define_function!(pub get_invoker, 47, *mut NetObject<MethodBaseInvoker>, self: *mut Self);
    //define_function!(pub get_signature, 48, *mut NetObject<Signature>, self: *mut Self);
    define_function!(pub get_dynamic_il_info, 49, *mut NetObject<DynamicILInfo>, self: *mut Self);
    define_function!(pub get_il_generator_with_size, 50, *mut NetObject<ILGenerator>, self: *mut Self, stream_size : i32);
    define_function!(pub init, 60, (), self: *mut Self, name : *mut NetObject<SystemString>, attributes : MethodAttributes, calling_convention : CallingConventions, return_type : *mut NetObject<SystemType>, signature : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, owner : *mut NetObject<SystemType>, m : *mut NetObject<Module>, skip_visibility : bool, transparent_method : bool);
    define_function!(pub define_parameter, 61, *mut NetObject<ParameterBuilder>, self: *mut Self, position : i32, attributes : ParameterAttributes, parameter_name : *mut NetObject<SystemString>);
    define_function!(pub get_il_generator, 62, *mut NetObject<ILGenerator>, self: *mut Self);
    define_function!(pub get_init_locals, 63, bool, self: *mut Self);
    define_function!(pub set_init_locals, 64, (), self: *mut Self, value : bool);
    define_function!(pub get_dynamic_methods_module, 59, *mut NetObject<Module>);
}

impl MethodBaseBindings for NetObject<DynamicMethod> {}
impl MemberInfoBindings for NetObject<DynamicMethod> {}

bitflags! {
    pub struct ParameterAttributes: u32
    {
        const None = 0;
        const In = 1;
        const Out = 2;
        const Lcid = 4;
        const Retval = 8;
        const Optional = 16; // 0x00000010
        const HasDefault = 4096; // 0x00001000
        const HasFieldMarshal = 8192; // 0x00002000
    }
}

pub struct DynamicILInfo { }

define_typeof!(DynamicILInfo, "System.Reflection.Emit.DynamicILInfo");

impl DynamicILInfo {
}

impl NetObject<DynamicILInfo> {
    define_function!(pub get_callable_method, 5, (), self: *mut Self, module : *mut NetObject<Module>, dm : *mut NetObject<DynamicMethod>);
    define_function!(pub get_local_signature, 6, *mut NetObject<SystemArray<u8>>, self: *mut Self);
    define_function!(pub get_exceptions, 7, *mut NetObject<SystemArray<u8>>, self: *mut Self);
    define_function!(pub get_code, 8, *mut NetObject<SystemArray<u8>>, self: *mut Self);
    define_function!(pub get_max_stack_size, 9, i32, self: *mut Self);
    define_function!(pub get_dynamic_method, 10, *mut NetObject<DynamicMethod>, self: *mut Self);
    define_function!(pub set_code_arr, 12, (), self: *mut Self, code : *mut NetObject<SystemArray<u8>>, max_stack_size : i32);
    define_function!(pub set_code_p, 13, (), self: *mut Self, code : *mut u8, code_size : i32, max_stack_size : i32);
    define_function!(pub set_exceptions_arr, 14, (), self: *mut Self, exceptions : *mut NetObject<SystemArray<u8>>);
    define_function!(pub set_exceptions_p, 15, (), self: *mut Self, exceptions : *mut u8, exceptions_size : i32);
    define_function!(pub set_local_signature_arr, 16, (), self: *mut Self, local_signature : *mut NetObject<SystemArray<u8>>);
    define_function!(pub set_local_signature_p, 17, (), self: *mut Self, local_signature : *mut u8, signature_size : i32);
    define_function!(pub get_token_for_dy, 19, i32, self: *mut Self, method : *mut NetObject<DynamicMethod>);
    define_function!(pub get_token_for_str, 24, i32, self: *mut Self, literal : *mut NetObject<SystemString>);
    define_function!(pub get_token_for_arr, 25, i32, self: *mut Self, signature : *mut NetObject<SystemArray<u8>>);
}

pub struct ILGenerator { }

define_typeof!(ILGenerator, "System.Reflection.Emit.ILGenerator");

impl ILGenerator {
}

type Label = i32;

impl NetObject<ILGenerator> {
    define_virtual!(pub emit, 0, 4, (), opcode : OpCode);
    define_virtual!(pub emit_u8, 0, 5, (), opcode : OpCode, arg : u8);
    define_virtual!(pub emit_i16, 0, 6, (), opcode : OpCode, arg : i16);
    define_virtual!(pub emit_i64, 0, 7, (), opcode : OpCode, arg : i64);
    define_virtual!(pub emit_f32, 1, 0, (), opcode : OpCode, arg : f32);
    define_virtual!(pub emit_f64, 1, 1, (), opcode : OpCode, arg : f64);
    define_virtual!(pub emit_i32, 1, 2, (), opcode : OpCode, arg : i32);
    define_virtual!(pub emit_mi, 1, 3, (), opcode : OpCode, meth : *mut NetObject<MethodInfo>);
    define_virtual!(pub emit_calli, 1, 4, (), opcode : OpCode, calling_convention : CallingConventions, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>, optional_parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub emit_calli_unm, 1, 5, (), opcode : OpCode, unmanaged_call_conv : CallingConventions, return_type : *mut NetObject<SystemType>, parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub emit_call, 1, 6, (), opcode : OpCode, method_info : *mut NetObject<MethodInfo>, optional_parameter_types : *mut NetObject<SystemArray<*mut NetObject<SystemType>>>);
    define_virtual!(pub emit_con, 2, 0, (), opcode : OpCode, con : *mut NetObject<ConstructorInfo>);
    define_virtual!(pub emit_cls, 2, 1, (), opcode : OpCode, cls : *mut NetObject<SystemType>);
    define_virtual!(pub emit_lab, 2, 2, (), opcode : OpCode, label : Label);
    define_virtual!(pub emit_labs, 2, 3, (), opcode : OpCode, labels : *mut NetObject<SystemArray<Label>>);
    define_virtual!(pub emit_fi, 2, 4, (), opcode : OpCode, field : *mut NetObject<FieldInfo>);
    define_virtual!(pub emit_str, 2, 5, (), opcode : OpCode, str : *mut NetObject<SystemString>);
    define_virtual!(pub emit_loc, 2, 6, (), opcode : OpCode, local : *mut NetObject<LocalBuilder>);
    define_virtual!(pub begin_exception_block, 2, 7, Label);
    define_virtual!(pub end_exception_block, 3, 0, ());
    define_virtual!(pub begin_except_filter_block, 3, 1, ());
    define_virtual!(pub begin_catch_block, 3, 2, (), exception_type : *mut NetObject<SystemType>);
    define_virtual!(pub begin_fault_block, 3, 3, ());
    define_virtual!(pub begin_finally_block, 3, 4, ());
    define_virtual!(pub define_label, 3, 5, Label);
    define_virtual!(pub mark_label, 3, 6, (), loc : Label);
    define_virtual!(pub throw_exception, 3, 7, (), exc_type : *mut NetObject<SystemType>);
    define_virtual!(pub emit_write_line_val, 4, 0, (), value : *mut NetObject<SystemString>);
    define_virtual!(pub emit_write_line_loc, 4, 1, (), local_builder : *mut NetObject<LocalBuilder>);
    define_virtual!(pub emit_write_line_fld, 4, 2, (), fld : *mut NetObject<FieldInfo>);
    define_virtual!(pub declare_local, 4, 3, *mut NetObject<LocalBuilder>, local_type : *mut NetObject<SystemType>);
    define_virtual!(pub declare_local_pin, 4, 4, *mut NetObject<LocalBuilder>, local_type : *mut NetObject<SystemType>, pinned : bool);
    define_virtual!(pub using_namespace, 4, 5, (), using_namespace : *mut NetObject<SystemString>);
    define_virtual!(pub begin_scope, 4, 6, ());
    define_virtual!(pub end_scope, 4, 7, ());
    define_virtual!(pub get_il_offset, 5, 0, i32);

    // Non-Virtual functions
    define_function!(pub emit_i8, 43, (), self: *mut Self, opcode : OpCode, arg : i8);
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct OpCodeValues: u32
    {
        const Nop = 0x00;
        const Break = 0x01;
        const Ldarg_0 = 0x02;
        const Ldarg_1 = 0x03;
        const Ldarg_2 = 0x04;
        const Ldarg_3 = 0x05;
        const Ldloc_0 = 0x06;
        const Ldloc_1 = 0x07;
        const Ldloc_2 = 0x08;
        const Ldloc_3 = 0x09;
        const Stloc_0 = 0x0a;
        const Stloc_1 = 0x0b;
        const Stloc_2 = 0x0c;
        const Stloc_3 = 0x0d;
        const Ldarg_S = 0x0e;
        const Ldarga_S = 0x0f;
        const Starg_S = 0x10;
        const Ldloc_S = 0x11;
        const Ldloca_S = 0x12;
        const Stloc_S = 0x13;
        const Ldnull = 0x14;
        const Ldc_I4_M1 = 0x15;
        const Ldc_I4_0 = 0x16;
        const Ldc_I4_1 = 0x17;
        const Ldc_I4_2 = 0x18;
        const Ldc_I4_3 = 0x19;
        const Ldc_I4_4 = 0x1a;
        const Ldc_I4_5 = 0x1b;
        const Ldc_I4_6 = 0x1c;
        const Ldc_I4_7 = 0x1d;
        const Ldc_I4_8 = 0x1e;
        const Ldc_I4_S = 0x1f;
        const Ldc_I4 = 0x20;
        const Ldc_I8 = 0x21;
        const Ldc_R4 = 0x22;
        const Ldc_R8 = 0x23;
        const Dup = 0x25;
        const Pop = 0x26;
        const Jmp = 0x27;
        const Call = 0x28;
        const Calli = 0x29;
        const Ret = 0x2a;
        const Br_S = 0x2b;
        const Brfalse_S = 0x2c;
        const Brtrue_S = 0x2d;
        const Beq_S = 0x2e;
        const Bge_S = 0x2f;
        const Bgt_S = 0x30;
        const Ble_S = 0x31;
        const Blt_S = 0x32;
        const Bne_Un_S = 0x33;
        const Bge_Un_S = 0x34;
        const Bgt_Un_S = 0x35;
        const Ble_Un_S = 0x36;
        const Blt_Un_S = 0x37;
        const Br = 0x38;
        const Brfalse = 0x39;
        const Brtrue = 0x3a;
        const Beq = 0x3b;
        const Bge = 0x3c;
        const Bgt = 0x3d;
        const Ble = 0x3e;
        const Blt = 0x3f;
        const Bne_Un = 0x40;
        const Bge_Un = 0x41;
        const Bgt_Un = 0x42;
        const Ble_Un = 0x43;
        const Blt_Un = 0x44;
        const Switch = 0x45;
        const Ldind_I1 = 0x46;
        const Ldind_U1 = 0x47;
        const Ldind_I2 = 0x48;
        const Ldind_U2 = 0x49;
        const Ldind_I4 = 0x4a;
        const Ldind_U4 = 0x4b;
        const Ldind_I8 = 0x4c;
        const Ldind_I = 0x4d;
        const Ldind_R4 = 0x4e;
        const Ldind_R8 = 0x4f;
        const Ldind_Ref = 0x50;
        const Stind_Ref = 0x51;
        const Stind_I1 = 0x52;
        const Stind_I2 = 0x53;
        const Stind_I4 = 0x54;
        const Stind_I8 = 0x55;
        const Stind_R4 = 0x56;
        const Stind_R8 = 0x57;
        const Add = 0x58;
        const Sub = 0x59;
        const Mul = 0x5a;
        const Div = 0x5b;
        const Div_Un = 0x5c;
        const Rem = 0x5d;
        const Rem_Un = 0x5e;
        const And = 0x5f;
        const Or = 0x60;
        const Xor = 0x61;
        const Shl = 0x62;
        const Shr = 0x63;
        const Shr_Un = 0x64;
        const Neg = 0x65;
        const Not = 0x66;
        const Conv_I1 = 0x67;
        const Conv_I2 = 0x68;
        const Conv_I4 = 0x69;
        const Conv_I8 = 0x6a;
        const Conv_R4 = 0x6b;
        const Conv_R8 = 0x6c;
        const Conv_U4 = 0x6d;
        const Conv_U8 = 0x6e;
        const Callvirt = 0x6f;
        const Cpobj = 0x70;
        const Ldobj = 0x71;
        const Ldstr = 0x72;
        const Newobj = 0x73;
        const Castclass = 0x74;
        const Isinst = 0x75;
        const Conv_R_Un = 0x76;
        const Unbox = 0x79;
        const Throw = 0x7a;
        const Ldfld = 0x7b;
        const Ldflda = 0x7c;
        const Stfld = 0x7d;
        const Ldsfld = 0x7e;
        const Ldsflda = 0x7f;
        const Stsfld = 0x80;
        const Stobj = 0x81;
        const Conv_Ovf_I1_Un = 0x82;
        const Conv_Ovf_I2_Un = 0x83;
        const Conv_Ovf_I4_Un = 0x84;
        const Conv_Ovf_I8_Un = 0x85;
        const Conv_Ovf_U1_Un = 0x86;
        const Conv_Ovf_U2_Un = 0x87;
        const Conv_Ovf_U4_Un = 0x88;
        const Conv_Ovf_U8_Un = 0x89;
        const Conv_Ovf_I_Un = 0x8a;
        const Conv_Ovf_U_Un = 0x8b;
        const Box = 0x8c;
        const Newarr = 0x8d;
        const Ldlen = 0x8e;
        const Ldelema = 0x8f;
        const Ldelem_I1 = 0x90;
        const Ldelem_U1 = 0x91;
        const Ldelem_I2 = 0x92;
        const Ldelem_U2 = 0x93;
        const Ldelem_I4 = 0x94;
        const Ldelem_U4 = 0x95;
        const Ldelem_I8 = 0x96;
        const Ldelem_I = 0x97;
        const Ldelem_R4 = 0x98;
        const Ldelem_R8 = 0x99;
        const Ldelem_Ref = 0x9a;
        const Stelem_I = 0x9b;
        const Stelem_I1 = 0x9c;
        const Stelem_I2 = 0x9d;
        const Stelem_I4 = 0x9e;
        const Stelem_I8 = 0x9f;
        const Stelem_R4 = 0xa0;
        const Stelem_R8 = 0xa1;
        const Stelem_Ref = 0xa2;
        const Ldelem = 0xa3;
        const Stelem = 0xa4;
        const Unbox_Any = 0xa5;
        const Conv_Ovf_I1 = 0xb3;
        const Conv_Ovf_U1 = 0xb4;
        const Conv_Ovf_I2 = 0xb5;
        const Conv_Ovf_U2 = 0xb6;
        const Conv_Ovf_I4 = 0xb7;
        const Conv_Ovf_U4 = 0xb8;
        const Conv_Ovf_I8 = 0xb9;
        const Conv_Ovf_U8 = 0xba;
        const Refanyval = 0xc2;
        const Ckfinite = 0xc3;
        const Mkrefany = 0xc6;
        const Ldtoken = 0xd0;
        const Conv_U2 = 0xd1;
        const Conv_U1 = 0xd2;
        const Conv_I = 0xd3;
        const Conv_Ovf_I = 0xd4;
        const Conv_Ovf_U = 0xd5;
        const Add_Ovf = 0xd6;
        const Add_Ovf_Un = 0xd7;
        const Mul_Ovf = 0xd8;
        const Mul_Ovf_Un = 0xd9;
        const Sub_Ovf = 0xda;
        const Sub_Ovf_Un = 0xdb;
        const Endfinally = 0xdc;
        const Leave = 0xdd;
        const Leave_S = 0xde;
        const Stind_I = 0xdf;
        const Conv_U = 0xe0;
        const Prefix7 = 0xf8;
        const Prefix6 = 0xf9;
        const Prefix5 = 0xfa;
        const Prefix4 = 0xfb;
        const Prefix3 = 0xfc;
        const Prefix2 = 0xfd;
        const Prefix1 = 0xfe;
        const Prefixref = 0xff;
        const Arglist = 0xfe00;
        const Ceq = 0xfe01;
        const Cgt = 0xfe02;
        const Cgt_Un = 0xfe03;
        const Clt = 0xfe04;
        const Clt_Un = 0xfe05;
        const Ldftn = 0xfe06;
        const Ldvirtftn = 0xfe07;
        const Ldarg = 0xfe09;
        const Ldarga = 0xfe0a;
        const Starg = 0xfe0b;
        const Ldloc = 0xfe0c;
        const Ldloca = 0xfe0d;
        const Stloc = 0xfe0e;
        const Localloc = 0xfe0f;
        const Endfilter = 0xfe11;
        const Unaligned_ = 0xfe12;
        const Volatile_ = 0xfe13;
        const Tail_ = 0xfe14;
        const Initobj = 0xfe15;
        const Constrained_ = 0xfe16;
        const Cpblk = 0xfe17;
        const Initblk = 0xfe18;
        const Rethrow = 0xfe1a;
        const Sizeof = 0xfe1c;
        const Refanytype = 0xfe1d;
        const Readonly_ = 0xfe1e;
    }
}

#[derive(Clone, Copy)]
pub struct OpCode { 
    value: OpCodeValues,
    flags: u32
}

define_typeof!(OpCode, "System.Reflection.Emit.OpCode");

impl OpCode {
    // Virtual functions
    define_virtual!(pub equals, 0, 4, bool, obj : OpCode);

    // Non-Virtual functions
    define_function!(pub ends_uncond_jmp_blk, 6, bool, self: *mut Self);
    define_function!(pub stack_change, 7, i32, self: *mut Self);
    define_function!(pub get_operand_type, 8, OperandType, self: *mut Self);
    define_function!(pub get_flow_control, 9, FlowControl, self: *mut Self);
    define_function!(pub get_op_code_type, 10, OpCodeType, self: *mut Self);
    define_function!(pub get_stack_behaviour_pop, 11, StackBehaviour, self: *mut Self);
    define_function!(pub get_stack_behaviour_push, 12, StackBehaviour, self: *mut Self);
    define_function!(pub get_size, 13, i32, self: *mut Self);
    define_function!(pub get_value, 14, i16, self: *mut Self);
    define_function!(pub get_name, 15, *mut NetObject<SystemString>, self: *mut Self);
    define_function!(pub op_equality, 16, bool, a : OpCode, b : OpCode);
    define_function!(pub op_inequality, 17, bool, a : OpCode, b : OpCode);
}

#[repr(u32)]
pub enum OpCodeType {
    Macro,
    Nternal,
    Objmodel,
    Prefix,
    Primitive
}

#[repr(u32)]
pub enum StackBehaviour
{
  Pop0,
  Pop1,
  Pop1Pop1,
  Popi,
  PopiPop1,
  Popipopi,
  Popipopi8,
  Popipopipopi,
  Popipopr4,
  Popipopr8,
  Popref,
  Poprefpop1,
  Poprefpopi,
  Poprefpopipopi,
  Poprefpopipopi8,
  Poprefpopipopr4,
  Poprefpopipopr8,
  Poprefpopipopref,
  Push0,
  Push1,
  Push1push1,
  Pushi,
  Pushi8,
  Pushr4,
  Pushr8,
  Pushref,
  Arpop,
  Varpush,
  Poprefpopipop1,
}

#[repr(u32)]
pub enum OperandType
{
  InlineBrTarget = 0,
  InlineField = 1,
  InlineI = 2,
  InlineI8 = 3,
  InlineMethod = 4,
  InlineNone = 5,
  InlinePhi = 6,
  InlineR = 7,
  InlineSig = 9,
  InlineString = 10, // 0x0000000A
  InlineSwitch = 11, // 0x0000000B
  InlineTok = 12, // 0x0000000C
  InlineType = 13, // 0x0000000D
  InlineVar = 14, // 0x0000000E
  ShortInlineBrTarget = 15, // 0x0000000F
  ShortInlineI = 16, // 0x00000010
  ShortInlineR = 17, // 0x00000011
  ShortInlineVar = 18, // 0x00000012
}

#[repr(u32)]
pub enum FlowControl
{
  Branch,
  Break,
  Call,
  CondBranch,
  Meta,
  Next,
  Phi,
  Return,
  Throw,
}

pub struct LocalBuilder { }

define_typeof!(LocalBuilder, "System.Reflection.Emit.LocalBuilder");

impl NetObject<LocalBuilder> {
    define_virtual!(pub get_local_type, 0, 4, *mut NetObject<SystemType>);
    //define_virtual!(pub get_local_index, 0, 5, i32);
    define_virtual!(pub get_is_pinned, 0, 6, bool);

    define_function!(pub get_local_index, 8, i32, self: *mut Self);
    define_function!(pub get_method_builder, 9, *mut NetObject<MethodInfo>, self: *mut Self);
}

pub struct ParameterBuilder { }

define_typeof!(ParameterBuilder, "System.Reflection.Emit.ParameterBuilder");

impl NetObject<ParameterBuilder> {
    // Virtual functions
    define_virtual!(pub get_attributes, 0, 4, i32);
    define_virtual!(pub get_name, 0, 5, *mut NetObject<SystemString>);
    define_virtual!(pub get_position, 0, 6, i32);
    define_virtual!(pub set_constant, 0, 7, (), default_value : *mut NetObject<SystemObject>);

    // Non-Virtual functions
    define_function!(pub get_is_in, 10, bool, self: *mut Self);
    define_function!(pub get_is_optional, 11, bool, self: *mut Self);
    define_function!(pub get_is_out, 12, bool, self: *mut Self);
    define_function!(pub set_custom_attribute, 13, (), self: *mut Self, con : *mut NetObject<ConstructorInfo>, binary_attribute : *mut NetObject<SystemArray<u8>>);
}

pub struct OpCodes {}
define_typeof!(OpCodes, "System.Reflection.Emit.OpCodes");
impl OpCodes {
    define_static_field!(pub get_nop, "Nop", OpCode, unbox);
    define_static_field!(pub get_break, "Break", OpCode, unbox);
    define_static_field!(pub get_ldarg_0, "Ldarg_0", OpCode, unbox);
    define_static_field!(pub get_ldarg_1, "Ldarg_1", OpCode, unbox);
    define_static_field!(pub get_ldarg_2, "Ldarg_2", OpCode, unbox);
    define_static_field!(pub get_ldarg_3, "Ldarg_3", OpCode, unbox);
    define_static_field!(pub get_ldloc_0, "Ldloc_0", OpCode, unbox);
    define_static_field!(pub get_ldloc_1, "Ldloc_1", OpCode, unbox);
    define_static_field!(pub get_ldloc_2, "Ldloc_2", OpCode, unbox);
    define_static_field!(pub get_ldloc_3, "Ldloc_3", OpCode, unbox);
    define_static_field!(pub get_stloc_0, "Stloc_0", OpCode, unbox);
    define_static_field!(pub get_stloc_1, "Stloc_1", OpCode, unbox);
    define_static_field!(pub get_stloc_2, "Stloc_2", OpCode, unbox);
    define_static_field!(pub get_stloc_3, "Stloc_3", OpCode, unbox);
    define_static_field!(pub get_ldarg_s, "Ldarg_S", OpCode, unbox);
    define_static_field!(pub get_ldarga_s, "Ldarga_S", OpCode, unbox);
    define_static_field!(pub get_starg_s, "Starg_S", OpCode, unbox);
    define_static_field!(pub get_ldloc_s, "Ldloc_S", OpCode, unbox);
    define_static_field!(pub get_ldloca_s, "Ldloca_S", OpCode, unbox);
    define_static_field!(pub get_stloc_s, "Stloc_S", OpCode, unbox);
    define_static_field!(pub get_ldnull, "Ldnull", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_m1, "Ldc_I4_M1", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_0, "Ldc_I4_0", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_1, "Ldc_I4_1", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_2, "Ldc_I4_2", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_3, "Ldc_I4_3", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_4, "Ldc_I4_4", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_5, "Ldc_I4_5", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_6, "Ldc_I4_6", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_7, "Ldc_I4_7", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_8, "Ldc_I4_8", OpCode, unbox);
    define_static_field!(pub get_ldc_i4_s, "Ldc_I4_S", OpCode, unbox);
    define_static_field!(pub get_ldc_i4, "Ldc_I4", OpCode, unbox);
    define_static_field!(pub get_ldc_i8, "Ldc_I8", OpCode, unbox);
    define_static_field!(pub get_ldc_r4, "Ldc_R4", OpCode, unbox);
    define_static_field!(pub get_ldc_r8, "Ldc_R8", OpCode, unbox);
    define_static_field!(pub get_dup, "Dup", OpCode, unbox);
    define_static_field!(pub get_pop, "Pop", OpCode, unbox);
    define_static_field!(pub get_jmp, "Jmp", OpCode, unbox);
    define_static_field!(pub get_call, "Call", OpCode, unbox);
    define_static_field!(pub get_calli, "Calli", OpCode, unbox);
    define_static_field!(pub get_ret, "Ret", OpCode, unbox);
    define_static_field!(pub get_br_s, "Br_S", OpCode, unbox);
    define_static_field!(pub get_brfalse_s, "Brfalse_S", OpCode, unbox);
    define_static_field!(pub get_brtrue_s, "Brtrue_S", OpCode, unbox);
    define_static_field!(pub get_beq_s, "Beq_S", OpCode, unbox);
    define_static_field!(pub get_bge_s, "Bge_S", OpCode, unbox);
    define_static_field!(pub get_bgt_s, "Bgt_S", OpCode, unbox);
    define_static_field!(pub get_ble_s, "Ble_S", OpCode, unbox);
    define_static_field!(pub get_blt_s, "Blt_S", OpCode, unbox);
    define_static_field!(pub get_bne_un_s, "Bne_Un_S", OpCode, unbox);
    define_static_field!(pub get_bge_un_s, "Bge_Un_S", OpCode, unbox);
    define_static_field!(pub get_bgt_un_s, "Bgt_Un_S", OpCode, unbox);
    define_static_field!(pub get_ble_un_s, "Ble_Un_S", OpCode, unbox);
    define_static_field!(pub get_blt_un_s, "Blt_Un_S", OpCode, unbox);
    define_static_field!(pub get_br, "Br", OpCode, unbox);
    define_static_field!(pub get_brfalse, "Brfalse", OpCode, unbox);
    define_static_field!(pub get_brtrue, "Brtrue", OpCode, unbox);
    define_static_field!(pub get_beq, "Beq", OpCode, unbox);
    define_static_field!(pub get_bge, "Bge", OpCode, unbox);
    define_static_field!(pub get_bgt, "Bgt", OpCode, unbox);
    define_static_field!(pub get_ble, "Ble", OpCode, unbox);
    define_static_field!(pub get_blt, "Blt", OpCode, unbox);
    define_static_field!(pub get_bne_un, "Bne_Un", OpCode, unbox);
    define_static_field!(pub get_bge_un, "Bge_Un", OpCode, unbox);
    define_static_field!(pub get_bgt_un, "Bgt_Un", OpCode, unbox);
    define_static_field!(pub get_ble_un, "Ble_Un", OpCode, unbox);
    define_static_field!(pub get_blt_un, "Blt_Un", OpCode, unbox);
    define_static_field!(pub get_switch, "Switch", OpCode, unbox);
    define_static_field!(pub get_ldind_i1, "Ldind_I1", OpCode, unbox);
    define_static_field!(pub get_ldind_u1, "Ldind_U1", OpCode, unbox);
    define_static_field!(pub get_ldind_i2, "Ldind_I2", OpCode, unbox);
    define_static_field!(pub get_ldind_u2, "Ldind_U2", OpCode, unbox);
    define_static_field!(pub get_ldind_i4, "Ldind_I4", OpCode, unbox);
    define_static_field!(pub get_ldind_u4, "Ldind_U4", OpCode, unbox);
    define_static_field!(pub get_ldind_i8, "Ldind_I8", OpCode, unbox);
    define_static_field!(pub get_ldind_i, "Ldind_I", OpCode, unbox);
    define_static_field!(pub get_ldind_r4, "Ldind_R4", OpCode, unbox);
    define_static_field!(pub get_ldind_r8, "Ldind_R8", OpCode, unbox);
    define_static_field!(pub get_ldind_ref, "Ldind_Ref", OpCode, unbox);
    define_static_field!(pub get_stind_ref, "Stind_Ref", OpCode, unbox);
    define_static_field!(pub get_stind_i1, "Stind_I1", OpCode, unbox);
    define_static_field!(pub get_stind_i2, "Stind_I2", OpCode, unbox);
    define_static_field!(pub get_stind_i4, "Stind_I4", OpCode, unbox);
    define_static_field!(pub get_stind_i8, "Stind_I8", OpCode, unbox);
    define_static_field!(pub get_stind_r4, "Stind_R4", OpCode, unbox);
    define_static_field!(pub get_stind_r8, "Stind_R8", OpCode, unbox);
    define_static_field!(pub get_add, "Add", OpCode, unbox);
    define_static_field!(pub get_sub, "Sub", OpCode, unbox);
    define_static_field!(pub get_mul, "Mul", OpCode, unbox);
    define_static_field!(pub get_div, "Div", OpCode, unbox);
    define_static_field!(pub get_div_un, "Div_Un", OpCode, unbox);
    define_static_field!(pub get_rem, "Rem", OpCode, unbox);
    define_static_field!(pub get_rem_un, "Rem_Un", OpCode, unbox);
    define_static_field!(pub get_and, "And", OpCode, unbox);
    define_static_field!(pub get_or, "Or", OpCode, unbox);
    define_static_field!(pub get_xor, "Xor", OpCode, unbox);
    define_static_field!(pub get_shl, "Shl", OpCode, unbox);
    define_static_field!(pub get_shr, "Shr", OpCode, unbox);
    define_static_field!(pub get_shr_un, "Shr_Un", OpCode, unbox);
    define_static_field!(pub get_neg, "Neg", OpCode, unbox);
    define_static_field!(pub get_not, "Not", OpCode, unbox);
    define_static_field!(pub get_conv_i1, "Conv_I1", OpCode, unbox);
    define_static_field!(pub get_conv_i2, "Conv_I2", OpCode, unbox);
    define_static_field!(pub get_conv_i4, "Conv_I4", OpCode, unbox);
    define_static_field!(pub get_conv_i8, "Conv_I8", OpCode, unbox);
    define_static_field!(pub get_conv_r4, "Conv_R4", OpCode, unbox);
    define_static_field!(pub get_conv_r8, "Conv_R8", OpCode, unbox);
    define_static_field!(pub get_conv_u4, "Conv_U4", OpCode, unbox);
    define_static_field!(pub get_conv_u8, "Conv_U8", OpCode, unbox);
    define_static_field!(pub get_callvirt, "Callvirt", OpCode, unbox);
    define_static_field!(pub get_cpobj, "Cpobj", OpCode, unbox);
    define_static_field!(pub get_ldobj, "Ldobj", OpCode, unbox);
    define_static_field!(pub get_ldstr, "Ldstr", OpCode, unbox);
    define_static_field!(pub get_newobj, "Newobj", OpCode, unbox);
    define_static_field!(pub get_castclass, "Castclass", OpCode, unbox);
    define_static_field!(pub get_isinst, "Isinst", OpCode, unbox);
    define_static_field!(pub get_conv_r_un, "Conv_R_Un", OpCode, unbox);
    define_static_field!(pub get_unbox, "Unbox", OpCode, unbox);
    define_static_field!(pub get_throw, "Throw", OpCode, unbox);
    define_static_field!(pub get_ldfld, "Ldfld", OpCode, unbox);
    define_static_field!(pub get_ldflda, "Ldflda", OpCode, unbox);
    define_static_field!(pub get_stfld, "Stfld", OpCode, unbox);
    define_static_field!(pub get_ldsfld, "Ldsfld", OpCode, unbox);
    define_static_field!(pub get_ldsflda, "Ldsflda", OpCode, unbox);
    define_static_field!(pub get_stsfld, "Stsfld", OpCode, unbox);
    define_static_field!(pub get_stobj, "Stobj", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i1_un, "Conv_Ovf_I1_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i2_un, "Conv_Ovf_I2_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i4_un, "Conv_Ovf_I4_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i8_un, "Conv_Ovf_I8_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u1_un, "Conv_Ovf_U1_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u2_un, "Conv_Ovf_U2_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u4_un, "Conv_Ovf_U4_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u8_un, "Conv_Ovf_U8_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i_un, "Conv_Ovf_I_Un", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u_un, "Conv_Ovf_U_Un", OpCode, unbox);
    define_static_field!(pub get_box, "Box", OpCode, unbox);
    define_static_field!(pub get_newarr, "Newarr", OpCode, unbox);
    define_static_field!(pub get_ldlen, "Ldlen", OpCode, unbox);
    define_static_field!(pub get_ldelema, "Ldelema", OpCode, unbox);
    define_static_field!(pub get_ldelem_i1, "Ldelem_I1", OpCode, unbox);
    define_static_field!(pub get_ldelem_u1, "Ldelem_U1", OpCode, unbox);
    define_static_field!(pub get_ldelem_i2, "Ldelem_I2", OpCode, unbox);
    define_static_field!(pub get_ldelem_u2, "Ldelem_U2", OpCode, unbox);
    define_static_field!(pub get_ldelem_i4, "Ldelem_I4", OpCode, unbox);
    define_static_field!(pub get_ldelem_u4, "Ldelem_U4", OpCode, unbox);
    define_static_field!(pub get_ldelem_i8, "Ldelem_I8", OpCode, unbox);
    define_static_field!(pub get_ldelem_i, "Ldelem_I", OpCode, unbox);
    define_static_field!(pub get_ldelem_r4, "Ldelem_R4", OpCode, unbox);
    define_static_field!(pub get_ldelem_r8, "Ldelem_R8", OpCode, unbox);
    define_static_field!(pub get_ldelem_ref, "Ldelem_Ref", OpCode, unbox);
    define_static_field!(pub get_stelem_i, "Stelem_I", OpCode, unbox);
    define_static_field!(pub get_stelem_i1, "Stelem_I1", OpCode, unbox);
    define_static_field!(pub get_stelem_i2, "Stelem_I2", OpCode, unbox);
    define_static_field!(pub get_stelem_i4, "Stelem_I4", OpCode, unbox);
    define_static_field!(pub get_stelem_i8, "Stelem_I8", OpCode, unbox);
    define_static_field!(pub get_stelem_r4, "Stelem_R4", OpCode, unbox);
    define_static_field!(pub get_stelem_r8, "Stelem_R8", OpCode, unbox);
    define_static_field!(pub get_stelem_ref, "Stelem_Ref", OpCode, unbox);
    define_static_field!(pub get_ldelem, "Ldelem", OpCode, unbox);
    define_static_field!(pub get_stelem, "Stelem", OpCode, unbox);
    define_static_field!(pub get_unbox_any, "Unbox_Any", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i1, "Conv_Ovf_I1", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u1, "Conv_Ovf_U1", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i2, "Conv_Ovf_I2", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u2, "Conv_Ovf_U2", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i4, "Conv_Ovf_I4", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u4, "Conv_Ovf_U4", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i8, "Conv_Ovf_I8", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u8, "Conv_Ovf_U8", OpCode, unbox);
    define_static_field!(pub get_refanyval, "Refanyval", OpCode, unbox);
    define_static_field!(pub get_ckfinite, "Ckfinite", OpCode, unbox);
    define_static_field!(pub get_mkrefany, "Mkrefany", OpCode, unbox);
    define_static_field!(pub get_ldtoken, "Ldtoken", OpCode, unbox);
    define_static_field!(pub get_conv_u2, "Conv_U2", OpCode, unbox);
    define_static_field!(pub get_conv_u1, "Conv_U1", OpCode, unbox);
    define_static_field!(pub get_conv_i, "Conv_I", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_i, "Conv_Ovf_I", OpCode, unbox);
    define_static_field!(pub get_conv_ovf_u, "Conv_Ovf_U", OpCode, unbox);
    define_static_field!(pub get_add_ovf, "Add_Ovf", OpCode, unbox);
    define_static_field!(pub get_add_ovf_un, "Add_Ovf_Un", OpCode, unbox);
    define_static_field!(pub get_mul_ovf, "Mul_Ovf", OpCode, unbox);
    define_static_field!(pub get_mul_ovf_un, "Mul_Ovf_Un", OpCode, unbox);
    define_static_field!(pub get_sub_ovf, "Sub_Ovf", OpCode, unbox);
    define_static_field!(pub get_sub_ovf_un, "Sub_Ovf_Un", OpCode, unbox);
    define_static_field!(pub get_endfinally, "Endfinally", OpCode, unbox);
    define_static_field!(pub get_leave, "Leave", OpCode, unbox);
    define_static_field!(pub get_leave_s, "Leave_S", OpCode, unbox);
    define_static_field!(pub get_stind_i, "Stind_I", OpCode, unbox);
    define_static_field!(pub get_conv_u, "Conv_U", OpCode, unbox);
    define_static_field!(pub get_prefix7, "Prefix7", OpCode, unbox);
    define_static_field!(pub get_prefix6, "Prefix6", OpCode, unbox);
    define_static_field!(pub get_prefix5, "Prefix5", OpCode, unbox);
    define_static_field!(pub get_prefix4, "Prefix4", OpCode, unbox);
    define_static_field!(pub get_prefix3, "Prefix3", OpCode, unbox);
    define_static_field!(pub get_prefix2, "Prefix2", OpCode, unbox);
    define_static_field!(pub get_prefix1, "Prefix1", OpCode, unbox);
    define_static_field!(pub get_prefixref, "Prefixref", OpCode, unbox);
    define_static_field!(pub get_arglist, "Arglist", OpCode, unbox);
    define_static_field!(pub get_ceq, "Ceq", OpCode, unbox);
    define_static_field!(pub get_cgt, "Cgt", OpCode, unbox);
    define_static_field!(pub get_cgt_un, "Cgt_Un", OpCode, unbox);
    define_static_field!(pub get_clt, "Clt", OpCode, unbox);
    define_static_field!(pub get_clt_un, "Clt_Un", OpCode, unbox);
    define_static_field!(pub get_ldftn, "Ldftn", OpCode, unbox);
    define_static_field!(pub get_ldvirtftn, "Ldvirtftn", OpCode, unbox);
    define_static_field!(pub get_ldarg, "Ldarg", OpCode, unbox);
    define_static_field!(pub get_ldarga, "Ldarga", OpCode, unbox);
    define_static_field!(pub get_starg, "Starg", OpCode, unbox);
    define_static_field!(pub get_ldloc, "Ldloc", OpCode, unbox);
    define_static_field!(pub get_ldloca, "Ldloca", OpCode, unbox);
    define_static_field!(pub get_stloc, "Stloc", OpCode, unbox);
    define_static_field!(pub get_localloc, "Localloc", OpCode, unbox);
    define_static_field!(pub get_endfilter, "Endfilter", OpCode, unbox);
    define_static_field!(pub get_unaligned, "Unaligned", OpCode, unbox);
    define_static_field!(pub get_volatile, "Volatile", OpCode, unbox);
    define_static_field!(pub get_tailcall, "Tailcall", OpCode, unbox);
    define_static_field!(pub get_initobj, "Initobj", OpCode, unbox);
    define_static_field!(pub get_constrained, "Constrained", OpCode, unbox);
    define_static_field!(pub get_cpblk, "Cpblk", OpCode, unbox);
    define_static_field!(pub get_initblk, "Initblk", OpCode, unbox);
    define_static_field!(pub get_rethrow, "Rethrow", OpCode, unbox);
    define_static_field!(pub get_sizeof, "Sizeof", OpCode, unbox);
    define_static_field!(pub get_refanytype, "Refanytype", OpCode, unbox);
    define_static_field!(pub get_readonly, "Readonly", OpCode, unbox);
}