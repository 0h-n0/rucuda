//! Stream Ordered Memory Allocator functions.

use crate::types::*;
use std::os::raw::{c_uint, c_void};

unsafe extern "C" {
    /// Frees memory with stream ordered semantics.
    pub fn cudaFreeAsync(devPtr: *mut c_void, hStream: cudaStream_t) -> cudaError_t;

    /// Allocates memory with stream ordered semantics.
    pub fn cudaMallocAsync(
        devPtr: *mut *mut c_void,
        size: usize,
        hStream: cudaStream_t,
    ) -> cudaError_t;

    /// Allocates memory from a specified pool with stream ordered semantics.
    pub fn cudaMallocFromPoolAsync(
        ptr: *mut *mut c_void,
        size: usize,
        memPool: cudaMemPool_t,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Creates a memory pool.
    pub fn cudaMemPoolCreate(
        memPool: *mut cudaMemPool_t,
        poolProps: *const cudaMemPoolProps,
    ) -> cudaError_t;

    /// Destroys the specified memory pool.
    pub fn cudaMemPoolDestroy(memPool: cudaMemPool_t) -> cudaError_t;

    /// Export data to share a memory pool between processes.
    pub fn cudaMemPoolExportPointer(
        exportData: *mut cudaMemPoolPtrExportData,
        ptr: *mut c_void,
    ) -> cudaError_t;

    /// Exports a memory pool to the requested handle type.
    pub fn cudaMemPoolExportToShareableHandle(
        shareableHandle: *mut c_void,
        memPool: cudaMemPool_t,
        handleType: cudaMemAllocationHandleType,
        flags: c_uint,
    ) -> cudaError_t;

    /// Returns the accessibility of a pool from a device.
    pub fn cudaMemPoolGetAccess(
        flags: *mut cudaMemAccessFlags,
        memPool: cudaMemPool_t,
        location: *mut cudaMemLocation,
    ) -> cudaError_t;

    /// Gets attributes of a memory pool.
    pub fn cudaMemPoolGetAttribute(
        memPool: cudaMemPool_t,
        attr: cudaMemPoolAttr,
        value: *mut c_void,
    ) -> cudaError_t;

    /// Imports a memory pool from a shared handle.
    pub fn cudaMemPoolImportFromShareableHandle(
        memPool: *mut cudaMemPool_t,
        shareableHandle: *mut c_void,
        handleType: cudaMemAllocationHandleType,
        flags: c_uint,
    ) -> cudaError_t;

    /// Import a memory pool allocation from another process.
    pub fn cudaMemPoolImportPointer(
        ptr: *mut *mut c_void,
        memPool: cudaMemPool_t,
        exportData: *mut cudaMemPoolPtrExportData,
    ) -> cudaError_t;

    /// Controls visibility of pools between devices.
    pub fn cudaMemPoolSetAccess(
        memPool: cudaMemPool_t,
        descList: *const cudaMemAccessDesc,
        count: usize,
    ) -> cudaError_t;

    /// Sets attributes of a memory pool.
    pub fn cudaMemPoolSetAttribute(
        memPool: cudaMemPool_t,
        attr: cudaMemPoolAttr,
        value: *mut c_void,
    ) -> cudaError_t;

    /// Tries to release memory back to the OS.
    pub fn cudaMemPoolTrimTo(memPool: cudaMemPool_t, minBytesToKeep: usize) -> cudaError_t;
}
