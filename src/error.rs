//! Error Handling functions.

use crate::types::*;
use std::os::raw::c_char;

unsafe extern "C" {
    /// Returns the string representation of an error code enum name.
    pub fn cudaGetErrorName(error: cudaError_t) -> *const c_char;

    /// Returns the description string for an error code.
    pub fn cudaGetErrorString(error: cudaError_t) -> *const c_char;

    /// Returns the last error that has been produced by any of the runtime calls
    /// in the same host thread and resets it to `cudaSuccess`.
    pub fn cudaGetLastError() -> cudaError_t;

    /// Returns the last error that has been produced by any of the runtime calls
    /// in the same host thread. Does not reset the error.
    pub fn cudaPeekAtLastError() -> cudaError_t;
}
