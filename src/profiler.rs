//! Profiler Control functions.

use crate::types::*;

unsafe extern "C" {
    /// Enable profiling.
    pub fn cudaProfilerStart() -> cudaError_t;

    /// Disable profiling.
    pub fn cudaProfilerStop() -> cudaError_t;
}
