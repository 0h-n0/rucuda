//! Occupancy functions.

use crate::types::*;
use std::os::raw::{c_int, c_uint, c_void};

unsafe extern "C" {
    /// Returns dynamic shared memory available per block when launching
    /// `numBlocks` blocks on SM.
    pub fn cudaOccupancyAvailableDynamicSMemPerBlock(
        dynamicSmemSize: *mut usize,
        func: *const c_void,
        numBlocks: c_int,
        blockSize: c_int,
    ) -> cudaError_t;

    /// Returns occupancy for a device function.
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut c_int,
        func: *const c_void,
        blockSize: c_int,
        dynamicSMemSize: usize,
    ) -> cudaError_t;

    /// Returns occupancy for a device function with the specified flags.
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut c_int,
        func: *const c_void,
        blockSize: c_int,
        dynamicSMemSize: usize,
        flags: c_uint,
    ) -> cudaError_t;

    /// Returns the maximum number of active clusters for a given kernel.
    pub fn cudaOccupancyMaxActiveClusters(
        numClusters: *mut c_int,
        func: *const c_void,
        launchConfig: *const cudaLaunchConfig_t,
    ) -> cudaError_t;

    /// Returns the maximum potential cluster size for a given kernel.
    pub fn cudaOccupancyMaxPotentialClusterSize(
        clusterSize: *mut c_int,
        func: *const c_void,
        launchConfig: *const cudaLaunchConfig_t,
    ) -> cudaError_t;
}
