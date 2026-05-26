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
/*    _ux_utility_memory_free                             PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function frees a previously allocated memory block.            */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    memory                                Pointer to memory block       */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_mutex_on                  Start system protection       */
/*    _ux_utility_mutex_off                 End system protection         */
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
/*                                            resulting in version 6.1    */
/*  01-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.1.10 */
/*  10-31-2023     Yajun Xia, CQ Xiao       Modified comment(s),          */
/*                                            added some error traps,     */
/*                                            refined memory management,  */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
VOID  _ux_utility_memory_free(VOID *memory)
{
UX_MEMORY_BYTE_POOL *pool_ptr;
UCHAR               *work_ptr;
UCHAR               *temp_ptr;
UCHAR               *next_block_ptr;

ALIGN_TYPE          *free_ptr;
UX_MEMORY_BYTE_POOL **byte_pool_ptr;
UCHAR               **block_link_ptr;
#ifdef UX_ENABLE_MEMORY_POOL_SANITY_CHECK
UCHAR               *memory_address;
UCHAR               *regular_start, *regular_end;
UCHAR               *cache_safe_start, *cache_safe_end;
#endif
#ifdef UX_ENABLE_MEMORY_STATISTICS
UINT                index;
#endif

    /* Get the mutex as this is a critical section.  */
    _ux_system_mutex_on(&_ux_system -> ux_system_mutex);

#ifdef UX_ENABLE_MEMORY_POOL_SANITY_CHECK

    /* Sanity check, check if the memory is in memory pool.  */
    regular_start = (UCHAR *)_ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_start;
    regular_end = regular_start + _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_size;
    regular_start += UX_MEMORY_BLOCK_HEADER_SIZE;
    cache_safe_start = (UCHAR *)_ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE] -> ux_byte_pool_start;
    cache_safe_end = cache_safe_start + _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE] -> ux_byte_pool_size;
    cache_safe_start += UX_MEMORY_BLOCK_HEADER_SIZE;

    memory_address = (UCHAR *)memory;
    if (!((memory_address >= regular_start    && memory_address < regular_end) ||
          (memory_address >= cache_safe_start && memory_address < cache_safe_end)))
    {

        /* Not valid. Release the protection.  */
        _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

        /* Error trap.  */
        _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD,
                                UX_SYSTEM_CONTEXT_UTILITY, UX_MEMORY_CORRUPTED);

        /* No action taken.  */
        return;
    }
#endif

    /* Set the pool pointer to NULL.  */
    pool_ptr =  UX_NULL;

    /* Determine if the memory pointer is valid.  */
    work_ptr =  UX_VOID_TO_UCHAR_POINTER_CONVERT(memory);
    if (work_ptr != UX_NULL)
    {

        /* Back off the memory pointer to pickup its header.  */
        work_ptr =  UX_UCHAR_POINTER_SUB(work_ptr, UX_MEMORY_BLOCK_HEADER_SIZE);

        /* There is a pointer, pickup the pool pointer address.  */
        temp_ptr =  UX_UCHAR_POINTER_ADD(work_ptr, (sizeof(UCHAR *)));
        free_ptr =  UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(temp_ptr);
        if ((*free_ptr) != UX_BYTE_BLOCK_FREE)
        {

            /* Pickup the pool pointer.  */
            temp_ptr =  UX_UCHAR_POINTER_ADD(work_ptr, (sizeof(UCHAR *)));
            byte_pool_ptr = UX_UCHAR_TO_INDIRECT_BYTE_POOL_POINTER(temp_ptr);
            pool_ptr = *byte_pool_ptr;

            /* See if we have a valid pool pointer.  */
            if ((pool_ptr == UX_NULL) ||
                ((pool_ptr != _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR]) &&
                (pool_ptr != _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE])))
            {

                /* Release the protection.  */
                _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

                /* Error trap: maybe double free/memory issue here!  */
                _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD,
                                         UX_SYSTEM_CONTEXT_UTILITY, UX_MEMORY_CORRUPTED);

                /* Return to caller.  */
                return;
            }
        }
        else
        {
            /* Release the protection.  */
            _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

            /* Error trap: maybe double free/memory issue here!  */
            _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD,
                                     UX_SYSTEM_CONTEXT_UTILITY, UX_MEMORY_CORRUPTED);

            /* Return to caller.  */
            return;
        }
    }
    else
    {

        /* Release the protection.  */
        _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

        /* Error trap: maybe double free/bad flow here!  */
        _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD,
                                    UX_SYSTEM_CONTEXT_UTILITY, UX_MEMORY_CORRUPTED);

        /* Return to caller.  */
        return;
    }

    /* At this point, we know that the pool pointer is valid.  */

    /* Release the memory.  */
    temp_ptr =   UX_UCHAR_POINTER_ADD(work_ptr, (sizeof(UCHAR *)));
    free_ptr =   UX_UCHAR_TO_ALIGN_TYPE_POINTER_CONVERT(temp_ptr);
    *free_ptr =  UX_BYTE_BLOCK_FREE;

    /* Update the number of available bytes in the pool.  */
    block_link_ptr =  UX_UCHAR_TO_INDIRECT_UCHAR_POINTER_CONVERT(work_ptr);
    next_block_ptr =  *block_link_ptr;
    pool_ptr -> ux_byte_pool_available =
        pool_ptr -> ux_byte_pool_available + UX_UCHAR_POINTER_DIF(next_block_ptr, work_ptr);

    /* Determine if the free block is prior to current search pointer.  */
    if (work_ptr < (pool_ptr -> ux_byte_pool_search))
    {

        /* Yes, update the search pointer to the released block.  */
        pool_ptr -> ux_byte_pool_search =  work_ptr;
    }

#ifdef UX_ENABLE_MEMORY_STATISTICS
    if (((UCHAR*)memory >= _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_start) &&
        ((UCHAR*)memory < (_ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_start + _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_size)))
        index = UX_MEMORY_BYTE_POOL_REGULAR;
    else
        index = UX_MEMORY_BYTE_POOL_CACHE_SAFE;

    _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_count --;
    _ux_system -> ux_system_memory_byte_pool[index] -> ux_byte_pool_alloc_total -= UX_UCHAR_POINTER_DIF(next_block_ptr, work_ptr);
#endif

    /* Release the protection.  */
    _ux_system_mutex_off(&_ux_system -> ux_system_mutex);

    /* Return to caller.  */
    return;
}

