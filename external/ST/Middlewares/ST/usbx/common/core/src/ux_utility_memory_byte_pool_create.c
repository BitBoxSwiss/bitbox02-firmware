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
/*    _ux_utility_memory_byte_pool_create                 PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function creates a pool of memory bytes in the specified       */
/*    memory area.                                                        */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    pool_ptr                          Pointer to pool control block     */
/*    pool_start                        Address of beginning of pool area */
/*    pool_size                         Number of bytes in the byte pool  */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    UX_SUCCESS                        Successful completion status      */
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
UINT  _ux_utility_memory_byte_pool_create(UX_MEMORY_BYTE_POOL *pool_ptr, VOID *pool_start, ULONG pool_size)
{

UCHAR               *block_ptr;
UCHAR               **block_indirect_ptr;
UCHAR               *temp_ptr;
ALIGN_TYPE          *free_ptr;


    /* Initialize the byte pool control block to all zeros.  */
    _ux_utility_memory_set((UCHAR *)pool_ptr, 0, sizeof(UX_MEMORY_BYTE_POOL)); /* Use case of memset is verified. */

    /* Round the pool size down to something that is evenly divisible by
       an ULONG.  */
    pool_size =   (pool_size/(sizeof(ALIGN_TYPE))) * (sizeof(ALIGN_TYPE));

    /* Save the start and size of the pool.  */
    pool_ptr -> ux_byte_pool_start =   UX_VOID_TO_UCHAR_POINTER_CONVERT(pool_start);
    pool_ptr -> ux_byte_pool_size =    pool_size;
    pool_ptr -> ux_byte_pool_search =  UX_VOID_TO_UCHAR_POINTER_CONVERT(pool_start);

    /* Initially, the pool will have two blocks.  One large block at the
       beginning that is available and a small allocated block at the end
       of the pool that is there just for the algorithm.  Be sure to count
       the available block's header in the available bytes count.  */
    pool_ptr -> ux_byte_pool_available =   pool_size - ((sizeof(VOID *)) + (sizeof(ALIGN_TYPE)));
    pool_ptr -> ux_byte_pool_fragments =   ((UINT) 2);

    /* Each block contains a "next" pointer that points to the next block in the pool followed by a ALIGN_TYPE
       field that contains either the constant UX_BYTE_BLOCK_FREE (if the block is free) or a pointer to the
       owning pool (if the block is allocated).  */

    /* Calculate the end of the pool's memory area.  */
    block_ptr =  UX_VOID_TO_UCHAR_POINTER_CONVERT(pool_start);
    block_ptr =  UX_UCHAR_POINTER_ADD(block_ptr, pool_size);

    /* Backup the end of the pool pointer and build the pre-allocated block.  */
    block_ptr =  UX_UCHAR_POINTER_SUB(block_ptr, (sizeof(ALIGN_TYPE)));

    /* Cast the pool pointer into a ULONG.  */
    temp_ptr =             UX_BYTE_POOL_TO_UCHAR_POINTER_CONVERT(pool_ptr);
    block_indirect_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(block_ptr);
    *block_indirect_ptr =  temp_ptr;

    block_ptr =            UX_UCHAR_POINTER_SUB(block_ptr, (sizeof(UCHAR *)));
    block_indirect_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(block_ptr);
    *block_indirect_ptr =  UX_VOID_TO_UCHAR_POINTER_CONVERT(pool_start);

    /* Now setup the large available block in the pool.  */
    temp_ptr =             UX_VOID_TO_UCHAR_POINTER_CONVERT(pool_start);
    block_indirect_ptr =   UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(temp_ptr);
    *block_indirect_ptr =  block_ptr;
    block_ptr =            UX_VOID_TO_UCHAR_POINTER_CONVERT(pool_start);
    block_ptr =            UX_UCHAR_POINTER_ADD(block_ptr, (sizeof(UCHAR *)));
    free_ptr =             UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(block_ptr);
    *free_ptr =            UX_BYTE_BLOCK_FREE;

    /* Return UX_SUCCESS.  */
    return(UX_SUCCESS);
}
