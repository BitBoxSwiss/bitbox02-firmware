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
/**   Device Printer Class                                                */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_printer.h"
#include "ux_device_stack.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_printer_write                      PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function writes to  the Printer class.                         */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    printer                               Address of printer class      */
/*                                            instance                    */
/*    buffer                                Pointer to data to write      */
/*    requested_length                      Length of bytes to write,     */
/*                                            set to 0 to issue ZLP       */
/*    actual_length                         Pointer to save number of     */
/*                                            bytes written               */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*   _ux_utility_memory_copy                Copy memory                   */
/*   _ux_device_stack_transfer_request      Transfer request              */
/*   _ux_device_mutex_on                    Take Mutex                    */
/*   _ux_device_mutex_off                   Release Mutex                 */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  01-31-2022     Chaoqiong Xiao           Initial Version 6.1.10        */
/*  04-25-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            fixed standalone compile,   */
/*                                            resulting in version 6.1.11 */
/*  07-29-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added auto ZLP support,     */
/*                                            resulting in version 6.1.12 */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added a new mode to manage  */
/*                                            endpoint buffer in classes, */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT _ux_device_class_printer_write(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length)
{

UX_SLAVE_ENDPOINT           *endpoint;
UX_SLAVE_DEVICE             *device;
UX_SLAVE_TRANSFER           *transfer_request;
ULONG                       local_requested_length;
ULONG                       local_host_length;
UINT                        status = 0;

    /* If trace is enabled, insert this event into the trace buffer.  */
    UX_TRACE_IN_LINE_INSERT(UX_TRACE_DEVICE_CLASS_PRINTER_WRITE, printer, buffer, requested_length, 0, UX_TRACE_DEVICE_CLASS_EVENTS, 0, 0)

    /* Get the pointer to the device.  */
    device =  &_ux_system_slave -> ux_system_slave_device;

    /* As long as the device is in the CONFIGURED state.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
    {

        /* Error trap. */
        _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD, UX_SYSTEM_CONTEXT_CLASS, UX_CONFIGURATION_HANDLE_UNKNOWN);

        /* If trace is enabled, insert this event into the trace buffer.  */
        UX_TRACE_IN_LINE_INSERT(UX_TRACE_ERROR, UX_CONFIGURATION_HANDLE_UNKNOWN, device, 0, 0, UX_TRACE_ERRORS, 0, 0)

        /* Cannot proceed with command, the interface is down.  */
        return(UX_CONFIGURATION_HANDLE_UNKNOWN);
    }

    /* Locate the endpoints.  */
    endpoint = printer -> ux_device_class_printer_endpoint_in;

    /* Check if it's available.  */
    if (endpoint == UX_NULL)
        return(UX_FUNCTION_NOT_SUPPORTED);

    /* Protect this thread.  */
    _ux_device_mutex_on(&printer -> ux_device_class_printer_endpoint_in_mutex);

    /* We are writing to the IN endpoint.  */
    transfer_request =  &endpoint -> ux_slave_endpoint_transfer_request;

    /* Reset the actual length.  */
    *actual_length =  0;

    /* Check if the application forces a 0 length packet.  */
    if (requested_length == 0)
    {

        /* Send the request for 0 byte packet to the device controller.  */
        status =  _ux_device_stack_transfer_request(transfer_request, 0, 0);

        /* Free Mutex resource.  */
        _ux_device_mutex_off(&printer -> ux_device_class_printer_endpoint_in_mutex);

        /* Return the status.  */
        return(status);
    }

#if (UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1) && defined(UX_DEVICE_CLASS_PRINTER_ZERO_COPY)

    /* Check if device is configured.  */
    if (device -> ux_slave_device_state == UX_DEVICE_CONFIGURED)
    {

#if defined(UX_DEVICE_CLASS_PRINTER_WRITE_AUTO_ZLP)

        /* Issue with larger host length to append zlp if necessary.  */
        local_host_length = requested_length + 1;
#else
        local_host_length = requested_length;
#endif
        local_requested_length = requested_length;

        /* Issue the transfer request.  */
        transfer_request -> ux_slave_transfer_request_data_pointer =  buffer;
        status = _ux_device_stack_transfer_request(transfer_request, local_requested_length, local_host_length);
        *actual_length = transfer_request -> ux_slave_transfer_request_actual_length;
    }
#else

    /* Check if we need more transactions.  */
    local_host_length = UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE;
    while (device -> ux_slave_device_state == UX_DEVICE_CONFIGURED &&
            requested_length != 0)
    {

        /* Check if we have enough in the local buffer.  */
        if (requested_length > UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE)

            /* We have too much to transfer.  */
            local_requested_length = UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE;

        else
        {

            /* We can proceed with the demanded length.  */
            local_requested_length = requested_length;

#if !defined(UX_DEVICE_CLASS_PRINTER_WRITE_AUTO_ZLP)

            /* Assume expected length matches.  */
            local_host_length = requested_length;
#else

            /* Assume expected more so stack appends ZLP if needed.  */
            local_host_length = UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE + 1;
#endif
        }

        /* On a out, we copy the buffer to the caller. Not very efficient but it makes the API
            easier.  */
        _ux_utility_memory_copy(transfer_request -> ux_slave_transfer_request_data_pointer,
                            buffer, local_requested_length); /* Use case of memcpy is verified. */

        /* Send the request to the device controller.  */
        status =  _ux_device_stack_transfer_request(transfer_request,
                                local_requested_length, local_host_length);

        /* Check the status */
        if (status == UX_SUCCESS)
        {

            /* Next buffer address.  */
            buffer += transfer_request -> ux_slave_transfer_request_actual_length;

            /* Set the length actually received. */
            *actual_length += transfer_request -> ux_slave_transfer_request_actual_length;

            /* Decrement what left has to be done.  */
            requested_length -= transfer_request -> ux_slave_transfer_request_actual_length;
        }
        else
        {

            /* Free Mutex resource.  */
            _ux_device_mutex_off(&printer -> ux_device_class_printer_endpoint_in_mutex);

            /* We had an error, abort.  */
            return(status);
        }
    }
#endif

    /* Free Mutex resource.  */
    _ux_device_mutex_off(&printer -> ux_device_class_printer_endpoint_in_mutex);

    /* Check why we got here, either completion or device was extracted.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
    {

        /* Error trap. */
        _ux_system_error_handler(UX_SYSTEM_LEVEL_THREAD, UX_SYSTEM_CONTEXT_CLASS, UX_TRANSFER_NO_ANSWER);

        /* If trace is enabled, insert this event into the trace buffer.  */
        UX_TRACE_IN_LINE_INSERT(UX_TRACE_ERROR, UX_TRANSFER_NO_ANSWER, transfer_request, 0, 0, UX_TRACE_ERRORS, 0, 0)

        /* Device must have been extracted.  */
        return (UX_TRANSFER_NO_ANSWER);
    }
    else

        /* Simply return the last transaction result.  */
        return(status);
}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_printer_write                       PORTABLE C    */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in printer class write function         */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    printer                               Address of printer class      */
/*                                            instance                    */
/*    buffer                                Pointer to data to write      */
/*    requested_length                      Length of bytes to write,     */
/*                                            set to 0 to issue ZLP       */
/*    actual_length                         Pointer to save number of     */
/*                                            bytes written               */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_printer_write        Printer class write function  */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  03-08-2023     Yajun Xia                Initial Version 6.2.1         */
/*  10-31-2023     Yajun Xia                Modified comment(s),          */
/*                                            fixed error checking issue, */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT _uxe_device_class_printer_write(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                     ULONG requested_length, ULONG *actual_length)
{

    /* Sanity checks.  */
    if ((printer == UX_NULL) || ((buffer == UX_NULL) && (requested_length > 0)) || (actual_length == UX_NULL))
    {
        return (UX_INVALID_PARAMETER);
    }

    return (_ux_device_class_printer_write(printer, buffer, requested_length, actual_length));
}