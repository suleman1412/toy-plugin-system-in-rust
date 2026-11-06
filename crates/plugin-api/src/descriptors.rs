use core::ffi::c_char;

use crate::types::{DocumentInput, InputBuffer, PluginString};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginDescriptor {
    pub abi_version: u32,
    pub name: *const c_char,
    pub transform: unsafe extern "C" fn(input: InputBuffer) -> PluginString,
    pub free_string: unsafe extern "C" fn(value: PluginString),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DocumentPluginDescriptor {
    pub abi_version: u32,
    pub name: *const c_char,
    pub count_tags: unsafe extern "C" fn(input: DocumentInput) -> usize,
}
