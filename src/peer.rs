//! Peer Device Memory Access functions.

use crate::types::*;
use std::os::raw::{c_int, c_uint};

unsafe extern "C" {
    /// Queries if a device may directly access a peer device's memory.
    pub fn cudaDeviceCanAccessPeer(
        canAccessPeer: *mut c_int,
        device: c_int,
        peerDevice: c_int,
    ) -> cudaError_t;

    /// Disables direct access to memory allocations on a peer device.
    pub fn cudaDeviceDisablePeerAccess(peerDevice: c_int) -> cudaError_t;

    /// Enables direct access to memory allocations on a peer device.
    pub fn cudaDeviceEnablePeerAccess(peerDevice: c_int, flags: c_uint) -> cudaError_t;
}
