use crate::define_typeof;

use super::AutoStructBox;

define_typeof!(bool, "System.Boolean"); impl AutoStructBox for bool {}

define_typeof!(i8, "System.SByte"); impl AutoStructBox for i8 {}
define_typeof!(i16, "System.Int16"); impl AutoStructBox for i16 {}
define_typeof!(i32, "System.Int32"); impl AutoStructBox for i32 {}
define_typeof!(i64, "System.Int64"); impl AutoStructBox for i64 {} 
define_typeof!(i128, "System.Int128"); impl AutoStructBox for i128 {}
define_typeof!(isize, "System.IntPtr"); impl AutoStructBox for isize {}

define_typeof!(u8, "System.Byte"); impl AutoStructBox for u8 {}
define_typeof!(u16, "System.UInt16"); impl AutoStructBox for u16 {}
define_typeof!(u32, "System.UInt32"); impl AutoStructBox for u32 {}
define_typeof!(u64, "System.UInt64"); impl AutoStructBox for u64 {}
define_typeof!(u128, "System.UInt128"); impl AutoStructBox for u128 {}
define_typeof!(usize, "System.UIntPtr"); impl AutoStructBox for usize {}

define_typeof!(f32, "System.Single"); impl AutoStructBox for f32 {} 
define_typeof!(f64, "System.Double"); impl AutoStructBox for f64 {}