//! Driver Entry Point Access functions.

use crate::types::*;
use std::os::raw::{c_char, c_uint, c_void};

unsafe extern "C" {
    /// Returns the requested driver API function pointer.
    pub fn cudaGetDriverEntryPoint(
        symbol: *const c_char,
        funcPtr: *mut *mut c_void,
        flags: u64,
        driverStatus: *mut cudaDriverEntryPointQueryResult,
    ) -> cudaError_t;

    /// Returns the requested driver API function pointer by CUDA version.
    pub fn cudaGetDriverEntryPointByVersion(
        symbol: *const c_char,
        funcPtr: *mut *mut c_void,
        cudaVersion: c_uint,
        flags: u64,
        driverStatus: *mut cudaDriverEntryPointQueryResult,
    ) -> cudaError_t;
}
