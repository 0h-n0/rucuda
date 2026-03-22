//! Library Management functions.

use crate::types::*;
use std::os::raw::{c_char, c_int, c_uint, c_void};

unsafe extern "C" {
    /// Set an attribute for a kernel in a library for a specific device.
    pub fn cudaKernelSetAttributeForDevice(
        kernel: cudaKernel_t,
        attr: cudaFuncAttribute,
        value: c_int,
        device: c_int,
    ) -> cudaError_t;

    /// Enumerate kernels in a library.
    pub fn cudaLibraryEnumerateKernels(
        kernels: *mut cudaKernel_t,
        numKernels: c_uint,
        lib: cudaLibrary_t,
    ) -> cudaError_t;

    /// Returns a global device pointer from a library.
    pub fn cudaLibraryGetGlobal(
        dptr: *mut *mut c_void,
        bytes: *mut usize,
        library: cudaLibrary_t,
        name: *const c_char,
    ) -> cudaError_t;

    /// Returns a kernel handle from a library.
    pub fn cudaLibraryGetKernel(
        pKernel: *mut cudaKernel_t,
        library: cudaLibrary_t,
        name: *const c_char,
    ) -> cudaError_t;

    /// Returns the number of kernels in a library.
    pub fn cudaLibraryGetKernelCount(count: *mut c_uint, lib: cudaLibrary_t) -> cudaError_t;

    /// Returns a managed variable from a library.
    pub fn cudaLibraryGetManaged(
        dptr: *mut *mut c_void,
        bytes: *mut usize,
        library: cudaLibrary_t,
        name: *const c_char,
    ) -> cudaError_t;

    /// Returns a unified function pointer from a library.
    pub fn cudaLibraryGetUnifiedFunction(
        fptr: *mut *mut c_void,
        library: cudaLibrary_t,
        symbol: *const c_char,
    ) -> cudaError_t;

    /// Loads a library from data in memory.
    pub fn cudaLibraryLoadData(
        library: *mut cudaLibrary_t,
        code: *const c_void,
        jitOptions: *mut cudaJitOption,
        jitOptionsValues: *mut *mut c_void,
        numJitOptions: c_uint,
        libraryOptions: *mut cudaLibraryOption,
        libraryOptionValues: *mut *mut c_void,
        numLibraryOptions: c_uint,
    ) -> cudaError_t;

    /// Loads a library from a file.
    pub fn cudaLibraryLoadFromFile(
        library: *mut cudaLibrary_t,
        fileName: *const c_char,
        jitOptions: *mut cudaJitOption,
        jitOptionsValues: *mut *mut c_void,
        numJitOptions: c_uint,
        libraryOptions: *mut cudaLibraryOption,
        libraryOptionValues: *mut *mut c_void,
        numLibraryOptions: c_uint,
    ) -> cudaError_t;

    /// Unloads a library.
    pub fn cudaLibraryUnload(library: cudaLibrary_t) -> cudaError_t;
}
