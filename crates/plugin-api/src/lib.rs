pub const PLUGIN_ABI_VERSION: u32 = 2;

mod descriptors;
mod types;

pub use descriptors::{DocumentPluginDescriptor, PluginDescriptor};
pub use types::{DocumentInput, DocumentKind, InputBuffer, PluginString};
