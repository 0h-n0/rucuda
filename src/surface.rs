//! Surface Object Management functions.

use crate::types::*;

unsafe extern "C" {
    /// Creates a surface object.
    pub fn cudaCreateSurfaceObject(
        pSurfObject: *mut cudaSurfaceObject_t,
        pResDesc: *const cudaResourceDesc,
    ) -> cudaError_t;

    /// Destroys a surface object.
    pub fn cudaDestroySurfaceObject(surfObject: cudaSurfaceObject_t) -> cudaError_t;

    /// Returns a surface object's resource descriptor.
    pub fn cudaGetSurfaceObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        surfObject: cudaSurfaceObject_t,
    ) -> cudaError_t;
}
