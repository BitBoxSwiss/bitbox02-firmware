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
/**   Device CCID Class                                                   */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_ccid.h"
#include "ux_device_stack.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_ccid_activate                      PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function activates the USB CCID device.                        */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    command                               Pointer to ccid command       */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Device CCID                                                    */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  04-25-2022     Chaoqiong Xiao           Initial Version 6.1.11        */
/*  07-29-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            fixed parameter/variable    */
/*                                            names conflict C++ keyword, */
/*                                            resulting in version 6.1.12 */
/*  03-08-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.2.1  */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added a new mode to manage  */
/*                                            endpoint buffer in classes, */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT  _ux_device_class_ccid_activate(UX_SLAVE_CLASS_COMMAND *command)
{

UX_SLAVE_INTERFACE                      *ccid_interface;
UX_SLAVE_CLASS                          *ccid_class;
UX_DEVICE_CLASS_CCID                    *ccid;
UX_SLAVE_ENDPOINT                       *endpoint;
ULONG                                   endpoint_type;
#if !defined(UX_DEVICE_STANDALONE)
UINT                                    i;
#endif

    /* Get the class container.  */
    ccid_class =  command -> ux_slave_class_command_class_ptr;

    /* Get the class instance in the container.  */
    ccid = (UX_DEVICE_CLASS_CCID *) ccid_class -> ux_slave_class_instance;

    /* Get the interface that owns this instance.  */
    ccid_interface =  (UX_SLAVE_INTERFACE  *) command -> ux_slave_class_command_interface;

    /* Store the class instance into the interface.  */
    ccid_interface -> ux_slave_interface_class_instance =  (VOID *)ccid;

    /* Now the opposite, store the interface in the class instance.  */
    ccid -> ux_device_class_ccid_interface =  ccid_interface;

    /* Save endpoints.  */
    ccid -> ux_device_class_ccid_endpoint_notify = UX_NULL;
    endpoint = ccid_interface -> ux_slave_interface_first_endpoint;
    while(endpoint)
    {
        endpoint_type = endpoint -> ux_slave_endpoint_descriptor.bmAttributes;
        endpoint_type &= UX_MASK_ENDPOINT_TYPE;
        if (endpoint_type == UX_INTERRUPT_ENDPOINT)
        {
            ccid -> ux_device_class_ccid_endpoint_notify = endpoint;
#if UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1
            endpoint -> ux_slave_endpoint_transfer_request.
                ux_slave_transfer_request_data_pointer =
                                UX_DEVICE_CLASS_CCID_INTERRUPTIN_BUFFER(ccid);
#endif
        }
        if (endpoint_type == UX_BULK_ENDPOINT)
        {
            if (endpoint -> ux_slave_endpoint_descriptor.bEndpointAddress & UX_ENDPOINT_IN)
            {
                ccid -> ux_device_class_ccid_endpoint_in = endpoint;
#if UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1
                endpoint -> ux_slave_endpoint_transfer_request.
                    ux_slave_transfer_request_data_pointer =
                                UX_DEVICE_CLASS_CCID_BULKIN_BUFFER(ccid);
#endif
            }
            else
            {
                ccid -> ux_device_class_ccid_endpoint_out = endpoint;
#if UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1
                endpoint -> ux_slave_endpoint_transfer_request.
                    ux_slave_transfer_request_data_pointer =
                                UX_DEVICE_CLASS_CCID_BULKOUT_BUFFER(ccid);
#endif
            }
        }
        endpoint = endpoint -> ux_slave_endpoint_next_endpoint;
    }

#if defined(UX_DEVICE_STANDALONE)

    /* Initialize slots (optimized for 1 slot).  */
    ccid -> ux_device_class_ccid_slots -> ux_device_class_ccid_slot_runner = -1;
    ccid -> ux_device_class_ccid_slots -> ux_device_class_ccid_slot_icc_status =
                                    UX_DEVICE_CLASS_CCID_ICC_NOT_PRESENT;

    /* Initialize task states.  */
    ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_START;
    ccid -> ux_device_class_ccid_rsp_state = UX_DEVICE_CLASS_CCID_RSP_IDLE;
    ccid -> ux_device_class_ccid_notify_state = UX_DEVICE_CLASS_CCID_NOTIFY_IDLE;

    /* Initialize runner task state (optimized for 1 slot).  */
    ccid -> ux_device_class_ccid_runners -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_IDLE;
#else

    /* Initialize slots.  */
    for (i = 0;
        i < ccid -> ux_device_class_ccid_parameter.ux_device_class_ccid_max_n_slots;
        i ++)
    {
        ccid -> ux_device_class_ccid_slots[i].ux_device_class_ccid_slot_runner = -1;
        ccid -> ux_device_class_ccid_slots[i].ux_device_class_ccid_slot_icc_status =
                                        UX_DEVICE_CLASS_CCID_ICC_NOT_PRESENT;
    }

    /* Activate thread for Bulk-OUT command messages.  */
    _ux_device_thread_resume(&ccid -> ux_device_class_ccid_thread);

    /* Activate thread for Interrupt-IN notification messages.  */
    _ux_device_thread_resume(&ccid -> ux_device_class_ccid_notify_thread);

    /* Activate threads for runners.  */
    for (i = 0;
        i < ccid -> ux_device_class_ccid_parameter.ux_device_class_ccid_max_n_busy_slots;
        i ++)
    {
        _ux_device_thread_resume(&ccid -> ux_device_class_ccid_runners[i].
                                           ux_device_class_ccid_runner_thread);
    }
#endif

    /* If there is a activate function call it.  */
    if (ccid -> ux_device_class_ccid_parameter.ux_device_class_ccid_instance_activate != UX_NULL)
    {

        /* Invoke the application callback.  */
        ccid -> ux_device_class_ccid_parameter.ux_device_class_ccid_instance_activate(ccid);
    }

    /* If trace is enabled, insert this event into the trace buffer.  */
    UX_TRACE_IN_LINE_INSERT(UX_TRACE_DEVICE_CLASS_CCID_ACTIVATE, ccid, 0, 0, 0, UX_TRACE_DEVICE_CLASS_EVENTS, 0, 0)

    /* If trace is enabled, register this object.  */
    UX_TRACE_OBJECT_REGISTER(UX_TRACE_DEVICE_OBJECT_TYPE_INTERFACE, ccid, 0, 0, 0)

    /* Return completion status.  */
    return(UX_SUCCESS);
}
