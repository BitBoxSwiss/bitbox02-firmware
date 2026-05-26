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
/*    _ux_device_class_ccid_runner_task_run               PORTABLE C      */
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
UINT _ux_device_class_ccid_runner_task_run(UX_DEVICE_CLASS_CCID *ccid)
{

UX_SLAVE_DEVICE                                     *device;
UX_DEVICE_CLASS_CCID_PARAMETER                      *parameter;
UX_DEVICE_CLASS_CCID_HANDLE                         *handles;
UX_DEVICE_CLASS_CCID_HANDLE                         handle;
UX_DEVICE_CLASS_CCID_RUNNER                         *runner;
UX_DEVICE_CLASS_CCID_SLOT                           *slot;
UX_DEVICE_CLASS_CCID_MESSAGE_HEADER                 *cmd;
UX_DEVICE_CLASS_CCID_COMMAND_SETT                   *cmd_sett;
UX_DEVICE_CLASS_CCID_RDR_TO_PC_SLOT_STATUS_HEADER   *rsp;
UX_DEVICE_CLASS_CCID_MESSAGES                       *messages;
ULONG                                               cmd_checks;
UINT                                                status;
INT                                                 immediate_state = UX_TRUE;


    /* Get the pointer to the device.  */
    device =  &_ux_system_slave -> ux_system_slave_device;

    /* Check if the device is configured.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
        return(UX_STATE_EXIT);

    /* Get CCID runner (optimized only 1 runner).  */
    runner = ccid -> ux_device_class_ccid_runners;

    /* Slot to handle (optimized for only one slot).  */
    slot = ccid -> ux_device_class_ccid_slots;

    /* Get CCID parameter.  */
    parameter = &ccid -> ux_device_class_ccid_parameter;

    /* Get CCID messages.  */
    messages = &runner -> ux_device_class_ccid_runner_messages;

    /* Get command and response buffers.  */
    cmd = (UX_DEVICE_CLASS_CCID_MESSAGE_HEADER *)
                            runner -> ux_device_class_ccid_runner_command;
    rsp = (UX_DEVICE_CLASS_CCID_RDR_TO_PC_SLOT_STATUS_HEADER *)
                            runner -> ux_device_class_ccid_runner_response;

    /* Process states.  */
    while(immediate_state)
    {

        /* Check states.  */
        switch(runner -> ux_device_class_ccid_runner_state)
        {
        case UX_DEVICE_CLASS_CCID_RUNNER_IDLE:
            return(UX_STATE_IDLE);

        case UX_DEVICE_CLASS_CCID_RUNNER_START:

            /* Command settings.  */
            cmd_sett  = (UX_DEVICE_CLASS_CCID_COMMAND_SETT *)_ux_device_class_ccid_command_sett;
            cmd_sett += runner -> ux_device_class_ccid_runner_command_index;

            /* Message to pass to application.  */
            messages -> ux_device_class_ccid_messages_pc_to_rdr = (UCHAR *)cmd;
            messages -> ux_device_class_ccid_messages_rdr_to_pc = (UCHAR *)rsp;

            /* Internal checks.  */
            cmd_checks  = (ULONG)cmd_sett -> ux_device_class_ccid_command_sett_flags;
            cmd_checks &= (ULONG)slot -> ux_device_class_ccid_slot_flags;

            /* Check hardware error!  */
            if (cmd_checks & UX_DEVICE_CLASS_CCID_FLAG_HW_ERROR)
            {

                /* Response: (1,1,HW_ERROR).  */
                rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(1, 1);
                rsp -> bError  = UX_DEVICE_CLASS_CCID_HW_ERROR;

                /* Send response.  */
                _ux_device_class_ccid_response(ccid, (UCHAR *)rsp,
                            messages -> ux_device_class_ccid_messages_rdr_to_pc_length);

                /* Runner is idle (status is updated after response sent).  */
                runner -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_IDLE;
                continue;
            }

            /* Check auto sequencing!  */
            if (cmd_checks & UX_DEVICE_CLASS_CCID_FLAG_AUTO_SEQUENCING)
            {

                /* Response: (1,1,BUSY_WITH_AUTO_SEQUENCE).  */
                rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(1, 1);
                rsp -> bError  = UX_DEVICE_CLASS_CCID_BUSY_WITH_AUTO_SEQUENCE;

                /* Send response.  */
                _ux_device_class_ccid_response(ccid, (UCHAR *)rsp,
                            messages -> ux_device_class_ccid_messages_rdr_to_pc_length);

                /* Runner is idle (status is updated after response sent).  */
                runner -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_IDLE;
                continue;
            }

            /* Get command to process, application can fill status.  */
            handles = (UX_DEVICE_CLASS_CCID_HANDLE *)parameter -> ux_device_class_ccid_handles;
            handle = handles[(INT)cmd_sett -> ux_device_class_ccid_command_sett_handle_index];

            /* Initialize response length based on type.  */
            switch(rsp -> bMessageType)
            {
            case UX_DEVICE_CLASS_CCID_RDR_TO_PC_DATA_RATE_AND_CLOCK_FREQ:

                /* Length fixed to 10+8.  */
                messages -> ux_device_class_ccid_messages_rdr_to_pc_length = 18;
                UX_DEVICE_CLASS_CCID_MESSAGE_LENGTH_SET(rsp, 8);
                break;

            case UX_DEVICE_CLASS_CCID_RDR_TO_PC_SLOT_STATUS:

                /* Length fixed to 10.  */
                messages -> ux_device_class_ccid_messages_rdr_to_pc_length =
                                    UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH;
                break;

            default:

                /* There is possible data, set length to max transfer length.  */
                messages -> ux_device_class_ccid_messages_rdr_to_pc_length =
                        parameter -> ux_device_class_ccid_max_transfer_length;
                break;
            }

            /* Save handle that is running.  */
            runner -> ux_device_class_ccid_runner_handle = handle;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_RUNNER_HANDLE:

            /* Run handle (state machine).  */
            status = runner -> ux_device_class_ccid_runner_handle(cmd -> bSlot, messages);

            /* Error cases.  */
            if (status < UX_STATE_NEXT ||
                slot -> ux_device_class_ccid_slot_aborting)
            {

                /* There is no response in this case.  */
                runner -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_IDLE;
                return(UX_STATE_IDLE);
            }

            /* Done case.  */
            if (status == UX_STATE_NEXT)
            {

                /* Next: response start.  */
                runner -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_RSP_START;
                continue;
            }

            /* Waiting.  */
            return(UX_STATE_WAIT);

        case UX_DEVICE_CLASS_CCID_RUNNER_RSP_START:

            /* Wait response idle.  */
            if (ccid -> ux_device_class_ccid_rsp_state != UX_DEVICE_CLASS_CCID_RSP_IDLE)
                return(UX_STATE_WAIT);

            /* Send response.  */
            _ux_device_class_ccid_response(ccid, (UCHAR *)rsp,
                        messages -> ux_device_class_ccid_messages_rdr_to_pc_length);

            /* Runner is idle (status is updated after response sent).  */
            runner -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_IDLE;
            continue;

        default:
            break;
        }

        /* Break the loop.  */
        immediate_state = UX_FALSE;
    }

    /* Unhandled state.  */
    return(UX_STATE_EXIT);
}

#endif
