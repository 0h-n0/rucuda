//! Graph Management functions.

use crate::types::*;
use std::os::raw::{c_char, c_int, c_uint, c_void};

unsafe extern "C" {
    // ========================================================================
    // Graph Device Memory Management
    // ========================================================================

    /// Query a device graph memory attribute.
    pub fn cudaDeviceGetGraphMemAttribute(
        device: c_int,
        attr: cudaGraphMemAttributeType,
        value: *mut c_void,
    ) -> cudaError_t;

    /// Set a device graph memory attribute.
    pub fn cudaDeviceSetGraphMemAttribute(
        device: c_int,
        attr: cudaGraphMemAttributeType,
        value: *mut c_void,
    ) -> cudaError_t;

    /// Free unused memory on a device used for graph back to the OS.
    pub fn cudaDeviceGraphMemTrim(device: c_int) -> cudaError_t;

    // ========================================================================
    // Graph Creation & Destruction
    // ========================================================================

    /// Creates a graph.
    pub fn cudaGraphCreate(pGraph: *mut cudaGraph_t, flags: c_uint) -> cudaError_t;

    /// Destroys a graph.
    pub fn cudaGraphDestroy(graph: cudaGraph_t) -> cudaError_t;

    /// Clones a graph.
    pub fn cudaGraphClone(pGraphClone: *mut cudaGraph_t, originalGraph: cudaGraph_t) -> cudaError_t;

    // ========================================================================
    // Node Creation
    // ========================================================================

    /// Creates a child graph node and adds it to a graph.
    pub fn cudaGraphAddChildGraphNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        childGraph: cudaGraph_t,
    ) -> cudaError_t;

    /// Creates an empty node and adds it to a graph.
    pub fn cudaGraphAddEmptyNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t;

    /// Creates an event record node and adds it to a graph.
    pub fn cudaGraphAddEventRecordNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        event: cudaEvent_t,
    ) -> cudaError_t;

    /// Creates an event wait node and adds it to a graph.
    pub fn cudaGraphAddEventWaitNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        event: cudaEvent_t,
    ) -> cudaError_t;

    /// Creates an external semaphore signal node and adds it to a graph.
    pub fn cudaGraphAddExternalSemaphoresSignalNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;

    /// Creates an external semaphore wait node and adds it to a graph.
    pub fn cudaGraphAddExternalSemaphoresWaitNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;

    /// Creates a host execution node and adds it to a graph.
    pub fn cudaGraphAddHostNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t;

    /// Creates a kernel execution node and adds it to a graph.
    pub fn cudaGraphAddKernelNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t;

    /// Creates a memory allocation node and adds it to a graph.
    pub fn cudaGraphAddMemAllocNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *mut cudaMemAllocNodeParams,
    ) -> cudaError_t;

    /// Creates a memory free node and adds it to a graph.
    pub fn cudaGraphAddMemFreeNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dptr: *mut c_void,
    ) -> cudaError_t;

    /// Creates a memcpy node and adds it to a graph.
    pub fn cudaGraphAddMemcpyNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pCopyParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t;

    /// Creates a 1D memcpy node and adds it to a graph.
    pub fn cudaGraphAddMemcpyNode1D(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dst: *mut c_void,
        src: *const c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Creates a memcpy node to copy from a symbol on the device.
    pub fn cudaGraphAddMemcpyNodeFromSymbol(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dst: *mut c_void,
        symbol: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Creates a memcpy node to copy to a symbol on the device.
    pub fn cudaGraphAddMemcpyNodeToSymbol(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        symbol: *const c_void,
        src: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Creates a memset node and adds it to a graph.
    pub fn cudaGraphAddMemsetNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pMemsetParams: *const cudaMemsetParams,
    ) -> cudaError_t;

    /// Adds a node of arbitrary type to a graph.
    pub fn cudaGraphAddNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;

    // ========================================================================
    // Dependency Management
    // ========================================================================

    /// Adds dependency edges to a graph.
    pub fn cudaGraphAddDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t;

    /// Removes dependency edges from a graph.
    pub fn cudaGraphRemoveDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t;

    // ========================================================================
    // Graph Query
    // ========================================================================

    /// Returns a graph's nodes.
    pub fn cudaGraphGetNodes(
        graph: cudaGraph_t,
        nodes: *mut cudaGraphNode_t,
        numNodes: *mut usize,
    ) -> cudaError_t;

    /// Returns a graph's root nodes.
    pub fn cudaGraphGetRootNodes(
        graph: cudaGraph_t,
        pRootNodes: *mut cudaGraphNode_t,
        pNumRootNodes: *mut usize,
    ) -> cudaError_t;

    /// Returns a graph's dependency edges.
    pub fn cudaGraphGetEdges(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        numEdges: *mut usize,
    ) -> cudaError_t;

    /// Gets the ID of a graph.
    pub fn cudaGraphGetId(hGraph: cudaGraph_t, graphID: *mut c_uint) -> cudaError_t;

    // ========================================================================
    // Node Query & Manipulation
    // ========================================================================

    /// Gets a handle to the embedded graph of a child graph node.
    pub fn cudaGraphChildGraphNodeGetGraph(
        node: cudaGraphNode_t,
        pGraph: *mut cudaGraph_t,
    ) -> cudaError_t;

    /// Remove a node from the graph.
    pub fn cudaGraphDestroyNode(node: cudaGraphNode_t) -> cudaError_t;

    /// Returns a node's type.
    pub fn cudaGraphNodeGetType(
        node: cudaGraphNode_t,
        pType: *mut cudaGraphNodeType,
    ) -> cudaError_t;

    /// Returns the graph which contains a node.
    pub fn cudaGraphNodeGetContainingGraph(
        hNode: cudaGraphNode_t,
        phGraph: *mut cudaGraph_t,
    ) -> cudaError_t;

    /// Returns the local ID of a node within the containing graph.
    pub fn cudaGraphNodeGetLocalId(hNode: cudaGraphNode_t, nodeId: *mut c_uint) -> cudaError_t;

    /// Returns the tools ID of a graph node.
    pub fn cudaGraphNodeGetToolsId(
        hNode: cudaGraphNode_t,
        toolsNodeId: *mut u64,
    ) -> cudaError_t;

    /// Returns a node's parameters.
    pub fn cudaGraphNodeGetParams(
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;

    /// Sets a node's parameters.
    pub fn cudaGraphNodeSetParams(
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;

    /// Finds a cloned version of a node.
    pub fn cudaGraphNodeFindInClone(
        pNode: *mut cudaGraphNode_t,
        originalNode: cudaGraphNode_t,
        clonedGraph: cudaGraph_t,
    ) -> cudaError_t;

    // ========================================================================
    // Node Dependencies
    // ========================================================================

    /// Returns a node's dependencies.
    pub fn cudaGraphNodeGetDependencies(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependencies: *mut usize,
    ) -> cudaError_t;

    /// Returns a node's dependent nodes.
    pub fn cudaGraphNodeGetDependentNodes(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t;

    // ========================================================================
    // Event Node Functions
    // ========================================================================

    /// Returns the event associated with an event record node.
    pub fn cudaGraphEventRecordNodeGetEvent(
        node: cudaGraphNode_t,
        event_out: *mut cudaEvent_t,
    ) -> cudaError_t;

    /// Sets an event record node's event.
    pub fn cudaGraphEventRecordNodeSetEvent(
        node: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;

    /// Returns the event associated with an event wait node.
    pub fn cudaGraphEventWaitNodeGetEvent(
        node: cudaGraphNode_t,
        event_out: *mut cudaEvent_t,
    ) -> cudaError_t;

    /// Sets an event wait node's event.
    pub fn cudaGraphEventWaitNodeSetEvent(
        node: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;

    // ========================================================================
    // Host Node Functions
    // ========================================================================

    /// Returns a host node's parameters.
    pub fn cudaGraphHostNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaHostNodeParams,
    ) -> cudaError_t;

    /// Sets a host node's parameters.
    pub fn cudaGraphHostNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t;

    // ========================================================================
    // Kernel Node Functions
    // ========================================================================

    /// Returns a kernel node's parameters.
    pub fn cudaGraphKernelNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaKernelNodeParams,
    ) -> cudaError_t;

    /// Sets a kernel node's parameters.
    pub fn cudaGraphKernelNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t;

    /// Returns a kernel node's attribute.
    pub fn cudaGraphKernelNodeGetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaKernelNodeAttrID,
        value_out: *mut cudaKernelNodeAttrValue,
    ) -> cudaError_t;

    /// Sets a kernel node's attribute.
    pub fn cudaGraphKernelNodeSetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaKernelNodeAttrID,
        value: *const cudaKernelNodeAttrValue,
    ) -> cudaError_t;

    /// Copies attributes from one kernel node to another.
    pub fn cudaGraphKernelNodeCopyAttributes(
        hDst: cudaGraphNode_t,
        hSrc: cudaGraphNode_t,
    ) -> cudaError_t;

    /// Applies kernel node updates.
    pub fn cudaGraphKernelNodeUpdatesApply(
        updates: *const cudaGraphKernelNodeUpdate,
        updateCount: usize,
    ) -> cudaError_t;

    // ========================================================================
    // Memory Node Functions
    // ========================================================================

    /// Returns a memory allocation node's parameters.
    pub fn cudaGraphMemAllocNodeGetParams(
        node: cudaGraphNode_t,
        params_out: *mut cudaMemAllocNodeParams,
    ) -> cudaError_t;

    /// Returns a memory free node's parameters.
    pub fn cudaGraphMemFreeNodeGetParams(
        node: cudaGraphNode_t,
        dptr_out: *mut *mut c_void,
    ) -> cudaError_t;

    /// Returns a memcpy node's parameters.
    pub fn cudaGraphMemcpyNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaMemcpy3DParms,
    ) -> cudaError_t;

    /// Sets a memcpy node's parameters.
    pub fn cudaGraphMemcpyNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t;

    /// Sets a memcpy node's parameters to perform a 1D copy.
    pub fn cudaGraphMemcpyNodeSetParams1D(
        node: cudaGraphNode_t,
        dst: *mut c_void,
        src: *const c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Sets a memcpy node's parameters to copy from a symbol.
    pub fn cudaGraphMemcpyNodeSetParamsFromSymbol(
        node: cudaGraphNode_t,
        dst: *mut c_void,
        symbol: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Sets a memcpy node's parameters to copy to a symbol.
    pub fn cudaGraphMemcpyNodeSetParamsToSymbol(
        node: cudaGraphNode_t,
        symbol: *const c_void,
        src: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Returns a memset node's parameters.
    pub fn cudaGraphMemsetNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaMemsetParams,
    ) -> cudaError_t;

    /// Sets a memset node's parameters.
    pub fn cudaGraphMemsetNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemsetParams,
    ) -> cudaError_t;

    // ========================================================================
    // External Semaphore Node Functions
    // ========================================================================

    /// Returns an external semaphore signal node's parameters.
    pub fn cudaGraphExternalSemaphoresSignalNodeGetParams(
        hNode: cudaGraphNode_t,
        params_out: *mut cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;

    /// Sets an external semaphore signal node's parameters.
    pub fn cudaGraphExternalSemaphoresSignalNodeSetParams(
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;

    /// Returns an external semaphore wait node's parameters.
    pub fn cudaGraphExternalSemaphoresWaitNodeGetParams(
        hNode: cudaGraphNode_t,
        params_out: *mut cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;

    /// Sets an external semaphore wait node's parameters.
    pub fn cudaGraphExternalSemaphoresWaitNodeSetParams(
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;

    // ========================================================================
    // Graph Instantiation
    // ========================================================================

    /// Creates an executable graph from a graph.
    pub fn cudaGraphInstantiate(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        flags: u64,
    ) -> cudaError_t;

    /// Creates an executable graph from a graph with flags.
    pub fn cudaGraphInstantiateWithFlags(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        flags: u64,
    ) -> cudaError_t;

    /// Creates an executable graph from a graph with params.
    pub fn cudaGraphInstantiateWithParams(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        instantiateParams: *mut cudaGraphInstantiateParams,
    ) -> cudaError_t;

    // ========================================================================
    // Executable Graph Management
    // ========================================================================

    /// Destroys an executable graph.
    pub fn cudaGraphExecDestroy(graphExec: cudaGraphExec_t) -> cudaError_t;

    /// Gets the ID of an executable graph.
    pub fn cudaGraphExecGetId(
        hGraphExec: cudaGraphExec_t,
        graphID: *mut c_uint,
    ) -> cudaError_t;

    /// Gets the flags of an executable graph.
    pub fn cudaGraphExecGetFlags(graphExec: cudaGraphExec_t, flags: *mut u64) -> cudaError_t;

    // ========================================================================
    // Executable Graph Node Updates
    // ========================================================================

    /// Sets node parameters in an executable graph.
    pub fn cudaGraphExecNodeSetParams(
        graphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;

    /// Updates a child graph node in an executable graph.
    pub fn cudaGraphExecChildGraphNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        childGraph: cudaGraph_t,
    ) -> cudaError_t;

    /// Sets host node parameters in an executable graph.
    pub fn cudaGraphExecHostNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t;

    /// Sets kernel node parameters in an executable graph.
    pub fn cudaGraphExecKernelNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t;

    /// Sets memcpy node parameters in an executable graph.
    pub fn cudaGraphExecMemcpyNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t;

    /// Sets 1D memcpy node parameters in an executable graph.
    pub fn cudaGraphExecMemcpyNodeSetParams1D(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        dst: *mut c_void,
        src: *const c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Sets memcpy-from-symbol node parameters in an executable graph.
    pub fn cudaGraphExecMemcpyNodeSetParamsFromSymbol(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        dst: *mut c_void,
        symbol: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Sets memcpy-to-symbol node parameters in an executable graph.
    pub fn cudaGraphExecMemcpyNodeSetParamsToSymbol(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        symbol: *const c_void,
        src: *const c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;

    /// Sets memset node parameters in an executable graph.
    pub fn cudaGraphExecMemsetNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemsetParams,
    ) -> cudaError_t;

    /// Sets event record node event in an executable graph.
    pub fn cudaGraphExecEventRecordNodeSetEvent(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;

    /// Sets event wait node event in an executable graph.
    pub fn cudaGraphExecEventWaitNodeSetEvent(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;

    /// Sets external semaphore signal node params in an executable graph.
    pub fn cudaGraphExecExternalSemaphoresSignalNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;

    /// Sets external semaphore wait node params in an executable graph.
    pub fn cudaGraphExecExternalSemaphoresWaitNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;

    // ========================================================================
    // Executable Graph Execution
    // ========================================================================

    /// Launches an executable graph in a stream.
    pub fn cudaGraphLaunch(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t;

    /// Uploads an executable graph in a stream.
    pub fn cudaGraphUpload(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t;

    /// Check whether an executable graph can be updated with a graph.
    pub fn cudaGraphExecUpdate(
        hGraphExec: cudaGraphExec_t,
        hGraph: cudaGraph_t,
        resultInfo: *mut cudaGraphExecUpdateResultInfo,
    ) -> cudaError_t;

    // ========================================================================
    // Node Enabled State
    // ========================================================================

    /// Query whether a node in an executable graph is enabled.
    pub fn cudaGraphNodeGetEnabled(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        isEnabled: *mut c_uint,
    ) -> cudaError_t;

    /// Enables or disables a node in an executable graph.
    pub fn cudaGraphNodeSetEnabled(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        isEnabled: c_uint,
    ) -> cudaError_t;

    // ========================================================================
    // Conditional Graphs
    // ========================================================================

    /// Create a conditional handle.
    pub fn cudaGraphConditionalHandleCreate(
        pHandle_out: *mut cudaGraphConditionalHandle,
        graph: cudaGraph_t,
        defaultLaunchValue: c_uint,
        flags: c_uint,
    ) -> cudaError_t;

    // ========================================================================
    // User Object Functions
    // ========================================================================

    /// Create a user object.
    pub fn cudaUserObjectCreate(
        object_out: *mut cudaUserObject_t,
        ptr: *mut c_void,
        destroy: cudaHostFn_t,
        initialRefcount: c_uint,
        flags: c_uint,
    ) -> cudaError_t;

    /// Retain a reference to a user object.
    pub fn cudaUserObjectRetain(object: cudaUserObject_t, count: c_uint) -> cudaError_t;

    /// Release a reference to a user object.
    pub fn cudaUserObjectRelease(object: cudaUserObject_t, count: c_uint) -> cudaError_t;

    /// Retain a reference to a user object from a graph.
    pub fn cudaGraphRetainUserObject(
        graph: cudaGraph_t,
        object: cudaUserObject_t,
        count: c_uint,
        flags: c_uint,
    ) -> cudaError_t;

    /// Release a reference to a user object from a graph.
    pub fn cudaGraphReleaseUserObject(
        graph: cudaGraph_t,
        object: cudaUserObject_t,
        count: c_uint,
    ) -> cudaError_t;

    // ========================================================================
    // Debugging
    // ========================================================================

    /// Write a DOT file describing graph structure.
    pub fn cudaGraphDebugDotPrint(
        graph: cudaGraph_t,
        path: *const c_char,
        flags: c_uint,
    ) -> cudaError_t;
}
