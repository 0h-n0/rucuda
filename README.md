# rucuda

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

> Low-level Rust FFI bindings for the NVIDIA CUDA Runtime API.

## Overview

**rucuda** provides raw, `unsafe extern "C"` bindings to `libcudart` — the CUDA Runtime library. It exposes ~300 functions across 19 modules with 100+ `#[repr(C)]` types, all re-exported at the crate root for convenience.

**How it differs from other crates:**

- Unlike **cuda-sys**, rucuda is hand-written (no bindgen dependency) for minimal build complexity.
- Unlike **cudarc**, rucuda provides raw FFI only — no safe wrappers, no opinionated abstractions. Use it when you need full control.

## Requirements

- **CUDA Toolkit 12.x** (includes APIs up to CUDA 12.8+)
- `libcudart` available for dynamic linking
- **Rust 1.85+** (edition 2024)

## Installation

```toml
[dependencies]
rucuda = "0.1"
```

### Feature flags

| Feature  | Description |
|----------|-------------|
| `opengl` | OpenGL interoperability (`cudaGLGetDevices`, `cudaGraphicsGLRegisterBuffer`, `cudaGraphicsGLRegisterImage`) |

```toml
[dependencies]
rucuda = { version = "0.1", features = ["opengl"] }
```

## Quick Start

```rust
use rucuda::*;

unsafe {
    let mut count: std::os::raw::c_int = 0;
    let err = cudaGetDeviceCount(&mut count);
    if err == cudaSuccess {
        println!("Found {} CUDA device(s)", count);
    }
}
```

## Examples

### Device memory allocation and transfer

```rust
use rucuda::*;
use std::os::raw::c_void;
use std::ptr;

unsafe {
    // Allocate device memory
    let n = 1024usize;
    let size = n * std::mem::size_of::<f32>();
    let mut d_ptr: *mut c_void = ptr::null_mut();
    let err = cudaMalloc(&mut d_ptr, size);
    assert_eq!(err, cudaSuccess);

    // Copy host data to device
    let host_data: Vec<f32> = vec![1.0; n];
    let err = cudaMemcpy(
        d_ptr,
        host_data.as_ptr() as *const c_void,
        size,
        cudaMemcpyHostToDevice,
    );
    assert_eq!(err, cudaSuccess);

    // Copy back to host
    let mut result: Vec<f32> = vec![0.0; n];
    let err = cudaMemcpy(
        result.as_mut_ptr() as *mut c_void,
        d_ptr,
        size,
        cudaMemcpyDeviceToHost,
    );
    assert_eq!(err, cudaSuccess);

    // Free device memory
    cudaFree(d_ptr);
}
```

## Safety

All functions in this crate are `unsafe`. The caller must ensure:

- **Valid pointers** — all pointer arguments must be valid and properly aligned for their type
- **Memory region correctness** — device pointers for device memory, host pointers for host memory
- **Lifetime management** — resources (streams, events, memory) must not be used after being freed
- **Error checking** — always check `cudaError_t` return values; ignoring errors leads to undefined behavior in subsequent calls
- **Thread safety** — follow the CUDA Runtime API's threading rules (e.g., one primary context per device per process)

Refer to the [CUDA Runtime API documentation](https://docs.nvidia.com/cuda/cuda-runtime-api/) for detailed safety requirements of each function.

## Modules

| Module | Description |
|--------|-------------|
| `device` | Device Management |
| `error` | Error Handling |
| `stream` | Stream Management |
| `event` | Event Management |
| `execution` | Execution Control |
| `memory` | Memory Management |
| `memory_pool` | Stream Ordered Memory Allocator |
| `unified` | Unified Addressing |
| `peer` | Peer Device Memory Access |
| `occupancy` | Occupancy |
| `texture` | Texture Object Management |
| `surface` | Surface Object Management |
| `graph` | Graph Management |
| `external` | External Resource Interoperability |
| `profiler` | Profiler Control |
| `driver_entry` | Driver Entry Point Access |
| `library` | Library Management |
| `version` | Version Management |
| `graphics` | Graphics Interoperability |
| `opengl`* | OpenGL Interoperability |

\* Requires the `opengl` feature.

## CUDA Detection

The build script (`build.rs`) locates the CUDA Toolkit in this order:

1. **Environment variables**: `CUDA_PATH`, `CUDA_HOME`, `CUDA_ROOT`
2. **nvcc in PATH**: derives the CUDA root from `which nvcc`
3. **Standard paths**: `/usr/local/cuda`, `/opt/cuda`, `/usr/lib/cuda`

To specify a custom CUDA installation:

```bash
CUDA_PATH=/usr/local/cuda-12.8 cargo build
```

## ABI Verification

`tests/verify_sizes.c` is a C program that validates struct sizes, alignment, and field offsets match between the Rust definitions and the C headers. To run:

```bash
nvcc -o verify_sizes tests/verify_sizes.c && ./verify_sizes
```

## Development

```bash
cargo build           # Build
cargo test            # Run tests
cargo fmt             # Format
cargo clippy          # Lint
cargo doc --no-deps   # Generate docs
```

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
