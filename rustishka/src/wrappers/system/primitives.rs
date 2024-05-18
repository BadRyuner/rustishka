use crate::define_typeof;

define_typeof!(bool, "System.Boolean");

define_typeof!(i8, "System.SByte");
define_typeof!(i16, "System.Int16");
define_typeof!(i32, "System.Int32");
define_typeof!(i64, "System.Int64");
define_typeof!(i128, "System.Int128");
define_typeof!(isize, "System.IntPtr");

define_typeof!(u8, "System.Byte");
define_typeof!(u16, "System.UInt16");
define_typeof!(u32, "System.UInt32");
define_typeof!(u64, "System.UInt64");
define_typeof!(u128, "System.UInt128");
define_typeof!(usize, "System.UIntPtr");

define_typeof!(f32, "System.Single");
define_typeof!(f64, "System.Double");