//! OpenGL Interoperability functions.
//!
//! Requires the `opengl` feature to be enabled.

#![cfg(feature = "opengl")]

use crate::types::*;
use std::os::raw::{c_int, c_uint};

/// OpenGL unsigned int type alias.
pub type GLuint = c_uint;
/// OpenGL enum type alias.
pub type GLenum = c_uint;

unsafe extern "C" {
    /// Gets the CUDA devices associated with the current OpenGL context.
    pub fn cudaGLGetDevices(
        pCudaDeviceCount: *mut c_uint,
        pCudaDevices: *mut c_int,
        cudaDeviceCount: c_uint,
        deviceList: cudaGLDeviceList,
    ) -> cudaError_t;

    /// Registers an OpenGL buffer object.
    pub fn cudaGraphicsGLRegisterBuffer(
        resource: *mut cudaGraphicsResource_t,
        buffer: GLuint,
        flags: c_uint,
    ) -> cudaError_t;

    /// Register an OpenGL texture or renderbuffer object.
    pub fn cudaGraphicsGLRegisterImage(
        resource: *mut cudaGraphicsResource_t,
        image: GLuint,
        target: GLenum,
        flags: c_uint,
    ) -> cudaError_t;
}
