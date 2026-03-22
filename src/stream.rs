//! Stream Management functions.

use crate::types::*;
use std::os::raw::{c_int, c_uint, c_void};

unsafe extern "C" {
    /// Resets all persisting lines in cache to normal status.
    pub fn cudaCtxResetPersistingL2Cache() -> cudaError_t;

    /// Add a callback to a compute stream.
    pub fn cudaStreamAddCallback(
        stream: cudaStream_t,
        callback: cudaStreamCallback_t,
        userData: *mut c_void,
        flags: c_uint,
    ) -> cudaError_t;

    /// Attach memory to a stream asynchronously.
    pub fn cudaStreamAttachMemAsync(
        stream: cudaStream_t,
        devPtr: *mut c_void,
        length: usize,
        flags: c_uint,
    ) -> cudaError_t;

    /// Begins graph capture on a stream.
    pub fn cudaStreamBeginCapture(
        stream: cudaStream_t,
        mode: cudaStreamCaptureMode,
    ) -> cudaError_t;

    /// Begins graph capture on a stream to an existing graph.
    pub fn cudaStreamBeginCaptureToGraph(
        stream: cudaStream_t,
        graph: cudaGraph_t,
        dependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        mode: cudaStreamCaptureMode,
    ) -> cudaError_t;

    /// Copies stream attributes.
    pub fn cudaStreamCopyAttributes(dst: cudaStream_t, src: cudaStream_t) -> cudaError_t;

    /// Create an asynchronous stream.
    pub fn cudaStreamCreate(pStream: *mut cudaStream_t) -> cudaError_t;

    /// Create an asynchronous stream with flags.
    pub fn cudaStreamCreateWithFlags(pStream: *mut cudaStream_t, flags: c_uint) -> cudaError_t;

    /// Create an asynchronous stream with the specified priority.
    pub fn cudaStreamCreateWithPriority(
        pStream: *mut cudaStream_t,
        flags: c_uint,
        priority: c_int,
    ) -> cudaError_t;

    /// Destroys and cleans up an asynchronous stream.
    pub fn cudaStreamDestroy(stream: cudaStream_t) -> cudaError_t;

    /// Ends capture on a stream, returning the captured graph.
    pub fn cudaStreamEndCapture(
        stream: cudaStream_t,
        pGraph: *mut cudaGraph_t,
    ) -> cudaError_t;

    /// Queries stream attribute.
    pub fn cudaStreamGetAttribute(
        hStream: cudaStream_t,
        attr: cudaStreamAttrID,
        value_out: *mut cudaStreamAttrValue,
    ) -> cudaError_t;

    /// Query capture status of a stream (v2 - without edge data).
    pub fn cudaStreamGetCaptureInfo_v2(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut u64,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        numDependencies_out: *mut usize,
    ) -> cudaError_t;

    /// Query capture status of a stream (v3 - with edge data).
    pub fn cudaStreamGetCaptureInfo_v3(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut u64,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        edgeData_out: *mut *const cudaGraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cudaError_t;

    /// Query the device associated with a stream.
    pub fn cudaStreamGetDevice(hStream: cudaStream_t, device: *mut c_int) -> cudaError_t;

    /// Query the flags of a stream.
    pub fn cudaStreamGetFlags(hStream: cudaStream_t, flags: *mut c_uint) -> cudaError_t;

    /// Query the Id of a stream.
    pub fn cudaStreamGetId(hStream: cudaStream_t, streamId: *mut u64) -> cudaError_t;

    /// Query the priority of a stream.
    pub fn cudaStreamGetPriority(hStream: cudaStream_t, priority: *mut c_int) -> cudaError_t;

    /// Returns a stream's capture status.
    pub fn cudaStreamIsCapturing(
        stream: cudaStream_t,
        pCaptureStatus: *mut cudaStreamCaptureStatus,
    ) -> cudaError_t;

    /// Queries an asynchronous stream for completion status.
    pub fn cudaStreamQuery(stream: cudaStream_t) -> cudaError_t;

    /// Sets stream attribute.
    pub fn cudaStreamSetAttribute(
        hStream: cudaStream_t,
        attr: cudaStreamAttrID,
        value: *const cudaStreamAttrValue,
    ) -> cudaError_t;

    /// Waits for stream tasks to complete.
    pub fn cudaStreamSynchronize(stream: cudaStream_t) -> cudaError_t;

    /// Update the set of dependencies in a capturing stream.
    pub fn cudaStreamUpdateCaptureDependencies(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        flags: c_uint,
    ) -> cudaError_t;

    /// Make a compute stream wait on an event.
    pub fn cudaStreamWaitEvent(
        stream: cudaStream_t,
        event: cudaEvent_t,
        flags: c_uint,
    ) -> cudaError_t;

    /// Swap the stream capture interaction mode for a thread.
    pub fn cudaThreadExchangeStreamCaptureMode(
        mode: *mut cudaStreamCaptureMode,
    ) -> cudaError_t;
}
