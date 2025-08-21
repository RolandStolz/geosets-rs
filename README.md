# geosets-rs

[![CI](https://github.com/STZRoland/geosets-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/STZRoland/geosets-rs/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/RolandStolz/geosets-rs/badge.svg?branch=main)](https://coveralls.io/github/RolandStolz/geosets-rs?branch=main)
[![Crates.io](https://img.shields.io/crates/v/geosets-rs)](https://crates.io/crates/geosets-rs)
[![docs.rs](https://docs.rs/geosets-rs/badge.svg)](https://docs.rs/geosets-rs)
[![License](https://img.shields.io/crates/l/geosets-rs)](https://github.com/your-username/geosets-rs/blob/main/LICENSE)

A Rust library for continuous set arithmetic.

## Features

The following set representations are supported
  - H-Polytopes (Half-space representation)
  - V-Polytopes (Vertex representation)
  - Zonotopes
  - Intervals

## Installation

Add this to your `Cargo.toml`:

```bash
cargo add geosets-rs
```

### System Dependencies

This library depends on several system libraries that need to be installed:

#### Ubuntu/Debian
```bash
sudo apt-get install libcdd-dev libopenblas-dev liblapack-dev pkg-config clang libclang-dev coinor-cbc coinor-libcbc-dev
```

#### macOS
```bash
brew install cddlib openblas lapack
```

## Usage

```rust
let set = HPolytope::from_random(2, 4).unwrap();

// Convert to vertices
let _vertices = set.to_vertices().unwrap();

// Calculate volume
let volume = set.volume().unwrap();
println!("Volume: {}", volume);

// Translate the set
let translation = ndarray::Array1::ones(2);
let translated = set.translate(&translation).unwrap();

// Plot
let mut plot = translated.plot((0, 1), true, true).unwrap();

// Plot with original
let original_trace = set
    .create_trace((0, 1), Some("Original VPolytope"))
    .unwrap();
plot.add_trace(original_trace);
plot.show();
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
