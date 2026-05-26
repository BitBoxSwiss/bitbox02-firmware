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
/**   System                                                              */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/


/* Define UX_SYSTEM_INIT to bring in the USBX version ID string.  */

#define UX_SYSTEM_INIT


/* Include necessary system files.  */

#define UX_SOURCE_CODE

#include "ux_api.h"
#include "ux_system.h"

/* Define the USBX system data structure.  */

UX_SYSTEM         *_ux_system;
UX_SYSTEM_OTG     *_ux_system_otg;

/* Define names of all the packed descriptors in USBX.  */

UCHAR _ux_system_endpoint_descriptor_structure[] =                          {1,1,1,1,2,1 };
UCHAR _ux_system_device_descriptor_structure[] =                            {1,1,2,1,1,1,1,2,2,2,1,1,1,1};
UCHAR _ux_system_configuration_descriptor_structure[] =                     {1,1,2,1,1,1,1,1};
UCHAR _ux_system_interface_descriptor_structure[] =                         {1,1,1,1,1,1,1,1,1};
UCHAR _ux_system_interface_association_descriptor_structure[] =             {1,1,1,1,1,1,1,1};
UCHAR _ux_system_string_descriptor_structure[] =                            {1,1,2};
UCHAR _ux_system_dfu_functional_descriptor_structure[] =                    {1,1,1,2,2,2};
UCHAR _ux_system_class_audio_interface_descriptor_structure[] =             {1,1,1,1,1,1,1,1};
UCHAR _ux_system_class_audio_input_terminal_descriptor_structure[] =        {1,1,1,1,2,1,1,2,1,1};
UCHAR _ux_system_class_audio_output_terminal_descriptor_structure[] =       {1,1,1,1,2,1,1,1};
UCHAR _ux_system_class_audio_feature_unit_descriptor_structure[] =          {1,1,1,1,1,1,1};
UCHAR _ux_system_class_audio_streaming_interface_descriptor_structure[] =   {1,1,1,1,1,1};
UCHAR _ux_system_class_audio_streaming_endpoint_descriptor_structure[] =    {1,1,1,1,1,1};
UCHAR _ux_system_hub_descriptor_structure[] =                               {1,1,1,2,1,1,1,1};
UCHAR _ux_system_hid_descriptor_structure[] =                               {1,1,2,1,1,1,2};
UCHAR _ux_system_class_pima_storage_structure[] =                           {2,2,2,4,4,4,4,4};
UCHAR _ux_system_class_pima_object_structure[] =                            {4,2,2,4,2,4,4,4,4,4,4,4,2,4,4};
UCHAR _ux_system_ecm_interface_descriptor_structure[] =                     {1,1,1,1,4,2,2,1};

UCHAR _ux_system_bos_descriptor_structure[] =                               {1,1,2,1};
UCHAR _ux_system_usb_2_0_extension_descriptor_structure[] =                 {1,1,1,4};
UCHAR _ux_system_container_id_descriptor_structure[] =                      {1,1,1,1,4,4,4,4};


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_system_initialize                               PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function initializes the various control data structures for   */
/*    the USBX system.                                                    */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    regular_memory_pool_start        Start of non cached memory pool    */
/*    regular_memory_size              Size of non cached memory pool     */
/*    cache_safe_memory_pool_start     Start of cached memory pool        */
/*    cache_safe_memory_size           Size of cached memory pool         */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_memory_allocate           Allocate memory               */
/*    _ux_utility_memory_set                Set memory                    */
/*    _ux_utility_mutex_create              Create mutex                  */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
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
/*  12-31-2020     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added BOS support,          */
/*                                            resulting in version 6.1.3  */
/*  01-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.1.10 */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            refined memory management,  */
/*                                            added UX_ASSERT check for   */
/*                                            STD descriptor parse size,  */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT  _ux_system_initialize(VOID *regular_memory_pool_start, ULONG regular_memory_size,
                            VOID *cache_safe_memory_pool_start, ULONG cache_safe_memory_size)
{
ALIGN_TYPE          int_memory_pool_start;
VOID                *regular_memory_pool_end;
VOID                *cache_safe_memory_pool_end;
ULONG               memory_pool_offset;
#if !defined(UX_STANDALONE)
UINT                status;
#endif
ULONG               pool_size;

    /* Check if the regular memory pool is valid.  */
    if ((regular_memory_pool_start == UX_NULL) || (regular_memory_size == 0))
        return(UX_INVALID_PARAMETER);

    /* Reset memory block */
    _ux_utility_memory_set(regular_memory_pool_start, 0, regular_memory_size); /* Use case of memset is verified. */

    /* Set the _ux_system structure at the start of our regular memory */
    _ux_system =  (UX_SYSTEM *) regular_memory_pool_start;

    /* Add to the memory offset the size of the allocated block.  */
    memory_pool_offset = sizeof(UX_SYSTEM);

#ifndef UX_DEVICE_SIDE_ONLY

    /* Set the _ux_system_host structure.  */
    _ux_system_host =  (UX_SYSTEM_HOST *) (((UCHAR *) regular_memory_pool_start) + memory_pool_offset);

    /* Add to the memory offset the size of the allocated block.  */
    memory_pool_offset += (ULONG)sizeof(UX_SYSTEM_HOST);

#endif

#ifndef UX_HOST_SIDE_ONLY

    /* Set the _ux_system_slave structure.  */
    _ux_system_slave =  (UX_SYSTEM_SLAVE *) (((UCHAR *) regular_memory_pool_start) + memory_pool_offset);

    /* Add to the memory offset the size of the allocated block.  */
    memory_pool_offset += (ULONG)sizeof(UX_SYSTEM_SLAVE);

#endif


#ifdef UX_OTG_SUPPORT

    /* Set the _ux_system_otg structure.  */
    _ux_system_otg =  (UX_SYSTEM_OTG *) (((UCHAR *) regular_memory_pool_start) + memory_pool_offset);

    /* Add to the memory offset the size of the allocated block.  */
    memory_pool_offset += (ULONG)sizeof(UX_SYSTEM_OTG);
#endif

    /* Set the regular memory pool structure.  */
    _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] = (UX_MEMORY_BYTE_POOL *) (((UCHAR *) regular_memory_pool_start) + memory_pool_offset);

    /* Add to the memory offset the size of the allocated block.  */
    memory_pool_offset += (ULONG)sizeof(UX_MEMORY_BYTE_POOL);

    /* Check if the cache save memory pool is valid.  */
    if ((cache_safe_memory_pool_start != UX_NULL) && (cache_safe_memory_size != 0))
    {

        /* Set the cache safe memory pool structure.  */
        _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE] = (UX_MEMORY_BYTE_POOL *) (((UCHAR *) regular_memory_pool_start) + memory_pool_offset);

        /* Add to the memory offset the size of the allocated block.  */
        memory_pool_offset += (ULONG)sizeof(UX_MEMORY_BYTE_POOL);
    }
    else
    {

        /* Set the cache safe memory pool structure to regular pool. */
        _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE] = _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR];
    }

    /* Make sure the regular memory pool is aligned properly */
    int_memory_pool_start = (ALIGN_TYPE) (((UCHAR *) regular_memory_pool_start) + memory_pool_offset);
    int_memory_pool_start += UX_ALIGN_MIN;
    int_memory_pool_start &= ~((ALIGN_TYPE)UX_ALIGN_MIN);

    /* Set the end of the regular memory pool.  */
    regular_memory_pool_end =  (void *) (((UCHAR *) regular_memory_pool_start) + regular_memory_size);

    /* Check if we have memory available.  */
    if (int_memory_pool_start >= (ALIGN_TYPE)regular_memory_pool_end)
    {

        /* No memory available.  */
        return(UX_MEMORY_INSUFFICIENT);
    }

    /* get the regular memory pool size.  */
    pool_size = (ULONG) (((ALIGN_TYPE) regular_memory_pool_end) - int_memory_pool_start);

    /* Create the regular memory pool.  */
    _ux_utility_memory_byte_pool_create(_ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR],
                                        (UX_MEMORY_BYTE_POOL *)int_memory_pool_start,
                                        pool_size);

    /* Check the definition of the cache safe pool. If the application or controller do not require any cache safe memory,
       define the cached safe memory region as the regular memory region.  */
    if ((cache_safe_memory_pool_start != UX_NULL) && (cache_safe_memory_size != 0))
    {

        /* Reset this memory block */
        _ux_utility_memory_set(cache_safe_memory_pool_start, 0, cache_safe_memory_size); /* Use case of memset is verified. */

        /* Make sure the cache safe memory pool is aligned properly */
        int_memory_pool_start =   (ALIGN_TYPE) cache_safe_memory_pool_start;
        int_memory_pool_start +=  UX_ALIGN_MIN;
        int_memory_pool_start &=  ~((ALIGN_TYPE)UX_ALIGN_MIN);

        cache_safe_memory_pool_end =  (void *) (((UCHAR *) cache_safe_memory_pool_start) + cache_safe_memory_size);

        /* Check if we have memory available.  */
        if (int_memory_pool_start >= (ALIGN_TYPE) cache_safe_memory_pool_end)
        {

            /* No memory available.  */
            return(UX_MEMORY_INSUFFICIENT);
        }

        pool_size = (ULONG) (((ALIGN_TYPE) cache_safe_memory_pool_end) - int_memory_pool_start);

        _ux_utility_memory_byte_pool_create(_ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE],
                                            (UX_MEMORY_BYTE_POOL *)int_memory_pool_start, pool_size);
    }

#ifdef UX_ENABLE_MEMORY_STATISTICS
    _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_min_free =
            _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_REGULAR] -> ux_byte_pool_available;
    _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE] -> ux_byte_pool_min_free =
            _ux_system -> ux_system_memory_byte_pool[UX_MEMORY_BYTE_POOL_CACHE_SAFE] -> ux_byte_pool_available;

    /* Other fields are kept zero.  */
#endif

#ifdef UX_ENABLE_DEBUG_LOG

    /* Obtain memory for storing the debug log.  */
    _ux_system -> ux_system_debug_log_buffer =  _ux_utility_memory_allocate(UX_NO_ALIGN, UX_REGULAR_MEMORY, UX_DEBUG_LOG_SIZE);
    if (_ux_system -> ux_system_debug_log_buffer == UX_NULL)
        return(UX_MEMORY_INSUFFICIENT);

    /* Setup the head and tail pointers.  */
    _ux_system -> ux_system_debug_log_head = _ux_system -> ux_system_debug_log_buffer;
    _ux_system -> ux_system_debug_log_tail = _ux_system -> ux_system_debug_log_buffer;

    /* Keep the size in system structure variable.  */
    _ux_system -> ux_system_debug_log_size = UX_DEBUG_LOG_SIZE;

#endif

#if !defined(UX_STANDALONE)

    /* Create the Mutex object used by USBX to control critical sections.  */
    status =  _ux_system_mutex_create(&_ux_system -> ux_system_mutex, "ux_system_mutex");
    if(status != UX_SUCCESS)
        return(UX_MUTEX_ERROR);
#endif

    return(UX_SUCCESS);
}


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_system_initialize                              PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in system initialization function call. */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    regular_memory_pool_start        Start of non cached memory pool    */
/*    regular_memory_size              Size of non cached memory pool     */
/*    cache_safe_memory_pool_start     Start of cached memory pool        */
/*    cache_safe_memory_size           Size of cached memory pool         */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_system_initialize                 Get encoded feedback          */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Chaoqiong Xiao           Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
UINT  _uxe_system_initialize(VOID *regular_memory_pool_start, ULONG regular_memory_size,
                            VOID *cache_safe_memory_pool_start, ULONG cache_safe_memory_size)
{

    /* Compiling option check of descriptors structs.  */
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_endpoint_descriptor_structure, UX_ENDPOINT_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_ENDPOINT_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_device_descriptor_structure, UX_DEVICE_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_DEVICE_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_configuration_descriptor_structure, UX_CONFIGURATION_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_CONFIGURATION_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_interface_descriptor_structure, UX_INTERFACE_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_INTERFACE_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_interface_association_descriptor_structure, UX_INTERFACE_ASSOCIATION_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_INTERFACE_ASSOCIATION_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_string_descriptor_structure, UX_STRING_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_STRING_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_dfu_functional_descriptor_structure, UX_DFU_FUNCTIONAL_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_DFU_FUNCTIONAL_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_bos_descriptor_structure, UX_BOS_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_BOS_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_usb_2_0_extension_descriptor_structure, UX_USB_2_0_EXTENSION_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_USB_2_0_EXTENSION_DESCRIPTOR));
    UX_ASSERT((_ux_utility_descriptor_parse_size(_ux_system_container_id_descriptor_structure, UX_CONTAINER_ID_DESCRIPTOR_ENTRIES, 0x3u)) == sizeof(UX_CONTAINER_ID_DESCRIPTOR));


    /* Sanity check.  */
    if ((regular_memory_pool_start == UX_NULL) || (regular_memory_size == 0))
            return(UX_INVALID_PARAMETER);

    /* Invoke system initialization function.  */
    return(_ux_system_initialize(regular_memory_pool_start, regular_memory_size,
                                 cache_safe_memory_pool_start, cache_safe_memory_size));
}
