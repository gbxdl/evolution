# Profiling

## Perf

using perf on linux. https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html

- sudo perf record --call-graph=dwarf ./target/release/evolution
- perf report --hierarchy -M intel (and wait for the errors to clear)

## Flamegraph

https://github.com/flamegraph-rs/flamegraph

- cargo install flamegraph
- echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid (on linux, probably a bad idea but ok)
- maybe also this: echo 0 |sudo tee /proc/sys/kernel/kptr_restrict (to get rid of unknown)
- open flamegraph.svg with browser