//! Execution Control functions.

use crate::types::*;
use std::os::raw::{c_char, c_int, c_uint, c_void};

unsafe extern "C" {
    /// Find out attributes for a given function.
    pub fn cudaFuncGetAttributes(attr: *mut cudaFuncAttributes, func: *const c_void) -> cudaError_t;

    /// Returns the function name for a device function pointer.
    pub fn cudaFuncGetName(name: *mut *const c_char, func: *const c_void) -> cudaError_t;

    /// Returns the number of parameters for a given function.
    pub fn cudaFuncGetParamCount(func: *const c_void, paramCount: *mut usize) -> cudaError_t;

    /// Returns the offset and size of a kernel parameter in the device-side parameter layout.
    pub fn cudaFuncGetParamInfo(
        func: *const c_void,
        paramIndex: usize,
        paramOffset: *mut usize,
        paramSize: *mut usize,
    ) -> cudaError_t;

    /// Set attributes for a given function.
    pub fn cudaFuncSetAttribute(
        func: *const c_void,
        attr: cudaFuncAttribute,
        value: c_int,
    ) -> cudaError_t;

    /// Sets the preferred cache configuration for a device function.
    pub fn cudaFuncSetCacheConfig(
        func: *const c_void,
        cacheConfig: cudaFuncCache,
    ) -> cudaError_t;

    /// Launches a device function.
    pub fn cudaLaunchCooperativeKernel(
        func: *const c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Enqueues a host function call in a stream.
    pub fn cudaLaunchHostFunc(
        stream: cudaStream_t,
        fn_: cudaHostFn_t,
        userData: *mut c_void,
    ) -> cudaError_t;

    /// Enqueues a host function call in a stream with sync mode.
    pub fn cudaLaunchHostFunc_v2(
        stream: cudaStream_t,
        fn_: cudaHostFn_t,
        userData: *mut c_void,
        syncMode: c_uint,
    ) -> cudaError_t;

    /// Launches a device function.
    pub fn cudaLaunchKernel(
        func: *const c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;

    /// Launches a device function with launch configuration.
    pub fn cudaLaunchKernelExC(
        config: *const cudaLaunchConfig_t,
        func: *const c_void,
        args: *mut *mut c_void,
    ) -> cudaError_t;
}
