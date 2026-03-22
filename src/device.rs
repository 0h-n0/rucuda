//! Device Management functions.

use crate::types::*;
use std::os::raw::{c_char, c_int, c_uint, c_void};

unsafe extern "C" {
    /// Select device which best matches criteria.
    pub fn cudaChooseDevice(device: *mut c_int, prop: *const cudaDeviceProp) -> cudaError_t;

    /// Flush GPU direct RDMA writes.
    pub fn cudaDeviceFlushGPUDirectRDMAWrites(
        target: cudaFlushGPUDirectRDMAWritesTarget,
        scope: cudaFlushGPUDirectRDMAWritesScope,
    ) -> cudaError_t;

    /// Returns information about the device.
    pub fn cudaDeviceGetAttribute(
        value: *mut c_int,
        attr: cudaDeviceAttr,
        device: c_int,
    ) -> cudaError_t;

    /// Returns a handle to a compute device by PCI bus ID.
    pub fn cudaDeviceGetByPCIBusId(device: *mut c_int, pciBusId: *const c_char) -> cudaError_t;

    /// Returns the preferred cache configuration for the current device.
    pub fn cudaDeviceGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t;

    /// Returns the default memory pool of the device.
    pub fn cudaDeviceGetDefaultMemPool(
        memPool: *mut cudaMemPool_t,
        device: c_int,
    ) -> cudaError_t;

    /// Returns host atomic capabilities of a device for the specified atomic operations.
    pub fn cudaDeviceGetHostAtomicCapabilities(
        capabilities: *mut c_uint,
        operations: *const cudaAtomicOperation,
        count: c_uint,
        device: c_int,
    ) -> cudaError_t;

    /// Return resource limits.
    pub fn cudaDeviceGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t;

    /// Gets the current memory pool for the device.
    pub fn cudaDeviceGetMemPool(memPool: *mut cudaMemPool_t, device: c_int) -> cudaError_t;

    /// Return NvSciSync attributes for a device.
    pub fn cudaDeviceGetNvSciSyncAttributes(
        nvSciSyncAttrList: *mut c_void,
        device: c_int,
        flags: c_int,
    ) -> cudaError_t;

    /// Returns P2P atomic capabilities for the specified atomic operations.
    pub fn cudaDeviceGetP2PAtomicCapabilities(
        capabilities: *mut c_uint,
        operations: *const cudaAtomicOperation,
        count: c_uint,
        srcDevice: c_int,
        dstDevice: c_int,
    ) -> cudaError_t;

    /// Queries attributes of the link between two devices.
    pub fn cudaDeviceGetP2PAttribute(
        value: *mut c_int,
        attr: cudaDeviceP2PAttr,
        srcDevice: c_int,
        dstDevice: c_int,
    ) -> cudaError_t;

    /// Returns a PCI Bus Id string for the device.
    pub fn cudaDeviceGetPCIBusId(
        pciBusId: *mut c_char,
        len: c_int,
        device: c_int,
    ) -> cudaError_t;

    /// Returns numerical values that correspond to the least and greatest stream priorities.
    pub fn cudaDeviceGetStreamPriorityRange(
        leastPriority: *mut c_int,
        greatestPriority: *mut c_int,
    ) -> cudaError_t;

    /// Returns the maximum 1D texture width for the given format and device.
    pub fn cudaDeviceGetTexture1DLinearMaxWidth(
        maxWidthInElements: *mut usize,
        fmtDesc: *const cudaChannelFormatDesc,
        device: c_int,
    ) -> cudaError_t;

    /// Registers an async notification callback.
    pub fn cudaDeviceRegisterAsyncNotification(
        device: c_int,
        callbackFunc: cudaAsyncCallback,
        userData: *mut c_void,
        callback: *mut cudaAsyncCallbackHandle_t,
    ) -> cudaError_t;

    /// Destroy all allocations and reset all state on the current device.
    pub fn cudaDeviceReset() -> cudaError_t;

    /// Sets the preferred cache configuration for the current device.
    pub fn cudaDeviceSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t;

    /// Set resource limits.
    pub fn cudaDeviceSetLimit(limit: cudaLimit, value: usize) -> cudaError_t;

    /// Sets the current memory pool for a device.
    pub fn cudaDeviceSetMemPool(device: c_int, memPool: cudaMemPool_t) -> cudaError_t;

    /// Wait for compute device to finish.
    pub fn cudaDeviceSynchronize() -> cudaError_t;

    /// Unregisters an async notification callback.
    pub fn cudaDeviceUnregisterAsyncNotification(
        device: c_int,
        callback: cudaAsyncCallbackHandle_t,
    ) -> cudaError_t;

    /// Returns which device is currently being used.
    pub fn cudaGetDevice(device: *mut c_int) -> cudaError_t;

    /// Returns the number of compute-capable devices.
    pub fn cudaGetDeviceCount(count: *mut c_int) -> cudaError_t;

    /// Gets the flags for the current device.
    pub fn cudaGetDeviceFlags(flags: *mut c_uint) -> cudaError_t;

    /// Returns information about the compute-device.
    ///
    /// In CUDA 12+, this is the v2 symbol. The struct layout varies by CUDA version.
    /// Consider using [`cudaDeviceGetAttribute`] for portable property queries.
    pub fn cudaGetDeviceProperties_v2(prop: *mut cudaDeviceProp, device: c_int) -> cudaError_t;

    /// Returns information about the compute-device (CUDA 11 symbol).
    ///
    /// This links to the original `cudaGetDeviceProperties` symbol.
    /// On CUDA 12+, prefer `cudaGetDeviceProperties_v2`.
    pub fn cudaGetDeviceProperties(prop: *mut cudaDeviceProp, device: c_int) -> cudaError_t;

    /// Initialize device to be used for GPU executions.
    pub fn cudaInitDevice(
        device: c_int,
        deviceFlags: c_uint,
        flags: c_uint,
    ) -> cudaError_t;

    /// Close a memory handle that was opened with cudaIpcOpenMemHandle.
    pub fn cudaIpcCloseMemHandle(devPtr: *mut c_void) -> cudaError_t;

    /// Gets an interprocess handle for a previously allocated event.
    pub fn cudaIpcGetEventHandle(
        handle: *mut cudaIpcEventHandle_t,
        event: cudaEvent_t,
    ) -> cudaError_t;

    /// Gets an interprocess memory handle for an existing device memory allocation.
    pub fn cudaIpcGetMemHandle(
        handle: *mut cudaIpcMemHandle_t,
        devPtr: *mut c_void,
    ) -> cudaError_t;

    /// Opens an interprocess event handle for use in the current process.
    pub fn cudaIpcOpenEventHandle(
        event: *mut cudaEvent_t,
        handle: cudaIpcEventHandle_t,
    ) -> cudaError_t;

    /// Opens an interprocess memory handle exported from another process.
    pub fn cudaIpcOpenMemHandle(
        devPtr: *mut *mut c_void,
        handle: cudaIpcMemHandle_t,
        flags: c_uint,
    ) -> cudaError_t;

    /// Set device to be used for GPU executions.
    pub fn cudaSetDevice(device: c_int) -> cudaError_t;

    /// Sets flags to be used for device executions.
    pub fn cudaSetDeviceFlags(flags: c_uint) -> cudaError_t;

    /// Set a list of devices that can be used for CUDA.
    pub fn cudaSetValidDevices(device_arr: *mut c_int, len: c_int) -> cudaError_t;
}
