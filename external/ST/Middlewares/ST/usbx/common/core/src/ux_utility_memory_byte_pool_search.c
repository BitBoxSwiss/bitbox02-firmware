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
/**   USBX main stack                                                     */
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
/*    _ux_utility_memory_byte_pool_search                 PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function searches a byte pool for a memory block to satisfy    */
/*    the requested number of bytes.  Merging of adjacent free blocks     */
/*    takes place during the search.                                      */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    pool_ptr                          Pointer to pool control block     */
/*    memory_size                       Number of bytes required          */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    UCHAR *                           Pointer to the allocated memory,  */
/*                                        if successful.  Otherwise, a    */
/*                                        NULL is returned                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Components                                                     */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Yajun Xia                Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
UCHAR  *_ux_utility_memory_byte_pool_search(UX_MEMORY_BYTE_POOL *pool_ptr, ULONG memory_size)
{
UCHAR               *current_ptr;
UCHAR               *next_ptr;
UCHAR               **this_block_link_ptr;
UCHAR               **next_block_link_ptr;
ULONG               available_bytes;
UINT                examine_blocks;
UINT                first_free_block_found =  UX_FALSE;
ALIGN_TYPE          *free_ptr;
UCHAR               *work_ptr;
ULONG               total_theoretical_available;

    /* First, determine if there are enough bytes in the pool.  */
    /* Theoretical bytes available = free bytes + ((fragments-2) * overhead of each block) */
    total_theoretical_available = pool_ptr -> ux_byte_pool_available + ((pool_ptr -> ux_byte_pool_fragments - 2) * UX_MEMORY_BLOCK_HEADER_SIZE);
    if (memory_size >= total_theoretical_available)
    {

        /* Not enough memory, return a NULL pointer.  */
        return(UX_NULL);
    }

    /* Check if the search pointer is valid.  */
    if ((pool_ptr -> ux_byte_pool_search < pool_ptr -> ux_byte_pool_start) ||
        (pool_ptr -> ux_byte_pool_search > pool_ptr -> ux_byte_pool_start + pool_ptr -> ux_byte_pool_size))
    {

        /* Return a NULL pointer.  */
        return(UX_NULL);
    }

    /* Walk through the memory pool in search for a large enough block.  */
    current_ptr =      pool_ptr -> ux_byte_pool_search;
    examine_blocks =   pool_ptr -> ux_byte_pool_fragments + ((UINT) 1);
    available_bytes =  ((ULONG) 0);
    do
    {
        /* Check to see if this block is free.  */
        work_ptr =  UX_UCHAR_POINTER_ADD(current_ptr, (sizeof(UCHAR *)));
        free_ptr =  UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(work_ptr);
        if ((*free_ptr) == UX_BYTE_BLOCK_FREE)
        {

            /* Determine if this is the first free block.  */
            if (first_free_block_found == UX_FALSE)
            {
                /* This is the first free block.  */
                pool_ptr->ux_byte_pool_search =  current_ptr;

                /* Set the flag to indicate we have found the first free
                    block.  */
                first_free_block_found =  UX_TRUE;
            }

            /* Block is free, see if it is large enough.  */

            /* Pickup the next block's pointer.  */
            this_block_link_ptr =  UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(current_ptr);
            next_ptr =             *this_block_link_ptr;

            /* Calculate the number of bytes available in this block.  */
            available_bytes =   UX_UCHAR_POINTER_DIF(next_ptr, current_ptr);
            available_bytes =   available_bytes - UX_MEMORY_BLOCK_HEADER_SIZE;

            /* If this is large enough, we are done because our first-fit algorithm
                has been satisfied!  */
            if (available_bytes >= memory_size)
            {

                /* Get out of the search loop!  */
                break;
            }
            else
            {

                /* Clear the available bytes variable.  */
                available_bytes =  ((ULONG) 0);

                /* Not enough memory, check to see if the neighbor is
                    free and can be merged.  */
                work_ptr =  UX_UCHAR_POINTER_ADD(next_ptr, (sizeof(UCHAR *)));
                free_ptr =  UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(work_ptr);
                if ((*free_ptr) == UX_BYTE_BLOCK_FREE)
                {

                    /* Yes, neighbor block can be merged!  This is quickly accomplished
                        by updating the current block with the next blocks pointer.  */
                    next_block_link_ptr =  UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(next_ptr);
                    *this_block_link_ptr =  *next_block_link_ptr;

                    /* Reduce the fragment total.  We don't need to increase the bytes
                        available because all free headers are also included in the available
                        count.  */
                    pool_ptr -> ux_byte_pool_fragments--;

                    /* See if the search pointer is affected.  */
                    if (pool_ptr -> ux_byte_pool_search ==  next_ptr)
                    {
                        /* Yes, update the search pointer.   */
                        pool_ptr -> ux_byte_pool_search =  current_ptr;
                    }
                }
                else
                {
                    /* Neighbor is not free so we can skip over it!  */
                    next_block_link_ptr =  UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(next_ptr);
                    current_ptr =  *next_block_link_ptr;

                    /* Decrement the examined block count to account for this one.  */
                    if (examine_blocks != ((UINT) 0))
                    {
                        examine_blocks--;
                    }
                }
            }
        }
        else
        {

            /* Block is not free, move to next block.  */
            this_block_link_ptr =  UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(current_ptr);
            current_ptr =  *this_block_link_ptr;
        }

        /* Another block has been searched... decrement counter.  */
        if (examine_blocks != ((UINT) 0))
        {

            examine_blocks--;
        }

    } while(examine_blocks != ((UINT) 0));

    /* If a block was found, just return. */
    if (available_bytes == ((ULONG) 0))
    {
        return(UX_NULL);
    }

    /* Return the search pointer.  */
    return(current_ptr);
}
