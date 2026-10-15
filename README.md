# Toy Native Plugin System

This project shows a Rust host that loads native plugins at run time.

The host and plugins use a C ABI. They do not pass Rust types across the plugin boundary.

## Project Layout

```text
crates/host/          Host executable
crates/plugin-api/    Shared C ABI definitions
plugins/uppercase/    Plugin that converts text to uppercase
plugins/size/         Plugin that counts text characters
```

The root `Cargo.toml` defines a Cargo workspace. Each listed member is a separate Rust crate.

## Plugin ABI

Each plugin exports one function with the exact name `plugin_entry`.

```rust
#[unsafe(no_mangle)]
pub extern "C" fn plugin_entry() -> PluginDescriptor
```

The host loads this symbol with `libloading`. The function returns a `PluginDescriptor`.

The descriptor gives the host these items:

```text
ABI version
Plugin name
Transform function pointer
Function to free plugin output
```

The host checks the ABI version before it calls the plugin.

The host passes input as a null-terminated C string. The plugin returns a `PluginString` that contains a pointer to another null-terminated C string.

The plugin allocates returned output. The host reads the output. The host then calls `free_string`. The plugin frees its own allocation.

## Run Plugins

Build the project before you run a plugin.

On macOS, run the host with the uppercase plugin:

```bash
./target/debug/host "./target/debug/libuppercase_plugin.dylib" "hello"
```

Expected output:

```text
Loaded plugin: uppercase
Output: HELLO
```

Run the host with the size plugin:

```bash
./target/debug/host "./target/debug/libsize_plugin.dylib" "hello"
```

Expected output:

```text
Loaded plugin: size
Output: 5
```