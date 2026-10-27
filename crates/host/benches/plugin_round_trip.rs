use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use libloading::{Library, Symbol};
use plugin_api::{InputBuffer, PluginDescriptor, PLUGIN_ABI_VERSION};
use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
use std::ffi::CStr;
use std::path::PathBuf;

type PluginEntry = unsafe extern "C" fn() -> PluginDescriptor;

struct LoadedPlugin {
    // The descriptor contains plugin function pointers, so its library must outlive every call.
    _library: Library,
    descriptor: PluginDescriptor,
}

impl LoadedPlugin {
    unsafe fn load(path: PathBuf) -> Self {
        let library = unsafe { Library::new(path) }.expect("load uppercase plugin");
        let entry: Symbol<PluginEntry> =
            unsafe { library.get(b"plugin_entry") }.expect("load plugin_entry symbol");
        let descriptor = unsafe { entry() };

        assert_eq!(descriptor.abi_version, PLUGIN_ABI_VERSION);

        Self {
            _library: library,
            descriptor,
        }
    }

    fn transform(&self, input: &str) -> String {
        let output = unsafe {
            (self.descriptor.transform)(InputBuffer {
                ptr: input.as_ptr(),
                len: input.len(),
            })
        };
        assert!(!output.ptr.is_null(), "plugin returned null output");

        let output_text = unsafe { CStr::from_ptr(output.ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { (self.descriptor.free_string)(output) };
        output_text
    }
}

fn uppercase_plugin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("target")
        .join("release")
        .join(format!("{DLL_PREFIX}uppercase_plugin{DLL_SUFFIX}"))
}

fn benchmark_round_trip(c: &mut Criterion) {
    let plugin = unsafe { LoadedPlugin::load(uppercase_plugin_path()) };
    assert_eq!(plugin.transform("hello"), "HELLO");

    let inputs = [
        ("small", "hello".repeat(1)),
        ("64KiB", "a".repeat(64 * 1024)),
    ];

    let mut direct_group = c.benchmark_group("direct_rust");
    for (name, input) in &inputs {
        direct_group.throughput(Throughput::Bytes(input.len() as u64));
        direct_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| black_box(black_box(input.as_str()).to_uppercase()))
        });
    }
    direct_group.finish();

    let mut plugin_group = c.benchmark_group("plugin_round_trip");
    for (name, input) in &inputs {
        plugin_group.throughput(Throughput::Bytes(input.len() as u64));
        plugin_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| black_box(plugin.transform(black_box(input))))
        });
    }
    plugin_group.finish();
}

criterion_group!(benches, benchmark_round_trip);
criterion_main!(benches);
