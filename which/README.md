# which

A Rust implementation of `which` built for Windows first but cross-platform in theory.

![usage](./assets/usage.png)

## Installation

### CLI

```bash
cargo install --git https://github.com/DarkCeptor44/coreutils-windows which
```

Or grab a binary from [releases](https://github.com/DarkCeptor44/coreutils-windows/releases).

### Library

```bash
cargo add --git https://github.com/DarkCeptor44/coreutils-windows --no-default-features
```

Or:

```toml
[dependencies]
which = { version = "0.1.0", git = "https://github.com/DarkCeptor44/coreutils-windows", default-features = false }
```

## MSRV

The minimum supported Rust version is:

| Version | Edition | MSRV |
| --- | --- | --- |
| `<= 0.1.0` | 2024 | 1.85.0 |

## Usage Example

```rust
let found = which::which("python").unwrap();
// returns ["path/to/python.exe", "path/to/python3.exe"]
```

## Benchmarks

- **Machine:** Ryzen 7 3800X, RTX 2070 Super, 32GB DDR4 RAM

```text
Timer precision: 100 ns
which                fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ which                           │               │               │               │         │
│  ├─ t=1            1.664 ms      │ 3.346 ms      │ 1.946 ms      │ 2.009 ms      │ 100     │ 100
│  ├─ t=4            6.852 ms      │ 10.48 ms      │ 7.561 ms      │ 7.738 ms      │ 100     │ 100
│  ├─ t=8            13.69 ms      │ 15.24 ms      │ 14.41 ms      │ 14.47 ms      │ 104     │ 104
│  ╰─ t=16           26.44 ms      │ 30.21 ms      │ 28.01 ms      │ 28.21 ms      │ 112     │ 112
╰─ which_from_paths                │               │               │               │         │
   ├─ t=1            177.2 µs      │ 431.7 µs      │ 229.4 µs      │ 236.8 µs      │ 100     │ 100
   │                 11.28 Kitem/s │ 4.631 Kitem/s │ 8.714 Kitem/s │ 8.443 Kitem/s │         │
   ├─ t=4            610.9 µs      │ 1.607 ms      │ 738.1 µs      │ 791.2 µs      │ 100     │ 100
   │                 3.273 Kitem/s │ 1.244 Kitem/s │ 2.709 Kitem/s │ 2.527 Kitem/s │         │
   ├─ t=8            617.2 µs      │ 1.615 ms      │ 1.227 ms      │ 1.248 ms      │ 104     │ 104
   │                 3.239 Kitem/s │ 1.237 Kitem/s │ 1.629 Kitem/s │ 1.601 Kitem/s │         │
   ╰─ t=16           1.034 ms      │ 2.886 ms      │ 2.643 ms      │ 2.578 ms      │ 112     │ 112
                     1.932 Kitem/s │ 692.8 item/s  │ 756.5 item/s  │ 775.5 item/s  │         │
```

## Audits

Refer to [audits](../README.md#audits)

## License

This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/en-US/MPL/2.0/). See the [LICENSE](../LICENSE) file for details.
