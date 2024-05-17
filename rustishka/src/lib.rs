#![feature(arbitrary_self_types)]
#![allow(dead_code)]

use std::sync::OnceLock;

use imports::DotnetImportsContainer;

pub mod wrappers;
pub mod imports;

pub static mut DOTNET_RUNTIME: OnceLock<DotnetImportsContainer> = OnceLock::new();