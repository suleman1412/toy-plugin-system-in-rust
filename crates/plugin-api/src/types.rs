use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputBuffer {
    pub ptr: *const u8,
    pub len: usize,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DocumentKind {
    Article = 0,
    Note = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DocumentInput {
    pub title: InputBuffer,
    pub tags: *const InputBuffer,
    pub tag_count: usize,
    pub kind: DocumentKind,
}

impl DocumentInput {
    pub unsafe fn tag_slice(&self) -> &[InputBuffer] {
        if self.tag_count == 0 {
            return &[];
        }

        assert!(!self.tags.is_null());
        unsafe { std::slice::from_raw_parts(self.tags, self.tag_count) }
    }
}

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
