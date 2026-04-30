/**************************************************************************/
/*                                                                        */
/*       Copyright (c) Microsoft Corporation. All rights reserved.        */
/*                                                                        */
/*       This software is licensed under the Microsoft Software License   */
/*       Terms for Microsoft Azure RTOS. Full text of the license can be  */
/*       found in the LICENSE file at https://aka.ms/AzureRTOS_EULA       */
/*       and in the root directory of this software.                      */
/*                                                                        */
/**************************************************************************/


/**************************************************************************/
/**************************************************************************/
/**                                                                       */
/** USBX Component                                                        */
/**                                                                       */
/**   Utility                                                             */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/


/* Include necessary system files.  */

#define UX_SOURCE_CODE

#include "ux_api.h"

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_utility_memory_allocate                         PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function allocates a block of memory for the specified size    */
/*    and alignment.                                                      */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    memory_alignment                      Memory alignment required     */
/*    memory_cache_flag                     Memory pool source            */
/*    memory_size_requested                 Number of bytes required      */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Pointer to block of memory                                          */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_memory_free_block_best_get Get best fit block of memory */
/*    _ux_utility_memory_set                 Set block of memory          */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Components                                                     */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  05-19-2020     Chaoqiong Xiao           Initial Version 6.0           */
/*  09-30-2020     Chaoqiong Xiao           Modified comment(s),          */
/*                                            verified memset and memcpy  */
/*                                            cases,                      */
/*                                            resulting in version 6.1    */
/*  01-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.1.10 */
/*  04-25-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            internal clean up,          */
/*                                            resulting in version 6.1.11 */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            refined memory management,  */
/*                                            fixed issue in 64-bit env,  */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
VOID  *_ux_utility_memory_allocate(ULONG memory_alignment, ULONG memory_cache_flag,
                                   ULONG memory_size_requested)
{
UX_MEMORY_BYTE_POOL *pool_ptr;
UCHAR               *current_ptr;
UCHAR               *work_ptr;
UCHAR               *next_ptr;
ALIGN_TYPE          *free_ptr;
UCHAR               **this_block_link_ptr;
UCHAR               **next_block_link_ptr;
ULONG               available_bytes;

ALIGN_TYPE          int_memory_buffer;
#ifdef UX_ENABLE_MEMORY_STATISTICS
UINT                index;
#endif

    /* Get the pool ptr */
    if (memory_cache_flag == UX_REGULAR_MEMORY)
    {
        pool_ptr = _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR];
    }
    else if (memory_cache_flag == UX_CACHE_SAFE_MEMORY)
    {
        pool_ptr = _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE];
    }
    else
    {
        return(UX_NULL);
    }

    /* Check if pool_ptr is NX_NULL */
    if (pool_ptr == UX_NULL)
    {
        return(UX_NULL);
    }

    /* Check if the memory size requested is 0.  */
    if (memory_size_requested == 0)
    {
        return(UX_NULL);
    }

    /* Get the mutex as this is a critical section.  */
    _ux_system_mutex_on(&_ux_system -> ux_system_mutex);

#ifdef UX_ENFORCE_SAFE_ALIGNMENT

    /* Check if safe alignment requested, in this case switch to UX_NO_ALIGN.  */
    if (memory_alignment == UX_SAFE_ALIGN)
    {

        /* We will use the memory_size_requested for the alignment.
           But we check to see if we have a minimum or maximum alignment.  */
        if (memory_size_requested < UX_ALIGN_MIN)

            /* No need to bother about alignment for small packets sizes.  */
            memory_alignment = UX_NO_ALIGN;

        /* Check if we are over the maximum.  */
        else if (memory_size_requested > UX_MAX_SCATTER_GATHER_ALIGNMENT)

            /* We are over the max alignment required. Use the maximum instead.  */
            memory_alignment = UX_MAX_SCATTER_GATHER_ALIGNMENT - 1;

        /* We are not over the maximum, so approximate the alignment according to the size of the memory.
            Check range for alignment on 4096 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_2048 + 1)
            memory_alignment = UX_ALIGN_4096;

        /* Check range for alignment on 2048 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_1024 + 1)
            memory_alignment = UX_ALIGN_2048;

        /* Check range for alignment on 1024 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_512 + 1)
            memory_alignment = UX_ALIGN_1024;

        /* Check range for alignment on 512 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_256 + 1)
            memory_alignment = UX_ALIGN_512;

        /* Check range for alignment on 256 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_128 + 1)
            memory_alignment = UX_ALIGN_256;

        /* Check range for alignment on 128 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_64 + 1)
            memory_alignment = UX_ALIGN_128;

        /* Check range for alignment on 64 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_32 + 1)
            memory_alignment = UX_ALIGN_64;

        /* Check range for alignment on 32 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_16 + 1)
            memory_alignment = UX_ALIGN_32;

        /* Check range for alignment on 16 bytes.  */
        else if (memory_size_requested >= UX_ALIGN_8 + 1)
            memory_alignment = UX_ALIGN_16;

        else
            memory_alignment = UX_ALIGN_MIN;
    }

#else

    /* Check if safe alignment requested, in this case switch to UX_NO_ALIGN.  */
    if (memory_alignment == UX_SAFE_ALIGN)
        memory_alignment = UX_NO_ALIGN;

#endif

    /* Ensure the alignment meats the minimum.  */
    if (memory_alignment < UX_ALIGN_MIN)
        memory_alignment =  UX_ALIGN_MIN;

    /* We need to make sure that the next memory block buffer is 8-byte aligned too. We
       do this by first adjusting the requested memory to be 8-byte aligned. One problem
       now is that the memory block might not be a size that is a multiple of 8, so we need
       to add the amount of memory required such that the memory buffer after the block has
       the correct alignment. For example, if the memory block has a size of 12, then we need
       to make sure it is placed on an 8-byte alignment that is after a 8-byte alignment so
       that the memory right after the memory block is 8-byte aligned (16).  */
    memory_size_requested =  (memory_size_requested + UX_ALIGN_MIN) & (~(ULONG)UX_ALIGN_MIN);
    memory_size_requested += (((ULONG)(UX_MEMORY_BLOCK_HEADER_SIZE + UX_ALIGN_MIN) & (~(ULONG)UX_ALIGN_MIN)) - (ULONG)UX_MEMORY_BLOCK_HEADER_SIZE);

    if (memory_alignment <= UX_ALIGN_MIN)
        current_ptr = _ux_utility_memory_byte_pool_search(pool_ptr, memory_size_requested);
    else
        current_ptr = _ux_utility_memory_byte_pool_search(pool_ptr, memory_size_requested + memory_alignment);

    /* Check if we found a memory block.  */
    if (current_ptr == UX_NULL)
    {

        /* We could not find a memory block.  */
        _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

        UX_TRACE_IN_LINE_INSERT(UX_TRACE_ERROR, UX_MEMORY_INSUFFICIENT, memory_size_requested, 0, 0, UX_TRACE_ERRORS, 0, 0)

        /* Error trap. */
        _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD, UX_SYSTEM_CONTEXT_UTILITY, UX_MEMORY_INSUFFICIENT);

        return(UX_NULL);
    }

    /* Pickup the next block's pointer.  */
    this_block_link_ptr =  UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(current_ptr);
    next_ptr =             *this_block_link_ptr;

    /* Calculate the number of bytes available in this block.  */
    available_bytes =   UX_UCHAR_POINTER_DIF(next_ptr, current_ptr);
    available_bytes =   available_bytes - UX_MEMORY_BLOCK_HEADER_SIZE;

    /* Get the memory buffer for this block.  */
    int_memory_buffer = (ALIGN_TYPE) (UX_UCHAR_POINTER_ADD(current_ptr, UX_MEMORY_BLOCK_HEADER_SIZE));

    /* In case we are not aligned  */
    if ((int_memory_buffer & memory_alignment) != 0)
    {

        /* No, we need to align the memory buffer.  */
        int_memory_buffer += (ALIGN_TYPE)UX_MEMORY_BLOCK_HEADER_SIZE;
        int_memory_buffer += memory_alignment;
        int_memory_buffer &=  ~((ALIGN_TYPE) memory_alignment);
        int_memory_buffer -= (ALIGN_TYPE)UX_MEMORY_BLOCK_HEADER_SIZE;

        /* Setup the new free block.  */
        next_ptr = (UCHAR *)int_memory_buffer;

        /* Setup the new free block.  */
        next_block_link_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(next_ptr);
        *next_block_link_ptr =  *this_block_link_ptr;
        work_ptr =              UX_UCHAR_POINTER_ADD(next_ptr, (sizeof(UCHAR *)));
        free_ptr =              UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(work_ptr);
        *free_ptr =             UX_BYTE_BLOCK_FREE;

        /* Increase the total fragment counter.  */
        pool_ptr -> ux_byte_pool_fragments++;

        /* Update the current pointer to point at the newly created block.  */
        *this_block_link_ptr =  next_ptr;

        /* Calculate the available bytes.  */
        available_bytes -=  UX_UCHAR_POINTER_DIF(next_ptr, current_ptr);

        /* Set Current pointer to the aligned memory buffer.  */
        current_ptr = next_ptr;
    }

    /* Now we are aligned, determine if we need to split this block.  */
    if ((available_bytes - memory_size_requested) >= ((ULONG) UX_BYTE_BLOCK_MIN))
    {

        /* Split the block.  */
        next_ptr =  UX_UCHAR_POINTER_ADD(current_ptr, (memory_size_requested + UX_MEMORY_BLOCK_HEADER_SIZE));

        /* Setup the new free block.  */
        next_block_link_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(next_ptr);
        this_block_link_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(current_ptr);
        *next_block_link_ptr =  *this_block_link_ptr;
        work_ptr =              UX_UCHAR_POINTER_ADD(next_ptr, (sizeof(UCHAR *)));
        free_ptr =              UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(work_ptr);
        *free_ptr =             UX_BYTE_BLOCK_FREE;

        /* Increase the total fragment counter.  */
        pool_ptr -> ux_byte_pool_fragments++;

        /* Update the current pointer to point at the newly created block.  */
        *this_block_link_ptr =  next_ptr;

        /* Set available equal to memory size for subsequent calculation.  */
        available_bytes =  memory_size_requested;
    }

    /* In any case, mark the current block as allocated.  */
    work_ptr =              UX_UCHAR_POINTER_ADD(current_ptr, (sizeof(UCHAR *)));
    this_block_link_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(work_ptr);
    *this_block_link_ptr =  UX_BYTE_POOL_TO_UCHAR_POINTER_CONVERT(pool_ptr);

    /* Reduce the number of available bytes in the pool.  */
    pool_ptr -> ux_byte_pool_available =  pool_ptr -> ux_byte_pool_available - (available_bytes + UX_MEMORY_BLOCK_HEADER_SIZE);

    /* Determine if the search pointer needs to be updated. This is only done
        if the search pointer matches the block to be returned.  */
    if (current_ptr == pool_ptr -> ux_byte_pool_search)
    {

        /* Yes, update the search pointer to the next block.  */
        this_block_link_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(current_ptr);
        pool_ptr -> ux_byte_pool_search =  *this_block_link_ptr;
    }

    /* Adjust the pointer for the application.  */
    work_ptr =  UX_UCHAR_POINTER_ADD(current_ptr, UX_MEMORY_BLOCK_HEADER_SIZE);

    /* Clear the memory block.  */
    _ux_utility_memory_set(work_ptr, 0, available_bytes); /* Use case of memset is verified. */

#ifdef UX_ENABLE_MEMORY_STATISTICS

    /* Update allocate count, total size.  */
    if (memory_cache_flag == UX_REGULAR_MEMORY)
        index = UX_MEMORY_BYTE_POOL_REGULAR;
    else
        index = UX_MEMORY_BYTE_POOL_CACHE_SAFE;

    /* Update allocate count, total size.  */
    _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_count ++;
    _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_total += (available_bytes + UX_MEMORY_BLOCK_HEADER_SIZE);

    if (_ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_max_count < _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_count)
        _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_max_count = _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_count;

    if (_ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_max_total < _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_total)
        _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_max_total = _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_total;

    /* Log max usage of memory pool.  */
    if (_ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_min_free > _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_available)
        _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_min_free = _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_available;
#endif

    /* Release the protection.  */
    _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

    return(work_ptr);
}
