# `keetree` (WIP)

[![Version](https://img.shields.io/crates/v/keetree?style=for-the-badge)](https://crates.io/crates/keetree)
[![License](https://img.shields.io/crates/l/keetree?style=for-the-badge)](https://crates.io/crates/keetree)

A lightweight and fast router with no_std support. The main library comes in at **182** lines only.

```rust
use keetree::Node;

fn main() {
    let router = Node::default();
    node.insert("a/b/c".split('/'), 1)
    assert_eq!(node.match("a/b/c".split('/')).unwrap(), 1)
}
```
## Notable Behavior
Inserts on the same route with different values will update it. 

## Benchmarks
Only twice as slow as ```matchit``` which is the fastest rust router (that I know of). Not bad for something that's completely unoptimized. You can find ```matchit```'s benchmarks [here](https://github.com/ibraheemdev/matchit?tab=readme-ov-file#benchmarks).

|Library |Match|
| ----------- | ----------- |
|matchit|190.58 ns|
|keetree|490.32 ns|
