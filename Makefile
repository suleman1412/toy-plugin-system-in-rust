.PHONY: build, size, uppercase

SIZE-PLUGIN := ./target/debug/libsize_plugin.dylib
UPPERCASE-PLUGIN := ./target/debug/libuppercase_plugin.dylib
TEXT := hello world

build:
	cargo build

size: 
	cargo run -p host -- "$(SIZE-PLUGIN)" "$(TEXT)"

uppercase:
	cargo run -p host -- "$(UPPERCASE-PLUGIN)" "$(TEXT)"
