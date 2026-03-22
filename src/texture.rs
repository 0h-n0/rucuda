//! Texture Object Management functions.

use crate::types::*;
use std::os::raw::c_int;

unsafe extern "C" {
    /// Creates a channel descriptor.
    pub fn cudaCreateChannelDesc(
        x: c_int,
        y: c_int,
        z: c_int,
        w: c_int,
        f: cudaChannelFormatKind,
    ) -> cudaChannelFormatDesc;

    /// Creates a texture object.
    pub fn cudaCreateTextureObject(
        pTexObject: *mut cudaTextureObject_t,
        pResDesc: *const cudaResourceDesc,
        pTexDesc: *const cudaTextureDesc,
        pResViewDesc: *const cudaResourceViewDesc,
    ) -> cudaError_t;

    /// Destroys a texture object.
    pub fn cudaDestroyTextureObject(texObject: cudaTextureObject_t) -> cudaError_t;

    /// Get the channel descriptor of an array.
    pub fn cudaGetChannelDesc(
        desc: *mut cudaChannelFormatDesc,
        array: cudaArray_const_t,
    ) -> cudaError_t;

    /// Returns a texture object's resource descriptor.
    pub fn cudaGetTextureObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;

    /// Returns a texture object's resource view descriptor.
    pub fn cudaGetTextureObjectResourceViewDesc(
        pResViewDesc: *mut cudaResourceViewDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;

    /// Returns a texture object's texture descriptor.
    pub fn cudaGetTextureObjectTextureDesc(
        pTexDesc: *mut cudaTextureDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
}
