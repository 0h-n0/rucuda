//! Unified Addressing functions.

use crate::types::*;
use std::os::raw::c_void;

unsafe extern "C" {
    /// Returns attributes about a specified pointer.
    pub fn cudaPointerGetAttributes(
        attributes: *mut cudaPointerAttributes,
        ptr: *const c_void,
    ) -> cudaError_t;
}
