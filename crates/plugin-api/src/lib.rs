use core::ffi::c_char;

pub const PLUGIN_ABI_VERSION: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginString {
    pub ptr: *mut c_char,
}

impl PluginString {
    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginDescriptor {
    pub abi_version: u32,
    pub name: *const c_char,
    pub transform: unsafe extern "C" fn(input_ptr: *const c_char) -> PluginString,
    pub free_string: unsafe extern "C" fn(value: PluginString),
}
