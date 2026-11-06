# Toy String Plugins

This is a small Rust project. It uses a C ABI to do basic string operations.

Each string operation is a separate plugin. The host program loads a plugin at run time and sends it a string.

Current plugins:

```text
uppercase  Convert text to uppercase.
size       Count characters in text.
document   Count tags in a document input.
```

## Project Layout

```text
crates/host/          Host that loads and runs plugins
crates/plugin-api/    Shared C ABI definitions
  src/types.rs        ABI data types and buffers
  src/descriptors.rs  ABI function descriptors
plugins/*             Individual plugin definition
```

## How It Works

Each plugin exports a function named `plugin_entry`.

The host uses `libloading` to find this function in the plugin library. The function gives the host the plugin name and its string operation.

The host sends input as a pointer and a byte length. The plugin reads the host input without a copy.

The plugin returns its result as a C string. The plugin also gives the host a function to free that result.

The document plugin exports `document_plugin_entry`. Its `count_tags` function receives a `DocumentInput` value. `DocumentInput` contains buffers for the title and tags, plus a document kind.

All plugin descriptors use the shared `PLUGIN_ABI_VERSION`. A descriptor change requires an ABI version change.

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
