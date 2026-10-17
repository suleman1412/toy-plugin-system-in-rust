use libloading::{Library, Symbol};
use plugin_api::{InputBuffer, PluginDescriptor, PLUGIN_ABI_VERSION};
use std::env;
use std::error::Error;
use std::ffi::{CStr};
use std::path::PathBuf;

type PluginEntry = unsafe extern "C" fn() -> PluginDescriptor;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        eprintln!("usage: cargo run -p host -- <plugin-library-path> <input>");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let plugin_path = args
        .next()
        .map(PathBuf::from)
        .ok_or("missing plugin library path")?;
    let input = args
        .next()
        .ok_or("missing input string")?
        .to_string_lossy()
        .into_owned();

    let input_buffer = InputBuffer {
        ptr: input.as_ptr(),
        len: input.len(),
    };

    println!("Host ptr: {:?}", input_buffer.ptr);
    unsafe {
        let library = Library::new(plugin_path)?;
        let entry: Symbol<PluginEntry> = library.get(b"plugin_entry")?;
        let descriptor = entry();

        if descriptor.abi_version != PLUGIN_ABI_VERSION {
            return Err(format!(
                "unsupported plugin ABI version {}, expected {}",
                descriptor.abi_version, PLUGIN_ABI_VERSION
            )
            .into());
        }
        if descriptor.name.is_null() {
            return Err("plugin returned a null name".into());
        }

        let name = CStr::from_ptr(descriptor.name).to_string_lossy();
        let output = (descriptor.transform)(input_buffer);

        if output.ptr.is_null() {
            return Err(format!("plugin {name} returned a null output string").into());
        }

        let output_text = CStr::from_ptr(output.ptr).to_string_lossy().into_owned();
        (descriptor.free_string)(output);

        println!("Loaded plugin: {name}");
        println!("Output: {output_text}");

        drop(library);
    }

    Ok(())
}
