# String plugin benchmarks

The [benchmark](crates/host/benches/plugin_round_trip.rs) compares:

- **Direct Rust:** uppercase the text normally.
- **Owned output:** plugin uppercases → host copies result → plugin frees original.
- **Borrowed output:** plugin uppercases → host uses result temporarily → plugin frees it.

Borrowing avoids the host's extra allocation and copy, but the host cannot keep that text afterward.

## Results

- **Small input (`hello`):** direct **16.54 ns**, owned **77.00 ns**, borrowed **43.06 ns**.
  Borrowing took about **44% less time** than owned output.
- **64 KiB:** direct **1.83 µs**, owned **46.46 µs**, borrowed **46.87 µs**.
  Borrowing showed no improvement in this run.

The output copy explains some small-call overhead, but not the large-input slowdown.
Library loading is excluded from timing. Allocation counts were not measured.

Original baseline: small input **16.46 ns direct / 76.68 ns plugin**;
64 KiB **1.85 µs direct / 46.38 µs plugin**.

## Run

```sh
cargo build --release --workspace
cargo bench -p host --bench plugin_round_trip
```

Environment: Rust 1.95.0, Apple Silicon macOS, release builds.
Criterion: 100 samples, 3-second warmup, approximately 5 seconds per case.
Results are from individual runs, not universal performance guarantees.
