//! External Resource Interoperability functions.

use crate::types::*;
use std::os::raw::c_uint;

unsafe extern "C" {
    /// Destroys an external memory object.
    pub fn cudaDestroyExternalMemory(extMem: cudaExternalMemory_t) -> cudaError_t;

    /// Destroys an external semaphore.
    pub fn cudaDestroyExternalSemaphore(extSem: cudaExternalSemaphore_t) -> cudaError_t;

    /// Maps a buffer onto an imported memory object.
    pub fn cudaExternalMemoryGetMappedBuffer(
        devPtr: *mut *mut std::os::raw::c_void,
        extMem: cudaExternalMemory_t,
        bufferDesc: *const cudaExternalMemoryBufferDesc,
    ) -> cudaError_t;

    /// Maps a CUDA mipmapped array onto an external memory object.
    pub fn cudaExternalMemoryGetMappedMipmappedArray(
        mipmap: *mut cudaMipmappedArray_t,
        extMem: cudaExternalMemory_t,
        mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
    ) -> cudaError_t;

    /// Imports an external memory object.
    pub fn cudaImportExternalMemory(
        extMem_out: *mut cudaExternalMemory_t,
        memHandleDesc: *const cudaExternalMemoryHandleDesc,
    ) -> cudaError_t;

    /// Imports an external semaphore.
    pub fn cudaImportExternalSemaphore(
        extSem_out: *mut cudaExternalSemaphore_t,
        semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
    ) -> cudaError_t;

    /// Signals a set of external semaphore objects.
    pub fn cudaSignalExternalSemaphoresAsync(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreSignalParams,
        numExtSems: c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Waits on a set of external semaphore objects.
    pub fn cudaWaitExternalSemaphoresAsync(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreWaitParams,
        numExtSems: c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
