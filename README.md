# rust-benchmarks
A collection of micro benchmarks to build intuition about performance of Rust code.

### Examples
- What is the performance of loops vs iterators?
- How much Arena allocator is faster than Box allocation?
- What is effect of inlining in a loop vs function call?
- ...

### Setup

```
git clone git@github.com:aslesarenko/rust-benchmarks.git
cd rust-benchmarks
cargo bench
```
