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
/**   Device HID Class                                                    */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_hid.h"
#include "ux_device_stack.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_hid_event_check                    PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks if there is an event from the application and  */
/*    fill a pointer to access the event.                                 */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    hid                                      Address of hid class       */
/*    event                                    Pointer to fill address    */
/*                                             to access event            */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    status                                   UX_SUCCESS if there is an  */
/*                                             event                      */
/*  CALLS                                                                 */
/*                                                                        */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Device HID Class                                                    */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Chaoqiong Xiao           Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
UINT  _ux_device_class_hid_event_check(UX_SLAVE_CLASS_HID *hid,
                                       UX_DEVICE_CLASS_HID_EVENT **hid_event)
{
UX_SLAVE_DEVICE                 *device;

    /* Get the pointer to the device.  */
    device =  &_ux_system_slave -> ux_system_slave_device;

    /* Check the device state.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
        return(UX_DEVICE_HANDLE_UNKNOWN);

    /* Check if the head and the tail of the event array is the same.  */
    if (hid -> ux_device_class_hid_event_array_head ==
        hid -> ux_device_class_hid_event_array_tail)

        /* No event to report.  */
        return(UX_ERROR);

    /* There is an event to report, get the current pointer to the event.  */
    *hid_event =  hid -> ux_device_class_hid_event_array_tail;
    return(UX_SUCCESS);
}


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_hid_event_free                     PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function free the event in queue tail.                         */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    hid                                      Address of hid class       */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Device HID Class                                                    */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Chaoqiong Xiao           Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
VOID  _ux_device_class_hid_event_free(UX_SLAVE_CLASS_HID *hid)
{
UCHAR                           *pos;

    pos = (UCHAR *) hid -> ux_device_class_hid_event_array_tail;
    pos += UX_DEVICE_CLASS_HID_EVENT_QUEUE_ITEM_SIZE(hid);
    if (pos >= (UCHAR *) hid -> ux_device_class_hid_event_array_end)
        pos = (UCHAR *) hid -> ux_device_class_hid_event_array;
    hid -> ux_device_class_hid_event_array_tail = (UX_DEVICE_CLASS_HID_EVENT *) pos;
}


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_hid_event_get                      PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks if there is an event from the application      */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    hid                                      Address of hid class       */
/*    event                                    Pointer of the event       */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    status                                   UX_SUCCESS if there is an  */
/*                                             event                      */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_memory_copy                  Copy memory                */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    ThreadX                                                             */
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
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added zero copy support,    */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT  _ux_device_class_hid_event_get(UX_SLAVE_CLASS_HID *hid,
                                     UX_SLAVE_CLASS_HID_EVENT *hid_event)
{

UX_DEVICE_CLASS_HID_EVENT       *current_hid_event;
UINT                            status;

    /* If trace is enabled, insert this event into the trace buffer.  */
    UX_TRACE_IN_LINE_INSERT(UX_TRACE_DEVICE_CLASS_HID_EVENT_GET, hid, hid_event, 0, 0, UX_TRACE_DEVICE_CLASS_EVENTS, 0, 0)

    /* Check and get event pointer.  */
    status = _ux_device_class_hid_event_check(hid, &current_hid_event);
    if (status != UX_SUCCESS)
        return(status);

    /* Keep the event data length inside buffer area.  */
    if (current_hid_event -> ux_device_class_hid_event_length > UX_DEVICE_CLASS_HID_EVENT_MAX_LENGTH(hid))
        current_hid_event -> ux_device_class_hid_event_length = UX_DEVICE_CLASS_HID_EVENT_MAX_LENGTH(hid);

    /* fill in the event structure from the user.  */
    hid_event -> ux_device_class_hid_event_length =  current_hid_event -> ux_device_class_hid_event_length;

    /* Copy the event data into the user buffer.  */
    _ux_utility_memory_copy(hid_event -> ux_device_class_hid_event_buffer,
                            UX_DEVICE_CLASS_HID_EVENT_BUFFER(current_hid_event),
                            current_hid_event -> ux_device_class_hid_event_length); /* Use case of memcpy is verified. */

    /* Free the tail event.  */
    _ux_device_class_hid_event_free(hid);

    /* Return event status to the user.  */
    return(UX_SUCCESS);
}


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_hid_event_get                     PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in HID event get function call.         */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    hid                                   Pointer to hid instance       */
/*    hid_event                             Pointer to hid event          */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_hid_event_get        Get an HID event              */
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
UINT  _uxe_device_class_hid_event_get(UX_SLAVE_CLASS_HID *hid,
                                      UX_SLAVE_CLASS_HID_EVENT *hid_event)
{

    /* Sanity checks.  */
    if ((hid == UX_NULL) || (hid_event == UX_NULL))
        return(UX_INVALID_PARAMETER);

    /* Invoke function to get event.  */
    return(_ux_device_class_hid_event_get(hid, hid_event));
}
