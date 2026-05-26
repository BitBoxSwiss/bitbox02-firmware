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
/**                                                                       */
/** USBX Component                                                        */
/**                                                                       */
/**   Device HID Class                                                    */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_hid.h"
#include "ux_device_stack.h"


#if defined(UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT)
/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_hid_receiver_uninitialize          PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function uninitialize the USB HID device receiver. It's only   */
/*    done once.                                                          */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    receiver                             Pointer to receiver instance   */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_memory_free               Free memory                   */
/*    _ux_utility_thread_delete             Delete thread                 */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Source Code                                                    */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  01-31-2022     Chaoqiong Xiao           Initial Version 6.1.10        */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added zero copy support,    */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
VOID _ux_device_class_hid_receiver_uninitialize(UX_DEVICE_CLASS_HID_RECEIVER *receiver)
{

#if !defined(UX_DEVICE_STANDALONE)

    /* Delete receiver thread.  */
    _ux_utility_thread_delete(&receiver -> ux_device_class_hid_receiver_thread);
#endif

#if (UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1) && defined(UX_DEVICE_CLASS_HID_ZERO_COPY)

    /* Free cache safe event memory.  */
    _ux_utility_memory_free(receiver -> ux_device_class_hid_receiver_events -> ux_device_class_hid_received_event_data);
#endif

    /* Free receiver and events memory.  */
    _ux_utility_memory_free(receiver);
}
#endif
