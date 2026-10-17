use plugin_api::{InputBuffer, PluginDescriptor, PluginString, PLUGIN_ABI_VERSION};
use std::ffi::CString;
use std::os::raw::c_char;

static PLUGIN_NAME: &[u8] = b"uppercase\0";

#[unsafe(no_mangle)]
pub extern "C" fn plugin_entry() -> PluginDescriptor {
    PluginDescriptor {
        abi_version: PLUGIN_ABI_VERSION,
        name: PLUGIN_NAME.as_ptr().cast::<c_char>(),
        transform,
        free_string,
    }
}

unsafe extern "C" fn transform(input: InputBuffer) -> PluginString {
    if input.ptr.is_null() {
        return PluginString::null();
    }

    let input = unsafe { std::slice::from_raw_parts(input.ptr, input.len) };
    let input = String::from_utf8_lossy(input);
    let res = match CString::new(input.to_uppercase()) {
        Ok(output) => PluginString {
            ptr: output.into_raw(),
        },
        Err(_) => PluginString::null(),
    };
    println!("{:?}", res.ptr);
    res
}

unsafe extern "C" fn free_string(value: PluginString) {
    if !value.ptr.is_null() {
        let _ = unsafe { CString::from_raw(value.ptr) };
    }
}
