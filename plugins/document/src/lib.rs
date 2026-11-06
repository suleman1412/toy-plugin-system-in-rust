use plugin_api::{DocumentInput, DocumentPluginDescriptor, PLUGIN_ABI_VERSION};
use std::os::raw::c_char;

static PLUGIN_NAME: &[u8] = b"document\0";

#[unsafe(no_mangle)]
pub extern "C" fn document_plugin_entry() -> DocumentPluginDescriptor {
    DocumentPluginDescriptor {
        abi_version: PLUGIN_ABI_VERSION,
        name: PLUGIN_NAME.as_ptr().cast::<c_char>(),
        count_tags,
    }
}

unsafe extern "C" fn count_tags(input: DocumentInput) -> usize {
    unsafe { input.tag_slice() }.len()
}
