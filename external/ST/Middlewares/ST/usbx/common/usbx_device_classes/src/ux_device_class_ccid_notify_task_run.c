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
/**   Device CCID Class                                                   */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_ccid.h"
#include "ux_device_stack.h"


#if defined(UX_DEVICE_STANDALONE)


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_ccid_notify_task_run               PORTABLE C      */
/*                                                           6.2.1        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function is the background task of the CCID.                   */
/*                                                                        */
/*    It's for standalone mode.                                           */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    ccid                                  Pointer to CCID class         */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    State machine status                                                */
/*    UX_STATE_EXIT                         Device not configured         */
/*    UX_STATE_IDLE                         No streaming transfer running */
/*    UX_STATE_WAIT                         Streaming transfer running    */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Device Stack                                                   */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  03-08-2023     Chaoqiong Xiao           Initial Version 6.2.1         */
/*                                                                        */
/**************************************************************************/
UINT _ux_device_class_ccid_notify_task_run(UX_DEVICE_CLASS_CCID *ccid)
{
UX_INTERRUPT_SAVE_AREA
UX_SLAVE_DEVICE                                     *device;
UX_DEVICE_CLASS_CCID_SLOT                           *slot;
UX_SLAVE_ENDPOINT                                   *endpoint;
UX_SLAVE_TRANSFER                                   *transfer;
UCHAR                                               *buffer;
ULONG                                               length;
UCHAR                                               icc_mask;
INT                                                 immediate_state = UX_TRUE;
UINT                                                status;


    /* Get the pointer to the device.  */
    device =  &_ux_system_slave -> ux_system_slave_device;

    /* Check if the device is configured.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
        return(UX_STATE_EXIT);

    /* Interrupt IN endpoint get (it's optional so it's OK not found).  */
    endpoint = ccid -> ux_device_class_ccid_endpoint_notify;
    if (endpoint == UX_NULL)
        return(UX_STATE_IDLE);
    transfer = &endpoint -> ux_slave_endpoint_transfer_request;

    /* Process states.  */
    while(immediate_state)
    {

        /* Check states.  */
        switch(ccid -> ux_device_class_ccid_notify_state)
        {

        case UX_DEVICE_CLASS_CCID_NOTIFY_IDLE:
            return(UX_STATE_IDLE);

        case UX_DEVICE_CLASS_CCID_NOTIFY_LOCK:

            /* Wait until status locked.  */
            UX_DISABLE
            if (ccid -> ux_device_class_ccid_flags & UX_DEVICE_CLASS_CCID_FLAG_LOCK)
            {
                UX_RESTORE
                return(UX_STATE_WAIT);
            }
            ccid -> ux_device_class_ccid_flags |= UX_DEVICE_CLASS_CCID_FLAG_LOCK;
            UX_RESTORE

            /* Next: check if there is notification pending.  */
            ccid -> ux_device_class_ccid_notify_state = UX_DEVICE_CLASS_CCID_NOTIFY_START;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_NOTIFY_START:

            /* Check slot notifications (optimized for only one slot).  */
            /* Get slot.  */
            slot = ccid -> ux_device_class_ccid_slots;

            /* Build slot change/hardware error message.  */
            buffer = transfer -> ux_slave_transfer_request_data_pointer;
            length = 0;

            /* By default no message.  */
            buffer[UX_DEVICE_CLASS_CCID_OFFSET_MESSAGE_TYPE] = 0;

            /* Check hardware error notification.  */
            if (slot -> ux_device_class_ccid_slot_flags &
                UX_DEVICE_CLASS_CCID_FLAG_NOTIFY_HW_ERROR)
            {
                slot -> ux_device_class_ccid_slot_flags &=
                                (UCHAR)~UX_DEVICE_CLASS_CCID_FLAG_NOTIFY_HW_ERROR;
                buffer[UX_DEVICE_CLASS_CCID_OFFSET_MESSAGE_TYPE] =
                                UX_DEVICE_CLASS_CCID_RDR_TO_PC_HARDWARE_ERROR;
                buffer[UX_DEVICE_CLASS_CCID_OFFSET_HW_ERROR_SLOT] = 0;
                buffer[UX_DEVICE_CLASS_CCID_OFFSET_HW_ERROR_SEQ] =
                                slot -> ux_device_class_ccid_slot_hw_error_seq;
                buffer[UX_DEVICE_CLASS_CCID_OFFSET_HW_ERROR_CODE] =
                                slot -> ux_device_class_ccid_slot_hw_error;
                length = 4;
            }

            /* Check slot change notification.  */
            else if (slot -> ux_device_class_ccid_slot_flags &
                UX_DEVICE_CLASS_CCID_FLAG_NOTIFY_CHANGE)
            {
                slot -> ux_device_class_ccid_slot_flags &=
                            (UCHAR)~UX_DEVICE_CLASS_CCID_FLAG_NOTIFY_CHANGE;

                /* Message type.  */
                buffer[UX_DEVICE_CLASS_CCID_OFFSET_MESSAGE_TYPE] =
                    UX_DEVICE_CLASS_CCID_RDR_TO_PC_NOTIFY_SLOT_CHANGE;

                /* Slot state bit.  */
                icc_mask = (UCHAR)((slot -> ux_device_class_ccid_slot_icc_status ==
                        UX_DEVICE_CLASS_CCID_SLOT_STATUS_ICC_NOT_PRESENT) ?
                        0u : 1u);

                /* Slot change bit.  */
                icc_mask |= 0x02u;

                /* Save slot state and change.  */
                buffer[UX_DEVICE_CLASS_CCID_OFFSET_SLOT_ICC_STATE] = icc_mask;

                length = 2;
            }

            /* Unlock status.  */
            ccid -> ux_device_class_ccid_flags &= ~UX_DEVICE_CLASS_CCID_FLAG_LOCK;

            /* Check message to see if there is message to send.  */
            if (buffer[UX_DEVICE_CLASS_CCID_OFFSET_MESSAGE_TYPE] == 0)
            {

                /* There is no pending meesage, -> idle.  */
                ccid -> ux_device_class_ccid_notify_state = UX_DEVICE_CLASS_CCID_NOTIFY_IDLE;
                return(UX_STATE_IDLE);
            }

            /* There is message to send.  */
            UX_SLAVE_TRANSFER_STATE_RESET(transfer);
            transfer -> ux_slave_transfer_request_requested_length = length;
            ccid -> ux_device_class_ccid_notify_state = UX_DEVICE_CLASS_CCID_NOTIFY_WAIT;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_NOTIFY_WAIT:
            length = transfer -> ux_slave_transfer_request_requested_length;
            status = _ux_device_stack_transfer_run(transfer, length, length);

            /* Error/success (done) case.  */
            if (status <= UX_STATE_NEXT)
            {

                /* Transfer done, check if there is pending message.  */
                ccid -> ux_device_class_ccid_notify_state = UX_DEVICE_CLASS_CCID_NOTIFY_LOCK;
                return(UX_STATE_WAIT);
            }

            /* Wait transfer.  */
            return(UX_STATE_WAIT);

        default:
            break;
        }


    }

    /* Unhandled state.  */
    return(UX_STATE_EXIT);
}

#endif
