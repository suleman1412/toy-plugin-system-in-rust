# Toy String Plugins

This is a small Rust project. It uses a C ABI to do basic string operations.

Each string operation is a separate plugin. The host program loads a plugin at run time and sends it a string.

Current plugins:

```text
uppercase  Convert text to uppercase.
size       Count characters in text.
```

## Project Layout

```text
crates/host/          Host that loads and runs plugins
crates/plugin-api/    Shared C ABI definitions
plugins/*             Individual plugin definition
```

## How It Works

Each plugin exports a function named `plugin_entry`.

The host uses `libloading` to find this function in the plugin library. The function gives the host the plugin name and its string operation.

The host sends input as a pointer and a byte length. The plugin reads the host input without a copy.

The plugin returns its result as a C string. The plugin also gives the host a function to free that result.

## Run A Plugin

On macOS, run the uppercase plugin:

```bash
./target/debug/host "./target/debug/libuppercase_plugin.dylib" "hello"
```

Expected output:

```text
Loaded plugin: uppercase
Output: HELLO
```

Run the size plugin:

```bash
./target/debug/host "./target/debug/libsize_plugin.dylib" "hello"
```

Expected output:

```text
Loaded plugin: size
Output: 5
```
