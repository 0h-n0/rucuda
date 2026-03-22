//! # rucuda
//!
//! Low-level Rust FFI bindings for the NVIDIA CUDA Runtime API.
//!
//! This crate provides raw, unsafe bindings to the CUDA Runtime API (`libcudart`),
//! allowing Rust programs to call CUDA functions directly. All functions are
//! `unsafe` and use C-compatible types.
//!
//! ## Requirements
//!
//! - CUDA Toolkit installed (detects via `CUDA_PATH`, `CUDA_HOME`, or standard locations)
//! - `libcudart` available for dynamic linking
//!
//! ## Modules
//!
//! Each module corresponds to a section of the CUDA Runtime API:
//!
//! - [`device`] - Device Management
//! - [`error`] - Error Handling
//! - [`stream`] - Stream Management
//! - [`event`] - Event Management
//! - [`execution`] - Execution Control
//! - [`memory`] - Memory Management
//! - [`memory_pool`] - Stream Ordered Memory Allocator
//! - [`unified`] - Unified Addressing
//! - [`peer`] - Peer Device Memory Access
//! - [`occupancy`] - Occupancy
//! - [`texture`] - Texture Object Management
//! - [`surface`] - Surface Object Management
//! - [`graph`] - Graph Management
//! - [`external`] - External Resource Interoperability
//! - [`profiler`] - Profiler Control
//! - [`driver_entry`] - Driver Entry Point Access
//! - [`library`] - Library Management
//! - [`version`] - Version Management
//! - [`graphics`] - Graphics Interoperability
//! - `opengl` - OpenGL Interoperability (feature-gated)
//!
//! ## Example
//!
//! ```rust,no_run
//! use rucuda::*;
//!
//! unsafe {
//!     let mut count: std::os::raw::c_int = 0;
//!     let err = cudaGetDeviceCount(&mut count);
//!     if err == cudaSuccess {
//!         println!("Found {} CUDA device(s)", count);
//!     }
//! }
//! ```

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod types;
pub mod device;
pub mod error;
pub mod stream;
pub mod event;
pub mod execution;
pub mod memory;
pub mod memory_pool;
pub mod unified;
pub mod peer;
pub mod occupancy;
pub mod texture;
pub mod surface;
pub mod graph;
pub mod external;
pub mod profiler;
pub mod driver_entry;
pub mod library;
pub mod version;
pub mod graphics;

#[cfg(feature = "opengl")]
pub mod opengl;

// Re-export all types and constants at the crate root for convenience.
pub use types::*;

// Re-export all FFI functions at the crate root.
pub use device::*;
pub use error::*;
pub use stream::*;
pub use event::*;
pub use execution::*;
pub use memory::*;
pub use memory_pool::*;
pub use unified::*;
pub use peer::*;
pub use occupancy::*;
pub use texture::*;
pub use surface::*;
pub use graph::*;
pub use external::*;
pub use profiler::*;
pub use driver_entry::*;
pub use library::*;
pub use version::*;
pub use graphics::*;

#[cfg(feature = "opengl")]
pub use opengl::*;
