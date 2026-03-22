//! Graphics Interoperability functions.

use crate::types::*;
use std::os::raw::{c_int, c_uint, c_void};

unsafe extern "C" {
    /// Map graphics resources for access by CUDA.
    pub fn cudaGraphicsMapResources(
        count: c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Get a mipmapped array through which to access a mapped graphics resource.
    pub fn cudaGraphicsResourceGetMappedMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;

    /// Get an device pointer through which to access a mapped graphics resource.
    pub fn cudaGraphicsResourceGetMappedPointer(
        devPtr: *mut *mut c_void,
        size: *mut usize,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;

    /// Set usage flags for mapping a graphics resource.
    pub fn cudaGraphicsResourceSetMapFlags(
        resource: cudaGraphicsResource_t,
        flags: c_uint,
    ) -> cudaError_t;

    /// Get an array through which to access a subresource of a mapped graphics resource.
    pub fn cudaGraphicsSubResourceGetMappedArray(
        array: *mut cudaArray_t,
        resource: cudaGraphicsResource_t,
        arrayIndex: c_uint,
        mipLevel: c_uint,
    ) -> cudaError_t;

    /// Unmap graphics resources.
    pub fn cudaGraphicsUnmapResources(
        count: c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Unregisters a graphics resource for access by CUDA.
    pub fn cudaGraphicsUnregisterResource(resource: cudaGraphicsResource_t) -> cudaError_t;
}
