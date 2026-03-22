//! Memory Management functions.

use crate::types::*;
use std::os::raw::{c_int, c_uint, c_void};

unsafe extern "C" {
    /// Gets info about the specified cudaArray.
    pub fn cudaArrayGetInfo(
        desc: *mut cudaChannelFormatDesc,
        extent: *mut cudaExtent,
        flags: *mut c_uint,
        array: cudaArray_t,
    ) -> cudaError_t;

    /// Returns the memory requirements of a CUDA array.
    pub fn cudaArrayGetMemoryRequirements(
        memoryRequirements: *mut cudaArrayMemoryRequirements,
        array: cudaArray_t,
        device: c_int,
    ) -> cudaError_t;

    /// Gets a CUDA array plane from a CUDA array.
    pub fn cudaArrayGetPlane(
        pPlaneArray: *mut cudaArray_t,
        hArray: cudaArray_t,
        planeIdx: c_uint,
    ) -> cudaError_t;

    /// Returns the layout properties of a sparse CUDA array.
    pub fn cudaArrayGetSparseProperties(
        sparseProperties: *mut cudaArraySparseProperties,
        array: cudaArray_t,
    ) -> cudaError_t;

    /// Frees memory on the device.
    pub fn cudaFree(devPtr: *mut c_void) -> cudaError_t;

    /// Frees an array on the device.
    pub fn cudaFreeArray(array: cudaArray_t) -> cudaError_t;

    /// Frees page-locked memory.
    pub fn cudaFreeHost(ptr: *mut c_void) -> cudaError_t;

    /// Frees a mipmapped array on the device.
    pub fn cudaFreeMipmappedArray(mipmappedArray: cudaMipmappedArray_t) -> cudaError_t;

    /// Gets a mipmap level of a CUDA mipmapped array.
    pub fn cudaGetMipmappedArrayLevel(
        levelArray: *mut cudaArray_t,
        mipmappedArray: cudaMipmappedArray_const_t,
        level: c_uint,
    ) -> cudaError_t;

    /// Finds the address associated with a CUDA symbol.
    pub fn cudaGetSymbolAddress(devPtr: *mut *mut c_void, symbol: *const c_void) -> cudaError_t;

    /// Finds the size of the object associated with a CUDA symbol.
    pub fn cudaGetSymbolSize(size: *mut usize, symbol: *const c_void) -> cudaError_t;

    /// Allocates page-locked memory on the host.
    pub fn cudaHostAlloc(pHost: *mut *mut c_void, size: usize, flags: c_uint) -> cudaError_t;

    /// Passes back device pointer of mapped host memory.
    pub fn cudaHostGetDevicePointer(
        pDevice: *mut *mut c_void,
        pHost: *mut c_void,
        flags: c_uint,
    ) -> cudaError_t;

    /// Passes back flags used to allocate pinned host memory.
    pub fn cudaHostGetFlags(pFlags: *mut c_uint, pHost: *mut c_void) -> cudaError_t;

    /// Registers an existing host memory range for use by CUDA.
    pub fn cudaHostRegister(ptr: *mut c_void, size: usize, flags: c_uint) -> cudaError_t;

    /// Unregisters a memory range that was registered with cudaHostRegister.
    pub fn cudaHostUnregister(ptr: *mut c_void) -> cudaError_t;

    /// Allocate memory on the device.
    pub fn cudaMalloc(devPtr: *mut *mut c_void, size: usize) -> cudaError_t;

    /// Allocates logical 1D, 2D, or 3D memory objects on the device.
    pub fn cudaMalloc3D(pitchedDevPtr: *mut cudaPitchedPtr, extent: cudaExtent) -> cudaError_t;

    /// Allocate an array on the device.
    pub fn cudaMalloc3DArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        flags: c_uint,
    ) -> cudaError_t;

    /// Allocate an array on the device.
    pub fn cudaMallocArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        flags: c_uint,
    ) -> cudaError_t;

    /// Allocates page-locked memory on the host.
    pub fn cudaMallocHost(ptr: *mut *mut c_void, size: usize) -> cudaError_t;

    /// Allocates memory that will be automatically managed by the Unified Memory system.
    pub fn cudaMallocManaged(devPtr: *mut *mut c_void, size: usize, flags: c_uint) -> cudaError_t;

    /// Allocate a mipmapped array on the device.
    pub fn cudaMallocMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        numLevels: c_uint,
        flags: c_uint,
    ) -> cudaError_t;

    /// Allocates pitched memory on the device.
    pub fn cudaMallocPitch(
        devPtr: *mut *mut c_void,
        pitch: *mut usize,
        width: usize,
        height: usize,
    ) -> cudaError_t;

    /// Advise about the usage of a given memory range.
    pub fn cudaMemAdvise(
        devPtr: *const c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        location: cudaMemLocation,
    ) -> cudaError_t;

    /// Gets free and total device memory.
    pub fn cudaMemGetInfo(free: *mut usize, total: *mut usize) -> cudaError_t;

    /// Prefetches memory to the specified destination device.
    pub fn cudaMemPrefetchAsync(
        devPtr: *const c_void,
        count: usize,
        location: cudaMemLocation,
        flags: c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Prefetches memory regions to the specified device in batch.
    pub fn cudaMemPrefetchBatchAsync(
        dptrs: *mut *mut c_void,
        sizes: *mut usize,
        count: usize,
        prefetchLocs: *mut cudaMemLocation,
        prefetchLocIdxs: *mut usize,
        numPrefetchLocs: usize,
        flags: u64,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Query an attribute of a given memory range.
    pub fn cudaMemRangeGetAttribute(
        data: *mut c_void,
        dataSize: usize,
        attribute: cudaMemRangeAttribute,
        devPtr: *const c_void,
        count: usize,
    ) -> cudaError_t;

    /// Query attributes of a given memory range.
    pub fn cudaMemRangeGetAttributes(
        data: *mut *mut c_void,
        dataSizes: *mut usize,
        attributes: *mut cudaMemRangeAttribute,
        numAttributes: usize,
        devPtr: *const c_void,
        count: usize,
    ) -> cudaError_t;

    /// Copies data between host and device.
    pub fn cudaMemcpy(
        dst: *mut c_void,
        src: *const c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data between host and device (2D).
    pub fn cudaMemcpy2D(
        dst: *mut c_void,
        dpitch: usize,
        src: *const c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data between host and device (2D, array to array).
    pub fn cudaMemcpy2DArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data between host and device (2D, async).
    pub fn cudaMemcpy2DAsync(
        dst: *mut c_void,
        dpitch: usize,
        src: *const c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies data from an array to host (2D).
    pub fn cudaMemcpy2DFromArray(
        dst: *mut c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data from an array to host (2D, async).
    pub fn cudaMemcpy2DFromArrayAsync(
        dst: *mut c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies data to an array (2D).
    pub fn cudaMemcpy2DToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data to an array (2D, async).
    pub fn cudaMemcpy2DToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies data between 3D objects.
    pub fn cudaMemcpy3D(p: *const cudaMemcpy3DParms) -> cudaError_t;

    /// Copies data between 3D objects (async).
    pub fn cudaMemcpy3DAsync(p: *const cudaMemcpy3DParms, stream: cudaStream_t) -> cudaError_t;

    /// Copies data between 3D objects in batch (async).
    pub fn cudaMemcpy3DBatchAsync(
        numOps: usize,
        opList: *mut cudaMemcpy3DBatchOp,
        flags: u64,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies memory between devices.
    pub fn cudaMemcpy3DPeer(p: *const cudaMemcpy3DPeerParms) -> cudaError_t;

    /// Copies memory between devices (async).
    pub fn cudaMemcpy3DPeerAsync(
        p: *const cudaMemcpy3DPeerParms,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies data asynchronously between host and device.
    pub fn cudaMemcpyAsync(
        dst: *mut c_void,
        src: *const c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies data from the given symbol on the device.
    pub fn cudaMemcpyFromSymbol(
        dst: *mut c_void,
        symbol: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data from the given symbol on the device (async).
    pub fn cudaMemcpyFromSymbolAsync(
        dst: *mut c_void,
        symbol: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies memory between two devices.
    pub fn cudaMemcpyPeer(
        dst: *mut c_void,
        dstDevice: c_int,
        src: *const c_void,
        srcDevice: c_int,
        count: usize,
    ) -> cudaError_t;

    /// Copies memory between two devices (async).
    pub fn cudaMemcpyPeerAsync(
        dst: *mut c_void,
        dstDevice: c_int,
        src: *const c_void,
        srcDevice: c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Copies data to the given symbol on the device.
    pub fn cudaMemcpyToSymbol(
        symbol: *const c_void,
        src: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Copies data to the given symbol on the device (async).
    pub fn cudaMemcpyToSymbolAsync(
        symbol: *const c_void,
        src: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Initializes or sets device memory to a value.
    pub fn cudaMemset(devPtr: *mut c_void, value: c_int, count: usize) -> cudaError_t;

    /// Initializes or sets device memory to a value (2D).
    pub fn cudaMemset2D(
        devPtr: *mut c_void,
        pitch: usize,
        value: c_int,
        width: usize,
        height: usize,
    ) -> cudaError_t;

    /// Initializes or sets device memory to a value (2D, async).
    pub fn cudaMemset2DAsync(
        devPtr: *mut c_void,
        pitch: usize,
        value: c_int,
        width: usize,
        height: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Initializes or sets device memory to a value (3D).
    pub fn cudaMemset3D(
        pitchedDevPtr: cudaPitchedPtr,
        value: c_int,
        extent: cudaExtent,
    ) -> cudaError_t;

    /// Initializes or sets device memory to a value (3D, async).
    pub fn cudaMemset3DAsync(
        pitchedDevPtr: cudaPitchedPtr,
        value: c_int,
        extent: cudaExtent,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Initializes or sets device memory to a value (async).
    pub fn cudaMemsetAsync(
        devPtr: *mut c_void,
        value: c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Returns the memory requirements of a CUDA mipmapped array.
    pub fn cudaMipmappedArrayGetMemoryRequirements(
        memoryRequirements: *mut cudaArrayMemoryRequirements,
        mipmap: cudaMipmappedArray_t,
        device: c_int,
    ) -> cudaError_t;

    /// Returns the layout properties of a sparse CUDA mipmapped array.
    pub fn cudaMipmappedArrayGetSparseProperties(
        sparseProperties: *mut cudaArraySparseProperties,
        mipmap: cudaMipmappedArray_t,
    ) -> cudaError_t;
}
