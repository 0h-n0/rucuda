//! Event Management functions.

use crate::types::*;
use std::os::raw::c_uint;

unsafe extern "C" {
    /// Creates an event object.
    pub fn cudaEventCreate(event: *mut cudaEvent_t) -> cudaError_t;

    /// Creates an event object with the specified flags.
    pub fn cudaEventCreateWithFlags(event: *mut cudaEvent_t, flags: c_uint) -> cudaError_t;

    /// Destroys an event object.
    pub fn cudaEventDestroy(event: cudaEvent_t) -> cudaError_t;

    /// Computes the elapsed time between events.
    pub fn cudaEventElapsedTime(
        ms: *mut f32,
        start: cudaEvent_t,
        end: cudaEvent_t,
    ) -> cudaError_t;

    /// Queries an event's status.
    pub fn cudaEventQuery(event: cudaEvent_t) -> cudaError_t;

    /// Records an event.
    pub fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t;

    /// Records an event with flags.
    pub fn cudaEventRecordWithFlags(
        event: cudaEvent_t,
        stream: cudaStream_t,
        flags: c_uint,
    ) -> cudaError_t;

    /// Waits for an event to complete.
    pub fn cudaEventSynchronize(event: cudaEvent_t) -> cudaError_t;
}
