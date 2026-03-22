/**
 * Compile with: nvcc -o verify_sizes tests/verify_sizes.c
 * or: gcc -I$CUDA_PATH/include -o verify_sizes tests/verify_sizes.c
 *
 * Run to print sizeof/offsetof for critical structs.
 * Compare output against Rust type sizes to verify ABI compatibility.
 */

#include <stdio.h>
#include <stddef.h>
#include <cuda_runtime.h>

#define PRINT_SIZE(T) printf("sizeof(%s) = %zu\n", #T, sizeof(T))
#define PRINT_ALIGN(T) printf("alignof(%s) = %zu\n", #T, _Alignof(T))
#define PRINT_OFFSET(T, F) printf("  offsetof(%s, %s) = %zu\n", #T, #F, offsetof(T, F))

int main(void) {
    /* Core types */
    PRINT_SIZE(cudaDeviceProp);
    PRINT_ALIGN(cudaDeviceProp);
    PRINT_OFFSET(cudaDeviceProp, name);
    PRINT_OFFSET(cudaDeviceProp, uuid);
    PRINT_OFFSET(cudaDeviceProp, totalGlobalMem);
    PRINT_OFFSET(cudaDeviceProp, major);
    PRINT_OFFSET(cudaDeviceProp, minor);
    PRINT_OFFSET(cudaDeviceProp, reservedSharedMemPerBlock);
    printf("\n");

    PRINT_SIZE(dim3);
    PRINT_SIZE(cudaExtent);
    PRINT_SIZE(cudaPos);
    PRINT_SIZE(cudaPitchedPtr);
    PRINT_SIZE(cudaMemcpy3DParms);
    PRINT_SIZE(cudaMemcpy3DPeerParms);
    PRINT_SIZE(cudaMemsetParams);
    printf("\n");

    /* Union types */
    PRINT_SIZE(cudaLaunchAttributeValue);
    PRINT_SIZE(cudaStreamAttrValue);
    PRINT_SIZE(cudaKernelNodeAttrValue);
    printf("\n");

    /* Graph types */
    PRINT_SIZE(cudaGraphNodeParams);
    PRINT_SIZE(cudaGraphEdgeData);
    PRINT_SIZE(cudaKernelNodeParams);
    PRINT_SIZE(cudaHostNodeParams);
    PRINT_SIZE(cudaGraphInstantiateParams);
    printf("\n");

    /* Resource types */
    PRINT_SIZE(cudaResourceDesc);
    PRINT_SIZE(cudaTextureDesc);
    PRINT_SIZE(cudaResourceViewDesc);
    printf("\n");

    /* External memory types */
    PRINT_SIZE(cudaExternalMemoryHandleDesc);
    PRINT_SIZE(cudaExternalMemoryBufferDesc);
    PRINT_SIZE(cudaExternalSemaphoreHandleDesc);
    PRINT_SIZE(cudaExternalSemaphoreSignalParams);
    PRINT_SIZE(cudaExternalSemaphoreWaitParams);
    printf("\n");

    /* Memory management types */
    PRINT_SIZE(cudaMemPoolProps);
    PRINT_SIZE(cudaMemPoolPtrExportData);
    PRINT_SIZE(cudaMemLocation);
    PRINT_SIZE(cudaMemAccessDesc);
    PRINT_SIZE(cudaPointerAttributes);
    PRINT_SIZE(cudaFuncAttributes);
    PRINT_SIZE(cudaChannelFormatDesc);
    PRINT_SIZE(cudaAccessPolicyWindow);
    printf("\n");

    /* Handle/opaque sizes */
    PRINT_SIZE(cudaIpcEventHandle_t);
    PRINT_SIZE(cudaIpcMemHandle_t);
    PRINT_SIZE(cudaLaunchConfig_t);
    PRINT_SIZE(cudaLaunchAttribute);

    printf("\nAll sizes printed. Compare against Rust output.\n");
    return 0;
}
