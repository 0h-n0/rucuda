//! Version Management functions.

use crate::types::*;
use std::os::raw::c_int;

unsafe extern "C" {
    /// Returns the CUDA runtime version.
    pub fn cudaRuntimeGetVersion(runtimeVersion: *mut c_int) -> cudaError_t;

    /// Returns the CUDA driver version.
    pub fn cudaDriverGetVersion(driverVersion: *mut c_int) -> cudaError_t;
}
