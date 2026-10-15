use plugin_api::{PluginDescriptor, PluginString, PLUGIN_ABI_VERSION};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

static PLUGIN_NAME : &[u8] = b"size\0";
#[unsafe(no_mangle)]
pub extern "C" fn plugin_entry() -> PluginDescriptor{
    PluginDescriptor{
        abi_version: PLUGIN_ABI_VERSION,
        name: PLUGIN_NAME.as_ptr().cast::<c_char>(),
        transform,
        free_string,
    }
}

unsafe extern "C" fn transform(input_ptr: *const c_char) -> PluginString {
    if input_ptr.is_null(){
        return PluginString::null();
    }
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_string_lossy();
    let count = input.chars().count();

    match CString::new(count.to_string()){
        Ok(output) => PluginString{
            ptr: output.into_raw(),
        },
        Err(_) => PluginString::null()
    }
}

unsafe extern "C" fn free_string(value: PluginString) {
    if !value.ptr.is_null() {
        let _ = unsafe { CString::from_raw(value.ptr) };
    }
}
