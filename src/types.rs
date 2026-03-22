//! CUDA Runtime API type definitions.
//!
//! All enumerations, structures, typedefs, and opaque handle types
//! used by the CUDA Runtime API.

use std::os::raw::{c_char, c_int, c_uint, c_void};

// ============================================================================
// Opaque Handle Types
// ============================================================================

/// Opaque CUDA stream handle.
pub enum CUstream_st {}
pub type cudaStream_t = *mut CUstream_st;

/// Opaque CUDA event handle.
pub enum CUevent_st {}
pub type cudaEvent_t = *mut CUevent_st;

/// Opaque CUDA graph handle.
pub enum CUgraph_st {}
pub type cudaGraph_t = *mut CUgraph_st;

/// Opaque CUDA graph node handle.
pub enum CUgraphNode_st {}
pub type cudaGraphNode_t = *mut CUgraphNode_st;

/// Opaque CUDA executable graph handle.
pub enum CUgraphExec_st {}
pub type cudaGraphExec_t = *mut CUgraphExec_st;

/// Opaque CUDA graph device node handle.
pub enum CUgraphDeviceNode_st {}
pub type cudaGraphDeviceNode_t = *mut CUgraphDeviceNode_st;

/// Opaque CUDA user object handle.
pub enum CUuserObject_st {}
pub type cudaUserObject_t = *mut CUuserObject_st;

/// Opaque CUDA function handle.
pub enum CUfunc_st {}
pub type cudaFunction_t = *mut CUfunc_st;

/// Opaque CUDA kernel handle.
pub enum CUkern_st {}
pub type cudaKernel_t = *mut CUkern_st;

/// Opaque CUDA memory pool handle.
pub enum CUmemPoolHandle_st {}
pub type cudaMemPool_t = *mut CUmemPoolHandle_st;

/// Opaque CUDA external memory handle.
pub enum CUextMemory_st {}
pub type cudaExternalMemory_t = *mut CUextMemory_st;

/// Opaque CUDA external semaphore handle.
pub enum CUextSemaphore_st {}
pub type cudaExternalSemaphore_t = *mut CUextSemaphore_st;

/// Opaque CUDA array handle.
pub enum cudaArray {}
pub type cudaArray_t = *mut cudaArray;
pub type cudaArray_const_t = *const cudaArray;

/// Opaque CUDA mipmapped array handle.
pub enum cudaMipmappedArray {}
pub type cudaMipmappedArray_t = *mut cudaMipmappedArray;
pub type cudaMipmappedArray_const_t = *const cudaMipmappedArray;

/// Opaque CUDA graphics resource handle.
pub enum cudaGraphicsResource {}
pub type cudaGraphicsResource_t = *mut cudaGraphicsResource;

/// Opaque CUDA library handle.
pub enum CUlib_st {}
pub type cudaLibrary_t = *mut CUlib_st;

/// Opaque CUDA execution context handle.
pub enum CUexecCtx_st {}
pub type cudaExecutionContext_t = *mut CUexecCtx_st;

/// Opaque CUDA async callback handle.
pub type cudaAsyncCallbackHandle_t = *mut c_void;

/// CUDA graph conditional handle.
pub type cudaGraphConditionalHandle = u64;

/// CUDA texture object.
pub type cudaTextureObject_t = u64;

/// CUDA surface object.
pub type cudaSurfaceObject_t = u64;

/// Host function pointer type.
pub type cudaHostFn_t = Option<unsafe extern "C" fn(userData: *mut c_void)>;

/// Stream callback function pointer type.
pub type cudaStreamCallback_t =
    Option<unsafe extern "C" fn(stream: cudaStream_t, status: cudaError_t, userData: *mut c_void)>;

/// Async notification callback function pointer type.
pub type cudaAsyncCallback = Option<
    unsafe extern "C" fn(
        info: *mut cudaAsyncNotificationInfo_t,
        userData: *mut c_void,
        callback: cudaAsyncCallbackHandle_t,
    ),
>;

// ============================================================================
// IPC Handle Types
// ============================================================================

/// IPC event handle.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaIpcEventHandle_t {
    pub reserved: [c_char; 64],
}

/// IPC memory handle.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaIpcMemHandle_t {
    pub reserved: [c_char; 64],
}

// ============================================================================
// UUID
// ============================================================================

/// CUDA UUID type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct cudaUUID_t {
    pub bytes: [c_char; 16],
}

// ============================================================================
// Error Type
// ============================================================================

/// CUDA error type. Use the `cuda*` constants for comparison.
pub type cudaError_t = c_int;

pub const cudaSuccess: cudaError_t = 0;
pub const cudaErrorInvalidValue: cudaError_t = 1;
pub const cudaErrorMemoryAllocation: cudaError_t = 2;
pub const cudaErrorInitializationError: cudaError_t = 3;
pub const cudaErrorCudartUnloading: cudaError_t = 4;
pub const cudaErrorProfilerDisabled: cudaError_t = 5;
pub const cudaErrorInvalidConfiguration: cudaError_t = 9;
pub const cudaErrorInvalidPitchValue: cudaError_t = 12;
pub const cudaErrorInvalidSymbol: cudaError_t = 13;
pub const cudaErrorInvalidTexture: cudaError_t = 18;
pub const cudaErrorInvalidTextureBinding: cudaError_t = 19;
pub const cudaErrorInvalidChannelDescriptor: cudaError_t = 20;
pub const cudaErrorInvalidMemcpyDirection: cudaError_t = 21;
pub const cudaErrorInvalidFilterSetting: cudaError_t = 26;
pub const cudaErrorInvalidNormSetting: cudaError_t = 27;
pub const cudaErrorStubLibrary: cudaError_t = 34;
pub const cudaErrorInsufficientDriver: cudaError_t = 35;
pub const cudaErrorCallRequiresNewerDriver: cudaError_t = 36;
pub const cudaErrorInvalidSurface: cudaError_t = 37;
pub const cudaErrorDuplicateVariableName: cudaError_t = 43;
pub const cudaErrorDuplicateTextureName: cudaError_t = 44;
pub const cudaErrorDuplicateSurfaceName: cudaError_t = 45;
pub const cudaErrorDevicesUnavailable: cudaError_t = 46;
pub const cudaErrorIncompatibleDriverContext: cudaError_t = 49;
pub const cudaErrorMissingConfiguration: cudaError_t = 52;
pub const cudaErrorLaunchMaxDepthExceeded: cudaError_t = 65;
pub const cudaErrorLaunchFileScopedTex: cudaError_t = 66;
pub const cudaErrorLaunchFileScopedSurf: cudaError_t = 67;
pub const cudaErrorSyncDepthExceeded: cudaError_t = 68;
pub const cudaErrorLaunchPendingCountExceeded: cudaError_t = 69;
pub const cudaErrorInvalidDeviceFunction: cudaError_t = 98;
pub const cudaErrorNoDevice: cudaError_t = 100;
pub const cudaErrorInvalidDevice: cudaError_t = 101;
pub const cudaErrorDeviceNotLicensed: cudaError_t = 102;
pub const cudaErrorSoftwareValidityNotEstablished: cudaError_t = 103;
pub const cudaErrorStartupFailure: cudaError_t = 127;
pub const cudaErrorInvalidKernelImage: cudaError_t = 200;
pub const cudaErrorDeviceUninitialized: cudaError_t = 201;
pub const cudaErrorMapBufferObjectFailed: cudaError_t = 205;
pub const cudaErrorUnmapBufferObjectFailed: cudaError_t = 206;
pub const cudaErrorArrayIsMapped: cudaError_t = 207;
pub const cudaErrorAlreadyMapped: cudaError_t = 208;
pub const cudaErrorNoKernelImageForDevice: cudaError_t = 209;
pub const cudaErrorAlreadyAcquired: cudaError_t = 210;
pub const cudaErrorNotMapped: cudaError_t = 211;
pub const cudaErrorNotMappedAsArray: cudaError_t = 212;
pub const cudaErrorNotMappedAsPointer: cudaError_t = 213;
pub const cudaErrorECCUncorrectable: cudaError_t = 214;
pub const cudaErrorUnsupportedLimit: cudaError_t = 215;
pub const cudaErrorDeviceAlreadyInUse: cudaError_t = 216;
pub const cudaErrorPeerAccessUnsupported: cudaError_t = 217;
pub const cudaErrorInvalidPtx: cudaError_t = 218;
pub const cudaErrorInvalidGraphicsContext: cudaError_t = 219;
pub const cudaErrorNvlinkUncorrectable: cudaError_t = 220;
pub const cudaErrorJitCompilerNotFound: cudaError_t = 221;
pub const cudaErrorUnsupportedPtxVersion: cudaError_t = 222;
pub const cudaErrorJitCompilationDisabled: cudaError_t = 223;
pub const cudaErrorUnsupportedExecAffinity: cudaError_t = 224;
pub const cudaErrorUnsupportedDevSideSync: cudaError_t = 225;
pub const cudaErrorInvalidSource: cudaError_t = 300;
pub const cudaErrorFileNotFound: cudaError_t = 301;
pub const cudaErrorSharedObjectSymbolNotFound: cudaError_t = 302;
pub const cudaErrorSharedObjectInitFailed: cudaError_t = 303;
pub const cudaErrorOperatingSystem: cudaError_t = 304;
pub const cudaErrorInvalidResourceHandle: cudaError_t = 400;
pub const cudaErrorIllegalState: cudaError_t = 401;
pub const cudaErrorLossyQuery: cudaError_t = 402;
pub const cudaErrorSymbolNotFound: cudaError_t = 500;
pub const cudaErrorNotReady: cudaError_t = 600;
pub const cudaErrorIllegalAddress: cudaError_t = 700;
pub const cudaErrorLaunchOutOfResources: cudaError_t = 701;
pub const cudaErrorLaunchTimeout: cudaError_t = 702;
pub const cudaErrorLaunchIncompatibleTexturing: cudaError_t = 703;
pub const cudaErrorPeerAccessAlreadyEnabled: cudaError_t = 704;
pub const cudaErrorPeerAccessNotEnabled: cudaError_t = 705;
pub const cudaErrorSetOnActiveProcess: cudaError_t = 708;
pub const cudaErrorContextIsDestroyed: cudaError_t = 709;
pub const cudaErrorAssert: cudaError_t = 710;
pub const cudaErrorTooManyPeers: cudaError_t = 711;
pub const cudaErrorHostMemoryAlreadyRegistered: cudaError_t = 712;
pub const cudaErrorHostMemoryNotRegistered: cudaError_t = 713;
pub const cudaErrorHardwareStackError: cudaError_t = 714;
pub const cudaErrorIllegalInstruction: cudaError_t = 715;
pub const cudaErrorMisalignedAddress: cudaError_t = 716;
pub const cudaErrorInvalidAddressSpace: cudaError_t = 717;
pub const cudaErrorInvalidPc: cudaError_t = 718;
pub const cudaErrorLaunchFailure: cudaError_t = 719;
pub const cudaErrorCooperativeLaunchTooLarge: cudaError_t = 720;
pub const cudaErrorNotPermitted: cudaError_t = 800;
pub const cudaErrorNotSupported: cudaError_t = 801;
pub const cudaErrorSystemNotReady: cudaError_t = 802;
pub const cudaErrorSystemDriverMismatch: cudaError_t = 803;
pub const cudaErrorCompatNotSupportedOnDevice: cudaError_t = 804;
pub const cudaErrorMpsConnectionFailed: cudaError_t = 805;
pub const cudaErrorMpsRpcFailure: cudaError_t = 806;
pub const cudaErrorMpsServerNotReady: cudaError_t = 807;
pub const cudaErrorMpsMaxClientsReached: cudaError_t = 808;
pub const cudaErrorMpsMaxConnectionsReached: cudaError_t = 809;
pub const cudaErrorMpsClientTerminated: cudaError_t = 810;
pub const cudaErrorCdpNotSupported: cudaError_t = 811;
pub const cudaErrorCdpVersionMismatch: cudaError_t = 812;
pub const cudaErrorStreamCaptureUnsupported: cudaError_t = 900;
pub const cudaErrorStreamCaptureInvalidated: cudaError_t = 901;
pub const cudaErrorStreamCaptureMerge: cudaError_t = 902;
pub const cudaErrorStreamCaptureUnmatched: cudaError_t = 903;
pub const cudaErrorStreamCaptureUnjoined: cudaError_t = 904;
pub const cudaErrorStreamCaptureIsolation: cudaError_t = 905;
pub const cudaErrorStreamCaptureImplicit: cudaError_t = 906;
pub const cudaErrorCapturedEvent: cudaError_t = 907;
pub const cudaErrorStreamCaptureWrongThread: cudaError_t = 908;
pub const cudaErrorTimeout: cudaError_t = 909;
pub const cudaErrorGraphExecUpdateFailure: cudaError_t = 910;
pub const cudaErrorExternalDevice: cudaError_t = 911;
pub const cudaErrorInvalidClusterSize: cudaError_t = 912;
pub const cudaErrorFunctionNotLoaded: cudaError_t = 913;
pub const cudaErrorInvalidResourceType: cudaError_t = 914;
pub const cudaErrorInvalidResourceConfiguration: cudaError_t = 915;
pub const cudaErrorUnknown: cudaError_t = 999;

// ============================================================================
// Enumerations (as type aliases + constants)
// ============================================================================

/// Memory copy direction.
pub type cudaMemcpyKind = c_int;
pub const cudaMemcpyHostToHost: cudaMemcpyKind = 0;
pub const cudaMemcpyHostToDevice: cudaMemcpyKind = 1;
pub const cudaMemcpyDeviceToHost: cudaMemcpyKind = 2;
pub const cudaMemcpyDeviceToDevice: cudaMemcpyKind = 3;
pub const cudaMemcpyDefault: cudaMemcpyKind = 4;

/// Compute mode.
pub type cudaComputeMode = c_int;
pub const cudaComputeModeDefault: cudaComputeMode = 0;
pub const cudaComputeModeExclusive: cudaComputeMode = 1;
pub const cudaComputeModeProhibited: cudaComputeMode = 2;
pub const cudaComputeModeExclusiveProcess: cudaComputeMode = 3;

/// Device attribute.
pub type cudaDeviceAttr = c_int;
pub const cudaDevAttrMaxThreadsPerBlock: cudaDeviceAttr = 1;
pub const cudaDevAttrMaxBlockDimX: cudaDeviceAttr = 2;
pub const cudaDevAttrMaxBlockDimY: cudaDeviceAttr = 3;
pub const cudaDevAttrMaxBlockDimZ: cudaDeviceAttr = 4;
pub const cudaDevAttrMaxGridDimX: cudaDeviceAttr = 5;
pub const cudaDevAttrMaxGridDimY: cudaDeviceAttr = 6;
pub const cudaDevAttrMaxGridDimZ: cudaDeviceAttr = 7;
pub const cudaDevAttrMaxSharedMemoryPerBlock: cudaDeviceAttr = 8;
pub const cudaDevAttrTotalConstantMemory: cudaDeviceAttr = 9;
pub const cudaDevAttrWarpSize: cudaDeviceAttr = 10;
pub const cudaDevAttrMaxPitch: cudaDeviceAttr = 11;
pub const cudaDevAttrMaxRegistersPerBlock: cudaDeviceAttr = 12;
pub const cudaDevAttrClockRate: cudaDeviceAttr = 13;
pub const cudaDevAttrTextureAlignment: cudaDeviceAttr = 14;
pub const cudaDevAttrGpuOverlap: cudaDeviceAttr = 15;
pub const cudaDevAttrMultiProcessorCount: cudaDeviceAttr = 16;
pub const cudaDevAttrKernelExecTimeout: cudaDeviceAttr = 17;
pub const cudaDevAttrIntegrated: cudaDeviceAttr = 18;
pub const cudaDevAttrCanMapHostMemory: cudaDeviceAttr = 19;
pub const cudaDevAttrComputeMode: cudaDeviceAttr = 20;
pub const cudaDevAttrMaxTexture1DWidth: cudaDeviceAttr = 21;
pub const cudaDevAttrMaxTexture2DWidth: cudaDeviceAttr = 22;
pub const cudaDevAttrMaxTexture2DHeight: cudaDeviceAttr = 23;
pub const cudaDevAttrMaxTexture3DWidth: cudaDeviceAttr = 24;
pub const cudaDevAttrMaxTexture3DHeight: cudaDeviceAttr = 25;
pub const cudaDevAttrMaxTexture3DDepth: cudaDeviceAttr = 26;
pub const cudaDevAttrMaxTexture2DLayeredWidth: cudaDeviceAttr = 27;
pub const cudaDevAttrMaxTexture2DLayeredHeight: cudaDeviceAttr = 28;
pub const cudaDevAttrMaxTexture2DLayeredLayers: cudaDeviceAttr = 29;
pub const cudaDevAttrSurfaceAlignment: cudaDeviceAttr = 30;
pub const cudaDevAttrConcurrentKernels: cudaDeviceAttr = 31;
pub const cudaDevAttrEccEnabled: cudaDeviceAttr = 32;
pub const cudaDevAttrPciBusId: cudaDeviceAttr = 33;
pub const cudaDevAttrPciDeviceId: cudaDeviceAttr = 34;
pub const cudaDevAttrTccDriver: cudaDeviceAttr = 35;
pub const cudaDevAttrMemoryClockRate: cudaDeviceAttr = 36;
pub const cudaDevAttrGlobalMemoryBusWidth: cudaDeviceAttr = 37;
pub const cudaDevAttrL2CacheSize: cudaDeviceAttr = 38;
pub const cudaDevAttrMaxThreadsPerMultiProcessor: cudaDeviceAttr = 39;
pub const cudaDevAttrAsyncEngineCount: cudaDeviceAttr = 40;
pub const cudaDevAttrUnifiedAddressing: cudaDeviceAttr = 41;
pub const cudaDevAttrMaxTexture1DLayeredWidth: cudaDeviceAttr = 42;
pub const cudaDevAttrMaxTexture1DLayeredLayers: cudaDeviceAttr = 43;
pub const cudaDevAttrMaxTexture2DGatherWidth: cudaDeviceAttr = 45;
pub const cudaDevAttrMaxTexture2DGatherHeight: cudaDeviceAttr = 46;
pub const cudaDevAttrMaxTexture3DWidthAlt: cudaDeviceAttr = 47;
pub const cudaDevAttrMaxTexture3DHeightAlt: cudaDeviceAttr = 48;
pub const cudaDevAttrMaxTexture3DDepthAlt: cudaDeviceAttr = 49;
pub const cudaDevAttrPciDomainId: cudaDeviceAttr = 50;
pub const cudaDevAttrTexturePitchAlignment: cudaDeviceAttr = 51;
pub const cudaDevAttrMaxTextureCubemapWidth: cudaDeviceAttr = 52;
pub const cudaDevAttrMaxTextureCubemapLayeredWidth: cudaDeviceAttr = 53;
pub const cudaDevAttrMaxTextureCubemapLayeredLayers: cudaDeviceAttr = 54;
pub const cudaDevAttrMaxSurface1DWidth: cudaDeviceAttr = 55;
pub const cudaDevAttrMaxSurface2DWidth: cudaDeviceAttr = 56;
pub const cudaDevAttrMaxSurface2DHeight: cudaDeviceAttr = 57;
pub const cudaDevAttrMaxSurface3DWidth: cudaDeviceAttr = 58;
pub const cudaDevAttrMaxSurface3DHeight: cudaDeviceAttr = 59;
pub const cudaDevAttrMaxSurface3DDepth: cudaDeviceAttr = 60;
pub const cudaDevAttrMaxSurface1DLayeredWidth: cudaDeviceAttr = 61;
pub const cudaDevAttrMaxSurface1DLayeredLayers: cudaDeviceAttr = 62;
pub const cudaDevAttrMaxSurface2DLayeredWidth: cudaDeviceAttr = 63;
pub const cudaDevAttrMaxSurface2DLayeredHeight: cudaDeviceAttr = 64;
pub const cudaDevAttrMaxSurface2DLayeredLayers: cudaDeviceAttr = 65;
pub const cudaDevAttrMaxSurfaceCubemapWidth: cudaDeviceAttr = 66;
pub const cudaDevAttrMaxSurfaceCubemapLayeredWidth: cudaDeviceAttr = 67;
pub const cudaDevAttrMaxSurfaceCubemapLayeredLayers: cudaDeviceAttr = 68;
pub const cudaDevAttrMaxTexture1DLinearWidth: cudaDeviceAttr = 69;
pub const cudaDevAttrMaxTexture2DLinearWidth: cudaDeviceAttr = 70;
pub const cudaDevAttrMaxTexture2DLinearHeight: cudaDeviceAttr = 71;
pub const cudaDevAttrMaxTexture2DLinearPitch: cudaDeviceAttr = 72;
pub const cudaDevAttrMaxTexture2DMipmappedWidth: cudaDeviceAttr = 73;
pub const cudaDevAttrMaxTexture2DMipmappedHeight: cudaDeviceAttr = 74;
pub const cudaDevAttrComputeCapabilityMajor: cudaDeviceAttr = 75;
pub const cudaDevAttrComputeCapabilityMinor: cudaDeviceAttr = 76;
pub const cudaDevAttrMaxTexture1DMipmappedWidth: cudaDeviceAttr = 77;
pub const cudaDevAttrStreamPrioritiesSupported: cudaDeviceAttr = 78;
pub const cudaDevAttrGlobalL1CacheSupported: cudaDeviceAttr = 79;
pub const cudaDevAttrLocalL1CacheSupported: cudaDeviceAttr = 80;
pub const cudaDevAttrMaxSharedMemoryPerMultiprocessor: cudaDeviceAttr = 81;
pub const cudaDevAttrMaxRegistersPerMultiprocessor: cudaDeviceAttr = 82;
pub const cudaDevAttrManagedMemory: cudaDeviceAttr = 83;
pub const cudaDevAttrIsMultiGpuBoard: cudaDeviceAttr = 84;
pub const cudaDevAttrMultiGpuBoardGroupID: cudaDeviceAttr = 85;
pub const cudaDevAttrHostNativeAtomicSupported: cudaDeviceAttr = 86;
pub const cudaDevAttrSingleToDoublePrecisionPerfRatio: cudaDeviceAttr = 87;
pub const cudaDevAttrPageableMemoryAccess: cudaDeviceAttr = 88;
pub const cudaDevAttrConcurrentManagedAccess: cudaDeviceAttr = 89;
pub const cudaDevAttrComputePreemptionSupported: cudaDeviceAttr = 90;
pub const cudaDevAttrCanUseHostPointerForRegisteredMem: cudaDeviceAttr = 91;
pub const cudaDevAttrCooperativeLaunch: cudaDeviceAttr = 95;
pub const cudaDevAttrCooperativeMultiDeviceLaunch: cudaDeviceAttr = 96;
pub const cudaDevAttrMaxSharedMemoryPerBlockOptin: cudaDeviceAttr = 97;
pub const cudaDevAttrCanFlushRemoteWrites: cudaDeviceAttr = 98;
pub const cudaDevAttrHostRegisterSupported: cudaDeviceAttr = 99;
pub const cudaDevAttrPageableMemoryAccessUsesHostPageTables: cudaDeviceAttr = 100;
pub const cudaDevAttrDirectManagedMemAccessFromHost: cudaDeviceAttr = 101;
pub const cudaDevAttrMaxBlocksPerMultiprocessor: cudaDeviceAttr = 106;
pub const cudaDevAttrMaxPersistingL2CacheSize: cudaDeviceAttr = 108;
pub const cudaDevAttrMaxAccessPolicyWindowSize: cudaDeviceAttr = 109;
pub const cudaDevAttrReservedSharedMemoryPerBlock: cudaDeviceAttr = 111;
pub const cudaDevAttrSparseCudaArraySupported: cudaDeviceAttr = 112;
pub const cudaDevAttrHostRegisterReadOnlySupported: cudaDeviceAttr = 113;
pub const cudaDevAttrTimelineSemaphoreInteropSupported: cudaDeviceAttr = 114;
pub const cudaDevAttrMaxTimelineSemaphoreInteropSupported: cudaDeviceAttr = 114;
pub const cudaDevAttrMemoryPoolsSupported: cudaDeviceAttr = 115;
pub const cudaDevAttrGPUDirectRDMASupported: cudaDeviceAttr = 116;
pub const cudaDevAttrGPUDirectRDMAFlushWritesOptions: cudaDeviceAttr = 117;
pub const cudaDevAttrGPUDirectRDMAWritesOrdering: cudaDeviceAttr = 118;
pub const cudaDevAttrMemoryPoolSupportedHandleTypes: cudaDeviceAttr = 119;
pub const cudaDevAttrClusterLaunch: cudaDeviceAttr = 120;
pub const cudaDevAttrDeferredMappingCudaArraySupported: cudaDeviceAttr = 121;
pub const cudaDevAttrIpcEventSupported: cudaDeviceAttr = 122;
pub const cudaDevAttrUnifiedFunctionPointers: cudaDeviceAttr = 123;
pub const cudaDevAttrNumaConfig: cudaDeviceAttr = 124;
pub const cudaDevAttrNumaId: cudaDeviceAttr = 125;
pub const cudaDevAttrMpsEnabled: cudaDeviceAttr = 126;
pub const cudaDevAttrHostNumaId: cudaDeviceAttr = 127;
pub const cudaDevAttrD3D12CigSupported: cudaDeviceAttr = 128;

/// Peer-to-peer device attribute.
pub type cudaDeviceP2PAttr = c_int;
pub const cudaDevP2PAttrPerformanceRank: cudaDeviceP2PAttr = 1;
pub const cudaDevP2PAttrAccessSupported: cudaDeviceP2PAttr = 2;
pub const cudaDevP2PAttrNativeAtomicSupported: cudaDeviceP2PAttr = 3;
pub const cudaDevP2PAttrCudaArrayAccessSupported: cudaDeviceP2PAttr = 4;

/// Function cache configuration.
pub type cudaFuncCache = c_int;
pub const cudaFuncCachePreferNone: cudaFuncCache = 0;
pub const cudaFuncCachePreferShared: cudaFuncCache = 1;
pub const cudaFuncCachePreferL1: cudaFuncCache = 2;
pub const cudaFuncCachePreferEqual: cudaFuncCache = 3;

/// Device limit.
pub type cudaLimit = c_int;
pub const cudaLimitStackSize: cudaLimit = 0x00;
pub const cudaLimitPrintfFifoSize: cudaLimit = 0x01;
pub const cudaLimitMallocHeapSize: cudaLimit = 0x02;
pub const cudaLimitDevRuntimeSyncDepth: cudaLimit = 0x03;
pub const cudaLimitDevRuntimePendingLaunchCount: cudaLimit = 0x04;
pub const cudaLimitMaxL2FetchGranularity: cudaLimit = 0x05;
pub const cudaLimitPersistingL2CacheSize: cudaLimit = 0x06;

/// Memory advise.
pub type cudaMemoryAdvise = c_int;
pub const cudaMemAdviseSetReadMostly: cudaMemoryAdvise = 1;
pub const cudaMemAdviseUnsetReadMostly: cudaMemoryAdvise = 2;
pub const cudaMemAdviseSetPreferredLocation: cudaMemoryAdvise = 3;
pub const cudaMemAdviseUnsetPreferredLocation: cudaMemoryAdvise = 4;
pub const cudaMemAdviseSetAccessedBy: cudaMemoryAdvise = 5;
pub const cudaMemAdviseUnsetAccessedBy: cudaMemoryAdvise = 6;

/// Memory range attribute.
pub type cudaMemRangeAttribute = c_int;
pub const cudaMemRangeAttributeReadMostly: cudaMemRangeAttribute = 1;
pub const cudaMemRangeAttributePreferredLocation: cudaMemRangeAttribute = 2;
pub const cudaMemRangeAttributeAccessedBy: cudaMemRangeAttribute = 3;
pub const cudaMemRangeAttributeLastPrefetchLocation: cudaMemRangeAttribute = 4;

/// Function attribute.
pub type cudaFuncAttribute = c_int;
pub const cudaFuncAttributeMaxDynamicSharedMemorySize: cudaFuncAttribute = 8;
pub const cudaFuncAttributePreferredSharedMemoryCarveout: cudaFuncAttribute = 9;
pub const cudaFuncAttributeClusterDimMustBeSet: cudaFuncAttribute = 10;
pub const cudaFuncAttributeRequiredClusterWidth: cudaFuncAttribute = 11;
pub const cudaFuncAttributeRequiredClusterHeight: cudaFuncAttribute = 12;
pub const cudaFuncAttributeRequiredClusterDepth: cudaFuncAttribute = 13;
pub const cudaFuncAttributeNonPortableClusterSizeAllowed: cudaFuncAttribute = 14;
pub const cudaFuncAttributeClusterSchedulingPolicyPreference: cudaFuncAttribute = 15;

/// Stream capture mode.
pub type cudaStreamCaptureMode = c_int;
pub const cudaStreamCaptureModeGlobal: cudaStreamCaptureMode = 0;
pub const cudaStreamCaptureModeThreadLocal: cudaStreamCaptureMode = 1;
pub const cudaStreamCaptureModeRelaxed: cudaStreamCaptureMode = 2;

/// Stream capture status.
pub type cudaStreamCaptureStatus = c_int;
pub const cudaStreamCaptureStatusNone: cudaStreamCaptureStatus = 0;
pub const cudaStreamCaptureStatusActive: cudaStreamCaptureStatus = 1;
pub const cudaStreamCaptureStatusInvalidated: cudaStreamCaptureStatus = 2;

/// Channel format kind.
pub type cudaChannelFormatKind = c_int;
pub const cudaChannelFormatKindSigned: cudaChannelFormatKind = 0;
pub const cudaChannelFormatKindUnsigned: cudaChannelFormatKind = 1;
pub const cudaChannelFormatKindFloat: cudaChannelFormatKind = 2;
pub const cudaChannelFormatKindNone: cudaChannelFormatKind = 3;
pub const cudaChannelFormatKindNV12: cudaChannelFormatKind = 4;
pub const cudaChannelFormatKindUnsignedNormalized8X1: cudaChannelFormatKind = 5;
pub const cudaChannelFormatKindUnsignedNormalized8X2: cudaChannelFormatKind = 6;
pub const cudaChannelFormatKindUnsignedNormalized8X4: cudaChannelFormatKind = 7;
pub const cudaChannelFormatKindUnsignedNormalized16X1: cudaChannelFormatKind = 8;
pub const cudaChannelFormatKindUnsignedNormalized16X2: cudaChannelFormatKind = 9;
pub const cudaChannelFormatKindUnsignedNormalized16X4: cudaChannelFormatKind = 10;
pub const cudaChannelFormatKindSignedNormalized8X1: cudaChannelFormatKind = 11;
pub const cudaChannelFormatKindSignedNormalized8X2: cudaChannelFormatKind = 12;
pub const cudaChannelFormatKindSignedNormalized8X4: cudaChannelFormatKind = 13;
pub const cudaChannelFormatKindSignedNormalized16X1: cudaChannelFormatKind = 14;
pub const cudaChannelFormatKindSignedNormalized16X2: cudaChannelFormatKind = 15;
pub const cudaChannelFormatKindSignedNormalized16X4: cudaChannelFormatKind = 16;
pub const cudaChannelFormatKindUnsignedBlockCompressed1: cudaChannelFormatKind = 17;
pub const cudaChannelFormatKindUnsignedBlockCompressed1SRGB: cudaChannelFormatKind = 18;
pub const cudaChannelFormatKindSignedBlockCompressed4: cudaChannelFormatKind = 19;
pub const cudaChannelFormatKindUnsignedBlockCompressed4: cudaChannelFormatKind = 20;
pub const cudaChannelFormatKindSignedBlockCompressed5: cudaChannelFormatKind = 21;
pub const cudaChannelFormatKindUnsignedBlockCompressed5: cudaChannelFormatKind = 22;
pub const cudaChannelFormatKindUnsignedBlockCompressed6H: cudaChannelFormatKind = 23;
pub const cudaChannelFormatKindSignedBlockCompressed6H: cudaChannelFormatKind = 24;
pub const cudaChannelFormatKindUnsignedBlockCompressed7: cudaChannelFormatKind = 25;
pub const cudaChannelFormatKindUnsignedBlockCompressed7SRGB: cudaChannelFormatKind = 26;

/// Resource type.
pub type cudaResourceType = c_int;
pub const cudaResourceTypeArray: cudaResourceType = 0x00;
pub const cudaResourceTypeMipmappedArray: cudaResourceType = 0x01;
pub const cudaResourceTypeLinear: cudaResourceType = 0x02;
pub const cudaResourceTypePitch2D: cudaResourceType = 0x03;

/// Resource view format.
pub type cudaResourceViewFormat = c_int;
pub const cudaResViewFormatNone: cudaResourceViewFormat = 0x00;
pub const cudaResViewFormatUnsignedChar1: cudaResourceViewFormat = 0x01;
pub const cudaResViewFormatUnsignedChar2: cudaResourceViewFormat = 0x02;
pub const cudaResViewFormatUnsignedChar4: cudaResourceViewFormat = 0x03;
pub const cudaResViewFormatSignedChar1: cudaResourceViewFormat = 0x04;
pub const cudaResViewFormatSignedChar2: cudaResourceViewFormat = 0x05;
pub const cudaResViewFormatSignedChar4: cudaResourceViewFormat = 0x06;
pub const cudaResViewFormatUnsignedShort1: cudaResourceViewFormat = 0x07;
pub const cudaResViewFormatUnsignedShort2: cudaResourceViewFormat = 0x08;
pub const cudaResViewFormatUnsignedShort4: cudaResourceViewFormat = 0x09;
pub const cudaResViewFormatSignedShort1: cudaResourceViewFormat = 0x0a;
pub const cudaResViewFormatSignedShort2: cudaResourceViewFormat = 0x0b;
pub const cudaResViewFormatSignedShort4: cudaResourceViewFormat = 0x0c;
pub const cudaResViewFormatUnsignedInt1: cudaResourceViewFormat = 0x0d;
pub const cudaResViewFormatUnsignedInt2: cudaResourceViewFormat = 0x0e;
pub const cudaResViewFormatUnsignedInt4: cudaResourceViewFormat = 0x0f;
pub const cudaResViewFormatSignedInt1: cudaResourceViewFormat = 0x10;
pub const cudaResViewFormatSignedInt2: cudaResourceViewFormat = 0x11;
pub const cudaResViewFormatSignedInt4: cudaResourceViewFormat = 0x12;
pub const cudaResViewFormatHalf1: cudaResourceViewFormat = 0x13;
pub const cudaResViewFormatHalf2: cudaResourceViewFormat = 0x14;
pub const cudaResViewFormatHalf4: cudaResourceViewFormat = 0x15;
pub const cudaResViewFormatFloat1: cudaResourceViewFormat = 0x16;
pub const cudaResViewFormatFloat2: cudaResourceViewFormat = 0x17;
pub const cudaResViewFormatFloat4: cudaResourceViewFormat = 0x18;
pub const cudaResViewFormatUnsignedBlockCompressed1: cudaResourceViewFormat = 0x19;
pub const cudaResViewFormatUnsignedBlockCompressed2: cudaResourceViewFormat = 0x1a;
pub const cudaResViewFormatUnsignedBlockCompressed3: cudaResourceViewFormat = 0x1b;
pub const cudaResViewFormatUnsignedBlockCompressed4: cudaResourceViewFormat = 0x1c;
pub const cudaResViewFormatSignedBlockCompressed4: cudaResourceViewFormat = 0x1d;
pub const cudaResViewFormatUnsignedBlockCompressed5: cudaResourceViewFormat = 0x1e;
pub const cudaResViewFormatSignedBlockCompressed5: cudaResourceViewFormat = 0x1f;
pub const cudaResViewFormatUnsignedBlockCompressed6H: cudaResourceViewFormat = 0x20;
pub const cudaResViewFormatSignedBlockCompressed6H: cudaResourceViewFormat = 0x21;
pub const cudaResViewFormatUnsignedBlockCompressed7: cudaResourceViewFormat = 0x22;

/// Texture address mode.
pub type cudaTextureAddressMode = c_int;
pub const cudaAddressModeWrap: cudaTextureAddressMode = 0;
pub const cudaAddressModeMirror: cudaTextureAddressMode = 1;
pub const cudaAddressModeClamp: cudaTextureAddressMode = 2;
pub const cudaAddressModeBorder: cudaTextureAddressMode = 3;

/// Texture filter mode.
pub type cudaTextureFilterMode = c_int;
pub const cudaFilterModePoint: cudaTextureFilterMode = 0;
pub const cudaFilterModeLinear: cudaTextureFilterMode = 1;

/// Texture read mode.
pub type cudaTextureReadMode = c_int;
pub const cudaReadModeElementType: cudaTextureReadMode = 0;
pub const cudaReadModeNormalizedFloat: cudaTextureReadMode = 1;

/// Graph node type.
pub type cudaGraphNodeType = c_int;
pub const cudaGraphNodeTypeKernel: cudaGraphNodeType = 0x00;
pub const cudaGraphNodeTypeMemcpy: cudaGraphNodeType = 0x01;
pub const cudaGraphNodeTypeMemset: cudaGraphNodeType = 0x02;
pub const cudaGraphNodeTypeHost: cudaGraphNodeType = 0x03;
pub const cudaGraphNodeTypeGraph: cudaGraphNodeType = 0x04;
pub const cudaGraphNodeTypeEmpty: cudaGraphNodeType = 0x05;
pub const cudaGraphNodeTypeWaitEvent: cudaGraphNodeType = 0x06;
pub const cudaGraphNodeTypeEventRecord: cudaGraphNodeType = 0x07;
pub const cudaGraphNodeTypeExtSemaphoreSignal: cudaGraphNodeType = 0x08;
pub const cudaGraphNodeTypeExtSemaphoreWait: cudaGraphNodeType = 0x09;
pub const cudaGraphNodeTypeMemAlloc: cudaGraphNodeType = 0x0a;
pub const cudaGraphNodeTypeMemFree: cudaGraphNodeType = 0x0b;
pub const cudaGraphNodeTypeConditional: cudaGraphNodeType = 0x0d;

/// Graph exec update result.
pub type cudaGraphExecUpdateResult = c_int;
pub const cudaGraphExecUpdateSuccess: cudaGraphExecUpdateResult = 0x0;
pub const cudaGraphExecUpdateError: cudaGraphExecUpdateResult = 0x1;
pub const cudaGraphExecUpdateErrorTopologyChanged: cudaGraphExecUpdateResult = 0x2;
pub const cudaGraphExecUpdateErrorNodeTypeChanged: cudaGraphExecUpdateResult = 0x3;
pub const cudaGraphExecUpdateErrorFunctionChanged: cudaGraphExecUpdateResult = 0x4;
pub const cudaGraphExecUpdateErrorParametersChanged: cudaGraphExecUpdateResult = 0x5;
pub const cudaGraphExecUpdateErrorNotSupported: cudaGraphExecUpdateResult = 0x6;
pub const cudaGraphExecUpdateErrorUnsupportedFunctionChange: cudaGraphExecUpdateResult = 0x7;

/// Graph memory attribute type.
pub type cudaGraphMemAttributeType = c_int;
pub const cudaGraphMemAttrUsedMemCurrent: cudaGraphMemAttributeType = 0;
pub const cudaGraphMemAttrUsedMemHigh: cudaGraphMemAttributeType = 1;
pub const cudaGraphMemAttrReservedMemCurrent: cudaGraphMemAttributeType = 2;
pub const cudaGraphMemAttrReservedMemHigh: cudaGraphMemAttributeType = 3;

/// GPU Direct RDMA writes target.
pub type cudaFlushGPUDirectRDMAWritesTarget = c_int;
pub const cudaFlushGPUDirectRDMAWritesTargetCurrentDevice: cudaFlushGPUDirectRDMAWritesTarget = 0;

/// GPU Direct RDMA writes scope.
pub type cudaFlushGPUDirectRDMAWritesScope = c_int;
pub const cudaFlushGPUDirectRDMAWritesToOwner: cudaFlushGPUDirectRDMAWritesScope = 100;
pub const cudaFlushGPUDirectRDMAWritesToAllDevices: cudaFlushGPUDirectRDMAWritesScope = 200;

/// Memory access flags.
pub type cudaMemAccessFlags = c_int;
pub const cudaMemAccessFlagsProtNone: cudaMemAccessFlags = 0;
pub const cudaMemAccessFlagsProtRead: cudaMemAccessFlags = 1;
pub const cudaMemAccessFlagsProtReadWrite: cudaMemAccessFlags = 3;

/// Memory location type.
pub type cudaMemLocationType = c_int;
pub const cudaMemLocationTypeInvalid: cudaMemLocationType = 0;
pub const cudaMemLocationTypeDevice: cudaMemLocationType = 1;
pub const cudaMemLocationTypeHost: cudaMemLocationType = 2;
pub const cudaMemLocationTypeHostNuma: cudaMemLocationType = 3;
pub const cudaMemLocationTypeHostNumaCurrentDevice: cudaMemLocationType = 4;

/// Memory allocation type.
pub type cudaMemAllocationType = c_int;
pub const cudaMemAllocationTypeInvalid: cudaMemAllocationType = 0x0;
pub const cudaMemAllocationTypePinned: cudaMemAllocationType = 0x1;
pub const cudaMemAllocationTypeMax: cudaMemAllocationType = 0x7FFFFFFF;

/// Memory allocation handle type.
pub type cudaMemAllocationHandleType = c_int;
pub const cudaMemHandleTypeNone: cudaMemAllocationHandleType = 0x0;
pub const cudaMemHandleTypePosixFileDescriptor: cudaMemAllocationHandleType = 0x1;
pub const cudaMemHandleTypeWin32: cudaMemAllocationHandleType = 0x2;
pub const cudaMemHandleTypeWin32Kmt: cudaMemAllocationHandleType = 0x4;
pub const cudaMemHandleTypeFabric: cudaMemAllocationHandleType = 0x8;

/// Memory pool attribute.
pub type cudaMemPoolAttr = c_int;
pub const cudaMemPoolReuseFollowEventDependencies: cudaMemPoolAttr = 0x1;
pub const cudaMemPoolReuseAllowOpportunistic: cudaMemPoolAttr = 0x2;
pub const cudaMemPoolReuseAllowInternalDependencies: cudaMemPoolAttr = 0x3;
pub const cudaMemPoolAttrReleaseThreshold: cudaMemPoolAttr = 0x4;
pub const cudaMemPoolAttrReservedMemCurrent: cudaMemPoolAttr = 0x5;
pub const cudaMemPoolAttrReservedMemHigh: cudaMemPoolAttr = 0x6;
pub const cudaMemPoolAttrUsedMemCurrent: cudaMemPoolAttr = 0x7;
pub const cudaMemPoolAttrUsedMemHigh: cudaMemPoolAttr = 0x8;

/// External memory handle type.
pub type cudaExternalMemoryHandleType = c_int;
pub const cudaExternalMemoryHandleTypeOpaqueFd: cudaExternalMemoryHandleType = 1;
pub const cudaExternalMemoryHandleTypeOpaqueWin32: cudaExternalMemoryHandleType = 2;
pub const cudaExternalMemoryHandleTypeOpaqueWin32Kmt: cudaExternalMemoryHandleType = 3;
pub const cudaExternalMemoryHandleTypeD3D12Heap: cudaExternalMemoryHandleType = 4;
pub const cudaExternalMemoryHandleTypeD3D12Resource: cudaExternalMemoryHandleType = 5;
pub const cudaExternalMemoryHandleTypeD3D11Resource: cudaExternalMemoryHandleType = 6;
pub const cudaExternalMemoryHandleTypeD3D11ResourceKmt: cudaExternalMemoryHandleType = 7;
pub const cudaExternalMemoryHandleTypeNvSciBuf: cudaExternalMemoryHandleType = 8;

/// External semaphore handle type.
pub type cudaExternalSemaphoreHandleType = c_int;
pub const cudaExternalSemaphoreHandleTypeOpaqueFd: cudaExternalSemaphoreHandleType = 1;
pub const cudaExternalSemaphoreHandleTypeOpaqueWin32: cudaExternalSemaphoreHandleType = 2;
pub const cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt: cudaExternalSemaphoreHandleType = 3;
pub const cudaExternalSemaphoreHandleTypeD3D12Fence: cudaExternalSemaphoreHandleType = 4;
pub const cudaExternalSemaphoreHandleTypeD3D11Fence: cudaExternalSemaphoreHandleType = 5;
pub const cudaExternalSemaphoreHandleTypeNvSciSync: cudaExternalSemaphoreHandleType = 6;
pub const cudaExternalSemaphoreHandleTypeKeyedMutex: cudaExternalSemaphoreHandleType = 7;
pub const cudaExternalSemaphoreHandleTypeKeyedMutexKmt: cudaExternalSemaphoreHandleType = 8;
pub const cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd: cudaExternalSemaphoreHandleType = 9;
pub const cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32: cudaExternalSemaphoreHandleType =
    10;

/// Graphics register flags.
pub type cudaGraphicsRegisterFlags = c_uint;
pub const cudaGraphicsRegisterFlagsNone: cudaGraphicsRegisterFlags = 0;
pub const cudaGraphicsRegisterFlagsReadOnly: cudaGraphicsRegisterFlags = 1;
pub const cudaGraphicsRegisterFlagsWriteDiscard: cudaGraphicsRegisterFlags = 2;
pub const cudaGraphicsRegisterFlagsSurfaceLoadStore: cudaGraphicsRegisterFlags = 4;
pub const cudaGraphicsRegisterFlagsTextureGather: cudaGraphicsRegisterFlags = 8;

/// Graphics map flags.
pub type cudaGraphicsMapFlags = c_uint;
pub const cudaGraphicsMapFlagsNone: cudaGraphicsMapFlags = 0;
pub const cudaGraphicsMapFlagsReadOnly: cudaGraphicsMapFlags = 1;
pub const cudaGraphicsMapFlagsWriteDiscard: cudaGraphicsMapFlags = 2;

/// Shared memory config.
pub type cudaSharedMemConfig = c_int;
pub const cudaSharedMemBankSizeDefault: cudaSharedMemConfig = 0;
pub const cudaSharedMemBankSizeFourByte: cudaSharedMemConfig = 1;
pub const cudaSharedMemBankSizeEightByte: cudaSharedMemConfig = 2;

/// Access property.
pub type cudaAccessProperty = c_int;
pub const cudaAccessPropertyNormal: cudaAccessProperty = 0;
pub const cudaAccessPropertyStreaming: cudaAccessProperty = 1;
pub const cudaAccessPropertyPersisting: cudaAccessProperty = 2;

/// Stream attribute ID.
pub type cudaStreamAttrID = c_int;
pub const cudaStreamAttributeAccessPolicyWindow: cudaStreamAttrID = 1;
pub const cudaStreamAttributeSynchronizationPolicy: cudaStreamAttrID = 3;
pub const cudaStreamAttributeMemSyncDomainMap: cudaStreamAttrID = 8;
pub const cudaStreamAttributeMemSyncDomain: cudaStreamAttrID = 9;
pub const cudaStreamAttributePriority: cudaStreamAttrID = 12;

/// Kernel node attribute ID.
pub type cudaKernelNodeAttrID = c_int;
pub const cudaKernelNodeAttributeAccessPolicyWindow: cudaKernelNodeAttrID = 1;
pub const cudaKernelNodeAttributeCooperative: cudaKernelNodeAttrID = 2;
pub const cudaKernelNodeAttributePriority: cudaKernelNodeAttrID = 8;
pub const cudaKernelNodeAttributeClusterDimension: cudaKernelNodeAttrID = 19;
pub const cudaKernelNodeAttributeClusterSchedulingPolicyPreference: cudaKernelNodeAttrID = 20;
pub const cudaKernelNodeAttributeMemSyncDomainMap: cudaKernelNodeAttrID = 12;
pub const cudaKernelNodeAttributeMemSyncDomain: cudaKernelNodeAttrID = 13;
pub const cudaKernelNodeAttributeDeviceUpdatableKernelNode: cudaKernelNodeAttrID = 15;

/// Launch attribute ID.
pub type cudaLaunchAttributeID = c_int;
pub const cudaLaunchAttributeIgnore: cudaLaunchAttributeID = 0;
pub const cudaLaunchAttributeAccessPolicyWindow: cudaLaunchAttributeID = 1;
pub const cudaLaunchAttributeCooperative: cudaLaunchAttributeID = 2;
pub const cudaLaunchAttributeSynchronizationPolicy: cudaLaunchAttributeID = 3;
pub const cudaLaunchAttributeClusterDimension: cudaLaunchAttributeID = 4;
pub const cudaLaunchAttributeClusterSchedulingPolicyPreference: cudaLaunchAttributeID = 5;
pub const cudaLaunchAttributeProgrammaticStreamSerialization: cudaLaunchAttributeID = 6;
pub const cudaLaunchAttributeProgrammaticEvent: cudaLaunchAttributeID = 7;
pub const cudaLaunchAttributePriority: cudaLaunchAttributeID = 8;
pub const cudaLaunchAttributeMemSyncDomainMap: cudaLaunchAttributeID = 9;
pub const cudaLaunchAttributeMemSyncDomain: cudaLaunchAttributeID = 10;
pub const cudaLaunchAttributeLaunchCompletionEvent: cudaLaunchAttributeID = 12;
pub const cudaLaunchAttributeDeviceUpdatableKernelNode: cudaLaunchAttributeID = 13;

/// Cluster scheduling policy.
pub type cudaClusterSchedulingPolicy = c_int;
pub const cudaClusterSchedulingPolicyDefault: cudaClusterSchedulingPolicy = 0;
pub const cudaClusterSchedulingPolicySpread: cudaClusterSchedulingPolicy = 1;
pub const cudaClusterSchedulingPolicyLoadBalancing: cudaClusterSchedulingPolicy = 2;

/// Memory sync domain.
pub type cudaLaunchMemSyncDomain = c_int;
pub const cudaLaunchMemSyncDomainDefault: cudaLaunchMemSyncDomain = 0;
pub const cudaLaunchMemSyncDomainRemote: cudaLaunchMemSyncDomain = 1;

/// Graph debug dot print flags.
pub type cudaGraphDebugDotFlags = c_uint;
pub const cudaGraphDebugDotFlagsVerbose: cudaGraphDebugDotFlags = 1 << 0;
pub const cudaGraphDebugDotFlagsKernelNodeParams: cudaGraphDebugDotFlags = 1 << 2;
pub const cudaGraphDebugDotFlagsMemcpyNodeParams: cudaGraphDebugDotFlags = 1 << 3;
pub const cudaGraphDebugDotFlagsMemsetNodeParams: cudaGraphDebugDotFlags = 1 << 4;
pub const cudaGraphDebugDotFlagsHostNodeParams: cudaGraphDebugDotFlags = 1 << 5;
pub const cudaGraphDebugDotFlagsEventNodeParams: cudaGraphDebugDotFlags = 1 << 6;
pub const cudaGraphDebugDotFlagsExtSemasSignalNodeParams: cudaGraphDebugDotFlags = 1 << 7;
pub const cudaGraphDebugDotFlagsExtSemasWaitNodeParams: cudaGraphDebugDotFlags = 1 << 8;
pub const cudaGraphDebugDotFlagsKernelNodeAttributes: cudaGraphDebugDotFlags = 1 << 9;
pub const cudaGraphDebugDotFlagsHandles: cudaGraphDebugDotFlags = 1 << 10;
pub const cudaGraphDebugDotFlagsConditionalNodeParams: cudaGraphDebugDotFlags = 1 << 15;

/// Graph instantiate flags.
pub type cudaGraphInstantiateFlags = c_uint;
pub const cudaGraphInstantiateFlagAutoFreeOnLaunch: cudaGraphInstantiateFlags = 1;
pub const cudaGraphInstantiateFlagUpload: cudaGraphInstantiateFlags = 2;
pub const cudaGraphInstantiateFlagDeviceLaunch: cudaGraphInstantiateFlags = 4;
pub const cudaGraphInstantiateFlagUseNodePriority: cudaGraphInstantiateFlags = 8;

/// Graph instantiate result.
pub type cudaGraphInstantiateResult = c_int;
pub const cudaGraphInstantiateSuccess: cudaGraphInstantiateResult = 0;
pub const cudaGraphInstantiateError: cudaGraphInstantiateResult = 1;
pub const cudaGraphInstantiateInvalidStructure: cudaGraphInstantiateResult = 2;
pub const cudaGraphInstantiateNodeOperationNotSupported: cudaGraphInstantiateResult = 3;
pub const cudaGraphInstantiateMultipleDevicesNotSupported: cudaGraphInstantiateResult = 4;

/// Graph dependency type.
pub type cudaGraphDependencyType = c_int;
pub const cudaGraphDependencyTypeDefault: cudaGraphDependencyType = 0;
pub const cudaGraphDependencyTypeProgrammatic: cudaGraphDependencyType = 1;

/// Graph conditional node type.
pub type cudaGraphConditionalNodeType = c_int;
pub const cudaGraphCondTypeIf: cudaGraphConditionalNodeType = 0;
pub const cudaGraphCondTypeWhile: cudaGraphConditionalNodeType = 1;

/// Driver entry point query result.
pub type cudaDriverEntryPointQueryResult = c_int;
pub const cudaDriverEntryPointSuccess: cudaDriverEntryPointQueryResult = 0;
pub const cudaDriverEntryPointSymbolNotFound: cudaDriverEntryPointQueryResult = 1;
pub const cudaDriverEntryPointVersionNotSufficent: cudaDriverEntryPointQueryResult = 2;

/// Async notification type.
pub type cudaAsyncNotificationType = c_int;
pub const cudaAsyncNotificationTypeOverBudget: cudaAsyncNotificationType = 1;

/// Jit option.
pub type cudaJitOption = c_int;
pub const cudaJitOptionMaxRegisters: cudaJitOption = 0;
pub const cudaJitOptionThreadsPerBlock: cudaJitOption = 1;
pub const cudaJitOptionWallTime: cudaJitOption = 2;
pub const cudaJitOptionInfoLogBuffer: cudaJitOption = 3;
pub const cudaJitOptionInfoLogBufferSizeBytes: cudaJitOption = 4;
pub const cudaJitOptionErrorLogBuffer: cudaJitOption = 5;
pub const cudaJitOptionErrorLogBufferSizeBytes: cudaJitOption = 6;
pub const cudaJitOptionOptimizationLevel: cudaJitOption = 7;
pub const cudaJitOptionTargetFromContext: cudaJitOption = 8;
pub const cudaJitOptionTarget: cudaJitOption = 9;
pub const cudaJitOptionFallbackStrategy: cudaJitOption = 10;
pub const cudaJitOptionGenerateDebugInfo: cudaJitOption = 11;
pub const cudaJitOptionLogVerbose: cudaJitOption = 12;
pub const cudaJitOptionGenerateLineInfo: cudaJitOption = 13;
pub const cudaJitOptionCacheMode: cudaJitOption = 14;
pub const cudaJitOptionFastCompile: cudaJitOption = 15;
pub const cudaJitOptionGlobalCFA: cudaJitOption = 16;
pub const cudaJitOptionLto: cudaJitOption = 18;
pub const cudaJitOptionFtz: cudaJitOption = 19;
pub const cudaJitOptionPrecDiv: cudaJitOption = 20;
pub const cudaJitOptionPrecSqrt: cudaJitOption = 21;
pub const cudaJitOptionFma: cudaJitOption = 22;

/// Library option.
pub type cudaLibraryOption = c_int;

/// GL device list.
#[cfg(feature = "opengl")]
pub type cudaGLDeviceList = c_int;
#[cfg(feature = "opengl")]
pub const cudaGLDeviceListAll: cudaGLDeviceList = 1;
#[cfg(feature = "opengl")]
pub const cudaGLDeviceListCurrentFrame: cudaGLDeviceList = 2;
#[cfg(feature = "opengl")]
pub const cudaGLDeviceListNextFrame: cudaGLDeviceList = 3;

/// Atomic operation.
pub type cudaAtomicOperation = c_int;

/// Atomic operation capability.
pub type cudaAtomicOperationCapability = c_uint;

/// Stream update capture dependencies flags.
pub type cudaStreamUpdateCaptureDependenciesFlags = c_uint;
pub const cudaStreamAddCaptureDependencies: cudaStreamUpdateCaptureDependenciesFlags = 0x0;
pub const cudaStreamSetCaptureDependencies: cudaStreamUpdateCaptureDependenciesFlags = 0x1;

/// Memory type.
pub type cudaMemoryType = c_int;
pub const cudaMemoryTypeUnregistered: cudaMemoryType = 0;
pub const cudaMemoryTypeHost: cudaMemoryType = 1;
pub const cudaMemoryTypeDevice: cudaMemoryType = 2;
pub const cudaMemoryTypeManaged: cudaMemoryType = 3;

/// Host task sync mode.
pub type cudaHostTaskSyncMode = c_uint;
pub const cudaHostTaskSyncModeDefault: cudaHostTaskSyncMode = 0;
pub const cudaHostTaskSyncModeAtLaunch: cudaHostTaskSyncMode = 1;

/// Graph kernel node field.
pub type cudaGraphKernelNodeField = c_int;
pub const cudaGraphKernelNodeFieldInvalid: cudaGraphKernelNodeField = 0;
pub const cudaGraphKernelNodeFieldGridDim: cudaGraphKernelNodeField = 1;
pub const cudaGraphKernelNodeFieldParam: cudaGraphKernelNodeField = 2;
pub const cudaGraphKernelNodeFieldEnabled: cudaGraphKernelNodeField = 3;

// ============================================================================
// Flag Constants
// ============================================================================

// Stream flags
pub const cudaStreamDefault: c_uint = 0x00;
pub const cudaStreamNonBlocking: c_uint = 0x01;

// Event flags
pub const cudaEventDefault: c_uint = 0x00;
pub const cudaEventBlockingSync: c_uint = 0x01;
pub const cudaEventDisableTiming: c_uint = 0x02;
pub const cudaEventInterprocess: c_uint = 0x04;

// Host alloc flags
pub const cudaHostAllocDefault: c_uint = 0x00;
pub const cudaHostAllocPortable: c_uint = 0x01;
pub const cudaHostAllocMapped: c_uint = 0x02;
pub const cudaHostAllocWriteCombined: c_uint = 0x04;

// Host register flags
pub const cudaHostRegisterDefault: c_uint = 0x00;
pub const cudaHostRegisterPortable: c_uint = 0x01;
pub const cudaHostRegisterMapped: c_uint = 0x02;
pub const cudaHostRegisterIoMemory: c_uint = 0x04;
pub const cudaHostRegisterReadOnly: c_uint = 0x08;

// Device flags
pub const cudaDeviceScheduleAuto: c_uint = 0x00;
pub const cudaDeviceScheduleSpin: c_uint = 0x01;
pub const cudaDeviceScheduleYield: c_uint = 0x02;
pub const cudaDeviceScheduleBlockingSync: c_uint = 0x04;
pub const cudaDeviceMapHost: c_uint = 0x08;
pub const cudaDeviceLmemResizeToMax: c_uint = 0x10;
pub const cudaDeviceSyncMemops: c_uint = 0x80;

// Array flags
pub const cudaArrayDefault: c_uint = 0x00;
pub const cudaArrayLayered: c_uint = 0x01;
pub const cudaArraySurfaceLoadStore: c_uint = 0x02;
pub const cudaArrayCubemap: c_uint = 0x04;
pub const cudaArrayTextureGather: c_uint = 0x08;
pub const cudaArrayColorAttachment: c_uint = 0x20;
pub const cudaArraySparse: c_uint = 0x40;
pub const cudaArrayDeferredMapping: c_uint = 0x80;

// IPC mem flags
pub const cudaIpcMemLazyEnablePeerAccess: c_uint = 0x01;

// Mem attach flags
pub const cudaMemAttachGlobal: c_uint = 0x01;
pub const cudaMemAttachHost: c_uint = 0x02;
pub const cudaMemAttachSingle: c_uint = 0x04;

// Cooperative launch multi-device flags
pub const cudaCooperativeLaunchMultiDeviceNoPreSync: c_uint = 0x01;
pub const cudaCooperativeLaunchMultiDeviceNoPostSync: c_uint = 0x02;

// User object flags
pub const cudaUserObjectNoDestructorSync: c_uint = 0x01;
pub const cudaGraphUserObjectMove: c_uint = 0x01;

// Driver entry point flags
pub const cudaEnableDefault: u64 = 0x0;
pub const cudaEnableLegacyStream: u64 = 0x1;
pub const cudaEnablePerThreadDefaultStream: u64 = 0x2;

// Event record/wait flags
pub const cudaEventRecordDefault: c_uint = 0x00;
pub const cudaEventRecordExternal: c_uint = 0x01;
pub const cudaEventWaitDefault: c_uint = 0x00;
pub const cudaEventWaitExternal: c_uint = 0x01;

// ============================================================================
// Structures
// ============================================================================

/// 3D dimensions.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct dim3 {
    pub x: c_uint,
    pub y: c_uint,
    pub z: c_uint,
}

/// Channel format descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaChannelFormatDesc {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub w: c_int,
    pub f: cudaChannelFormatKind,
}

/// 3D extent.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct cudaExtent {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

/// 3D position.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct cudaPos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

/// Pitched pointer.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaPitchedPtr {
    pub ptr: *mut c_void,
    pub pitch: usize,
    pub xsize: usize,
    pub ysize: usize,
}

/// Memory copy 3D parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemcpy3DParms {
    pub srcArray: cudaArray_t,
    pub srcPos: cudaPos,
    pub srcPtr: cudaPitchedPtr,
    pub dstArray: cudaArray_t,
    pub dstPos: cudaPos,
    pub dstPtr: cudaPitchedPtr,
    pub extent: cudaExtent,
    pub kind: cudaMemcpyKind,
}

/// Memory copy 3D peer parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemcpy3DPeerParms {
    pub srcArray: cudaArray_t,
    pub srcPos: cudaPos,
    pub srcPtr: cudaPitchedPtr,
    pub srcDevice: c_int,
    pub dstArray: cudaArray_t,
    pub dstPos: cudaPos,
    pub dstPtr: cudaPitchedPtr,
    pub dstDevice: c_int,
    pub extent: cudaExtent,
}

/// Memcpy 3D batch operation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemcpy3DBatchOp {
    pub src: cudaMemcpy3DOperand,
    pub dst: cudaMemcpy3DOperand,
    pub extent: cudaExtent,
}

/// Memcpy 3D operand type.
pub type cudaMemcpy3DOperandType = c_int;
pub const cudaMemcpy3DOperandTypePointer: cudaMemcpy3DOperandType = 0;
pub const cudaMemcpy3DOperandTypeArray: cudaMemcpy3DOperandType = 1;

/// Memcpy 3D operand.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaMemcpy3DOperand {
    pub operandType: cudaMemcpy3DOperandType,
    pub pointer: cudaMemcpy3DOperandPointer,
}

/// Memcpy 3D operand pointer info.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaMemcpy3DOperandPointer {
    pub ptr: *mut c_void,
    pub pitchInBytes: usize,
    pub device: c_int,
}

impl core::fmt::Debug for cudaMemcpy3DOperand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaMemcpy3DOperand")
            .field("operandType", &self.operandType)
            .finish()
    }
}

/// Memset parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemsetParams {
    pub dst: *mut c_void,
    pub pitch: usize,
    pub value: c_uint,
    pub elementSize: c_uint,
    pub width: usize,
    pub height: usize,
}

/// Memcpy attributes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemcpyAttributes {
    pub srcAccessOrder: c_int,
    pub dstAccessOrder: c_int,
    pub srcLocHint: cudaMemLocation,
    pub dstLocHint: cudaMemLocation,
}

/// Memory location.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemLocation {
    pub r#type: cudaMemLocationType,
    pub id: c_int,
}

/// Memory access descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemAccessDesc {
    pub location: cudaMemLocation,
    pub flags: cudaMemAccessFlags,
}

/// Memory pool properties.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemPoolProps {
    pub allocType: cudaMemAllocationType,
    pub handleTypes: cudaMemAllocationHandleType,
    pub location: cudaMemLocation,
    pub win32SecurityAttributes: *mut c_void,
    pub maxSize: usize,
    pub usage: u16,
    pub reserved: [u8; 54],
}

/// Memory pool pointer export data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemPoolPtrExportData {
    pub reserved: [u8; 64],
}

/// Pointer attributes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaPointerAttributes {
    pub r#type: cudaMemoryType,
    pub device: c_int,
    pub devicePointer: *mut c_void,
    pub hostPointer: *mut c_void,
}

/// Function attributes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaFuncAttributes {
    pub sharedSizeBytes: usize,
    pub constSizeBytes: usize,
    pub localSizeBytes: usize,
    pub maxThreadsPerBlock: c_int,
    pub numRegs: c_int,
    pub ptxVersion: c_int,
    pub binaryVersion: c_int,
    pub cacheModeCA: c_int,
    pub maxDynamicSharedSizeBytes: c_int,
    pub preferredShmemCarveout: c_int,
    pub clusterDimMustBeSet: c_int,
    pub requiredClusterWidth: c_int,
    pub requiredClusterHeight: c_int,
    pub requiredClusterDepth: c_int,
    pub nonPortableClusterSizeAllowed: c_int,
    pub reserved: [c_int; 16],
}

/// Device properties.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaDeviceProp {
    pub name: [c_char; 256],
    pub uuid: cudaUUID_t,
    pub luid: [c_char; 8],
    pub luidDeviceNodeMask: c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: c_int,
    pub warpSize: c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: c_int,
    pub maxThreadsDim: [c_int; 3],
    pub maxGridSize: [c_int; 3],
    pub clockRate: c_int,
    pub totalConstMem: usize,
    pub major: c_int,
    pub minor: c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: c_int,
    pub multiProcessorCount: c_int,
    pub kernelExecTimeoutEnabled: c_int,
    pub integrated: c_int,
    pub canMapHostMemory: c_int,
    pub computeMode: c_int,
    pub maxTexture1D: c_int,
    pub maxTexture1DMipmap: c_int,
    pub maxTexture1DLinear: c_int,
    pub maxTexture2D: [c_int; 2],
    pub maxTexture2DMipmap: [c_int; 2],
    pub maxTexture2DLinear: [c_int; 3],
    pub maxTexture2DGather: [c_int; 2],
    pub maxTexture3D: [c_int; 3],
    pub maxTexture3DAlt: [c_int; 3],
    pub maxTextureCubemap: c_int,
    pub maxTexture1DLayered: [c_int; 2],
    pub maxTexture2DLayered: [c_int; 3],
    pub maxTextureCubemapLayered: [c_int; 2],
    pub maxSurface1D: c_int,
    pub maxSurface2D: [c_int; 2],
    pub maxSurface3D: [c_int; 3],
    pub maxSurface1DLayered: [c_int; 2],
    pub maxSurface2DLayered: [c_int; 3],
    pub maxSurfaceCubemap: c_int,
    pub maxSurfaceCubemapLayered: [c_int; 2],
    pub surfaceAlignment: usize,
    pub concurrentKernels: c_int,
    pub ECCEnabled: c_int,
    pub pciBusID: c_int,
    pub pciDeviceID: c_int,
    pub pciDomainID: c_int,
    pub tccDriver: c_int,
    pub asyncEngineCount: c_int,
    pub unifiedAddressing: c_int,
    pub memoryClockRate: c_int,
    pub memoryBusWidth: c_int,
    pub l2CacheSize: c_int,
    pub persistingL2CacheMaxSize: c_int,
    pub maxThreadsPerMultiProcessor: c_int,
    pub streamPrioritiesSupported: c_int,
    pub globalL1CacheSupported: c_int,
    pub localL1CacheSupported: c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: c_int,
    pub managedMemory: c_int,
    pub isMultiGpuBoard: c_int,
    pub multiGpuBoardGroupID: c_int,
    pub hostNativeAtomicSupported: c_int,
    pub singleToDoublePrecisionPerfRatio: c_int,
    pub pageableMemoryAccess: c_int,
    pub concurrentManagedAccess: c_int,
    pub computePreemptionSupported: c_int,
    pub canUseHostPointerForRegisteredMem: c_int,
    pub cooperativeLaunch: c_int,
    pub cooperativeMultiDeviceLaunch: c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: c_int,
    pub directManagedMemAccessFromHost: c_int,
    pub maxBlocksPerMultiProcessor: c_int,
    pub accessPolicyMaxWindowSize: c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: c_int,
    pub sparsePropertiesSupported: c_int,
    pub hostRegisterReadOnlySupported: c_int,
    pub timelineSemaphoreInteropSupported: c_int,
    pub memoryPoolsSupported: c_int,
    pub gpuDirectRDMASupported: c_int,
    pub gpuDirectRDMAFlushWritesOptions: c_uint,
    pub gpuDirectRDMAWritesOrdering: c_int,
    pub memoryPoolSupportedHandleTypes: c_uint,
    pub deferredMappingCudaArraySupported: c_int,
    pub ipcEventSupported: c_int,
    pub clusterLaunch: c_int,
    pub unifiedFunctionPointers: c_int,
    pub reserved2: [c_int; 2],
    pub reserved: [c_int; 60],
}

impl core::fmt::Debug for cudaDeviceProp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaDeviceProp")
            .field("major", &self.major)
            .field("minor", &self.minor)
            .field("totalGlobalMem", &self.totalGlobalMem)
            .field("multiProcessorCount", &self.multiProcessorCount)
            .finish()
    }
}

/// Access policy window.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaAccessPolicyWindow {
    pub base_ptr: *mut c_void,
    pub num_bytes: usize,
    pub hitRatio: f32,
    pub hitProp: cudaAccessProperty,
    pub missProp: cudaAccessProperty,
}

/// Stream attribute value (union - represented as largest variant).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaStreamAttrValue {
    pub data: [u8; 64],
}

impl core::fmt::Debug for cudaStreamAttrValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaStreamAttrValue").finish()
    }
}

/// Kernel node attribute value (union).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaKernelNodeAttrValue {
    pub data: [u8; 64],
}

impl core::fmt::Debug for cudaKernelNodeAttrValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaKernelNodeAttrValue").finish()
    }
}

/// Launch attribute value (union).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaLaunchAttributeValue {
    pub data: [u8; 64],
}

impl core::fmt::Debug for cudaLaunchAttributeValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaLaunchAttributeValue").finish()
    }
}

/// Launch attribute.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaLaunchAttribute {
    pub id: cudaLaunchAttributeID,
    pub val: cudaLaunchAttributeValue,
}

/// Launch configuration.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaLaunchConfig_t {
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub dynamicSmemBytes: usize,
    pub stream: cudaStream_t,
    pub attrs: *mut cudaLaunchAttribute,
    pub numAttrs: c_uint,
}

/// Kernel node parameters.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaKernelNodeParams {
    pub func: *const c_void,
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub sharedMemBytes: c_uint,
    pub kernelParams: *mut *mut c_void,
    pub extra: *mut *mut c_void,
}

impl core::fmt::Debug for cudaKernelNodeParams {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaKernelNodeParams")
            .field("func", &self.func)
            .field("gridDim", &self.gridDim)
            .field("blockDim", &self.blockDim)
            .finish()
    }
}

/// Host node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaHostNodeParams {
    pub fn_: cudaHostFn_t,
    pub userData: *mut c_void,
}

/// Graph edge data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaGraphEdgeData {
    pub from_port: u8,
    pub to_port: u8,
    pub r#type: u8,
    pub reserved: [u8; 5],
}

/// Graph exec update result info.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaGraphExecUpdateResultInfo {
    pub result: cudaGraphExecUpdateResult,
    pub errorNode: cudaGraphNode_t,
    pub errorFromNode: cudaGraphNode_t,
}

/// Graph instantiate parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaGraphInstantiateParams {
    pub flags: u64,
    pub uploadStream: cudaStream_t,
    pub errNode_out: cudaGraphNode_t,
    pub result_out: cudaGraphInstantiateResult,
}

/// Kernel node update.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaGraphKernelNodeUpdate {
    pub node: cudaGraphDeviceNode_t,
    pub field: cudaGraphKernelNodeField,
    pub data: [u8; 32],
}

/// Graph node parameters (generic union-style).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaGraphNodeParams {
    pub r#type: cudaGraphNodeType,
    pub reserved0: [c_int; 3],
    pub data: [u8; 256],
    pub reserved2: u64,
}

/// Conditional node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaConditionalNodeParams {
    pub handle: cudaGraphConditionalHandle,
    pub r#type: cudaGraphConditionalNodeType,
    pub size: c_uint,
    pub phGraph_out: *mut cudaGraph_t,
    pub ctx: cudaExecutionContext_t,
}

/// Child graph node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaChildGraphNodeParams {
    pub graph: cudaGraph_t,
}

/// Event record node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaEventRecordNodeParams {
    pub event: cudaEvent_t,
}

/// Event wait node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaEventWaitNodeParams {
    pub event: cudaEvent_t,
}

/// Memory alloc node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemAllocNodeParams {
    pub poolProps: cudaMemPoolProps,
    pub accessDescs: *const cudaMemAccessDesc,
    pub accessDescCount: usize,
    pub bytesize: usize,
    pub dptr: *mut c_void,
}

/// Memory free node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemFreeNodeParams {
    pub dptr: *mut c_void,
}

/// Memcpy node parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaMemcpyNodeParams {
    pub src: *const c_void,
    pub dst: *mut c_void,
    pub count: usize,
    pub kind: cudaMemcpyKind,
}

/// External semaphore signal node parameters.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreSignalNodeParams {
    pub extSemArray: *const cudaExternalSemaphore_t,
    pub paramsArray: *const cudaExternalSemaphoreSignalParams,
    pub numExtSems: c_uint,
}

/// External semaphore wait node parameters.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreWaitNodeParams {
    pub extSemArray: *const cudaExternalSemaphore_t,
    pub paramsArray: *const cudaExternalSemaphoreWaitParams,
    pub numExtSems: c_uint,
}

/// External semaphore signal parameters.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreSignalParams {
    pub params: cudaExternalSemaphoreSignalParamsData,
    pub flags: c_uint,
    pub reserved: [c_uint; 16],
}

/// External semaphore signal parameters data.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreSignalParamsData {
    pub fence: cudaExternalSemaphoreFenceParams,
}

/// External semaphore wait parameters.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreWaitParams {
    pub params: cudaExternalSemaphoreWaitParamsData,
    pub flags: c_uint,
    pub reserved: [c_uint; 16],
}

/// External semaphore wait parameters data.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreWaitParamsData {
    pub fence: cudaExternalSemaphoreFenceParams,
}

/// External semaphore fence parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaExternalSemaphoreFenceParams {
    pub value: u64,
}

/// External memory handle descriptor.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalMemoryHandleDesc {
    pub r#type: cudaExternalMemoryHandleType,
    pub handle: cudaExternalMemoryHandleUnion,
    pub size: u64,
    pub flags: c_uint,
    pub reserved: [c_uint; 16],
}

/// External memory handle union (opaque).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalMemoryHandleUnion {
    pub data: [u8; 16],
}

/// External memory buffer descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaExternalMemoryBufferDesc {
    pub offset: u64,
    pub size: u64,
    pub flags: c_uint,
    pub reserved: [c_uint; 16],
}

/// External memory mipmapped array descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaExternalMemoryMipmappedArrayDesc {
    pub offset: u64,
    pub formatDesc: cudaChannelFormatDesc,
    pub extent: cudaExtent,
    pub flags: c_uint,
    pub numLevels: c_uint,
}

/// External semaphore handle descriptor.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaExternalSemaphoreHandleDesc {
    pub r#type: cudaExternalSemaphoreHandleType,
    pub handle: cudaExternalMemoryHandleUnion,
    pub flags: c_uint,
    pub reserved: [c_uint; 16],
}

/// Texture descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaTextureDesc {
    pub addressMode: [cudaTextureAddressMode; 3],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: c_int,
    pub borderColor: [f32; 4],
    pub normalizedCoords: c_int,
    pub maxAnisotropy: c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: c_int,
    pub seamlessCubemap: c_int,
}

/// Resource descriptor.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaResourceDesc {
    pub resType: cudaResourceType,
    pub res: cudaResourceDescRes,
}

/// Resource descriptor res union (opaque, 48 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cudaResourceDescRes {
    pub data: [u8; 48],
}

impl core::fmt::Debug for cudaResourceDesc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("cudaResourceDesc")
            .field("resType", &self.resType)
            .finish()
    }
}

/// Resource view descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaResourceViewDesc {
    pub format: cudaResourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: c_uint,
    pub lastMipmapLevel: c_uint,
    pub firstLayer: c_uint,
    pub lastLayer: c_uint,
}

/// Array memory requirements.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaArrayMemoryRequirements {
    pub size: usize,
    pub alignment: usize,
    pub reserved: [c_uint; 4],
}

/// Array sparse properties.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaArraySparseProperties {
    pub tileExtent: cudaArraySparsePropertiesTileExtent,
    pub miptailFirstLevel: c_uint,
    pub miptailSize: u64,
    pub flags: c_uint,
    pub reserved: [c_uint; 4],
}

/// Array sparse properties tile extent.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaArraySparsePropertiesTileExtent {
    pub width: c_uint,
    pub height: c_uint,
    pub depth: c_uint,
}

/// Async notification info.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaAsyncNotificationInfo_t {
    pub r#type: cudaAsyncNotificationType,
    pub data: [u8; 8],
}

/// Memory sync domain map.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cudaLaunchMemSyncDomainMap {
    pub default_: u8,
    pub remote: u8,
}

// ============================================================================
// Helper constructors (matching CUDA runtime inline functions)
// ============================================================================

/// Create a cudaExtent value.
pub const fn make_cudaExtent(w: usize, h: usize, d: usize) -> cudaExtent {
    cudaExtent {
        width: w,
        height: h,
        depth: d,
    }
}

/// Create a cudaPos value.
pub const fn make_cudaPos(x: usize, y: usize, z: usize) -> cudaPos {
    cudaPos { x, y, z }
}

/// Create a cudaPitchedPtr value.
pub const fn make_cudaPitchedPtr(d: *mut c_void, p: usize, xsz: usize, ysz: usize) -> cudaPitchedPtr {
    cudaPitchedPtr {
        ptr: d,
        pitch: p,
        xsize: xsz,
        ysize: ysz,
    }
}

/// Create a dim3 value.
pub const fn make_dim3(x: c_uint, y: c_uint, z: c_uint) -> dim3 {
    dim3 { x, y, z }
}
