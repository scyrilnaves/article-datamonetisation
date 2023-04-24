PROFILING:

https://nnethercote.github.io/perf-book/profiling.html
https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html

https://rustc-dev-guide.rust-lang.org/profiling/with_perf.html

1) vi home/renault/.cargo/config.toml

[rust]
debuginfo-level = 1

2) Cargo.toml file:

[profile.release]
debug = 1

3) cargo build --release

4) perf record --call-graph=dwarf ./target/release/node-template --tmp --dev --port 30333 --ws-port 9944 --ws-external

5) perf report --hierarchy -M intel