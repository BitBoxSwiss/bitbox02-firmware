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

static inline UINT _ux_device_class_ccid_cmd_task(UX_DEVICE_CLASS_CCID *ccid);
static inline UINT _ux_device_class_ccid_rsp_task(UX_DEVICE_CLASS_CCID *ccid);


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_ccid_tasks_run                     PORTABLE C      */
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
/*    instance                              Pointer to CCID class         */
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
/*    ux_device_class_ccid_notify_task_run Run interrupt notify task      */
/*    ux_device_class_ccid_runner_task_run Run slot command runner task   */
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
UINT _ux_device_class_ccid_tasks_run(VOID *instance)
{
UX_SLAVE_DEVICE                 *device;
UX_DEVICE_CLASS_CCID            *ccid;
ULONG                           run_count = 0;


    /* Get ccid instance.  */
    ccid = (UX_DEVICE_CLASS_CCID *) instance;

    /* Get the pointer to the device.  */
    device =  &_ux_system_slave -> ux_system_slave_device;

    /* Check if the device is configured.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
        return(UX_STATE_EXIT);

    /* Bulk OUT command process.  */
    if (_ux_device_class_ccid_cmd_task(ccid) != UX_STATE_IDLE)
        run_count ++;

    /* Bulk IN response process.  */
    if (_ux_device_class_ccid_rsp_task(ccid) != UX_STATE_IDLE)
        run_count ++;

    /* Interrupt IN notification process.  */
    if (_ux_device_class_ccid_notify_task_run(ccid) != UX_STATE_IDLE)
        run_count ++;

    /* Runner tasks process.  */
    if (_ux_device_class_ccid_runner_task_run(ccid) != UX_STATE_IDLE)
        run_count ++;

    return((run_count > 0) ? UX_STATE_WAIT : UX_STATE_IDLE);
}
static inline UINT _ux_device_class_ccid_cmd_task(UX_DEVICE_CLASS_CCID *ccid)
{
UX_INTERRUPT_SAVE_AREA
INT                                                 immediate_state = UX_TRUE;
UINT                                                status;
UX_DEVICE_CLASS_CCID_SLOT                           *slot;
UX_DEVICE_CLASS_CCID_RUNNER                         *runner = UX_NULL;
UX_DEVICE_CLASS_CCID_PARAMETER                      *parameter;
UX_DEVICE_CLASS_CCID_MESSAGES                       messages;
UX_SLAVE_ENDPOINT                                   *endpoint;
UX_SLAVE_TRANSFER                                   *transfer_cmd;
UX_DEVICE_CLASS_CCID_MESSAGE_HEADER                 *cmd;
UX_DEVICE_CLASS_CCID_RDR_TO_PC_SLOT_STATUS_HEADER   *rsp;
UX_DEVICE_CLASS_CCID_COMMAND_SETT                   *cmd_sett;
CHAR                                                cmd_index;
UX_DEVICE_CLASS_CCID_HANDLE                         *handles;


    /* Check endpoint.  */
    endpoint = ccid -> ux_device_class_ccid_endpoint_out;
    if (endpoint == UX_NULL)
    {
        ccid -> ux_device_class_ccid_cmd_state = UX_STATE_RESET;
        return(UX_STATE_EXIT);
    }
    transfer_cmd = &endpoint -> ux_slave_endpoint_transfer_request;

    /* Get running settings.  */
    parameter = &ccid -> ux_device_class_ccid_parameter;

    /* Wait Bulk-OUT command.  */
    while(immediate_state)
    {

        /* Process state.  */
        switch(ccid -> ux_device_class_ccid_cmd_state)
        {
        case UX_DEVICE_CLASS_CCID_CMD_START:

            /* Prepare transfer, next: wait.  */
            ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_WAIT;
            UX_SLAVE_TRANSFER_STATE_RESET(transfer_cmd);

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_CMD_WAIT:
            status = _ux_device_stack_transfer_run(transfer_cmd,
                    parameter -> ux_device_class_ccid_max_transfer_length,
                    parameter -> ux_device_class_ccid_max_transfer_length);

            /* Error case.  */
            if (status < UX_STATE_NEXT)
            {

                ccid -> ux_device_class_ccid_cmd_state = UX_STATE_RESET;
                return(UX_STATE_ERROR);
            }

            /* Success case.  */
            if (status == UX_STATE_NEXT)
            {

                /* Check transfer results.  */
                if ((transfer_cmd -> ux_slave_transfer_request_completion_code != UX_SUCCESS) ||
                    (transfer_cmd -> ux_slave_transfer_request_actual_length <
                        UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH) ||
                    (transfer_cmd -> ux_slave_transfer_request_actual_length >
                        parameter -> ux_device_class_ccid_max_transfer_length))
                {

                    ccid -> ux_device_class_ccid_cmd_state = UX_STATE_RESET;
                    return(UX_STATE_ERROR);
                }

                /* Access to CCID command message header.  */
                cmd = (UX_DEVICE_CLASS_CCID_MESSAGE_HEADER *)
                                transfer_cmd -> ux_slave_transfer_request_data_pointer;

                /* Get command setting index.  */
                cmd_sett = (UX_DEVICE_CLASS_CCID_COMMAND_SETT *)_ux_device_class_ccid_command_sett;
                for (cmd_index = 0; cmd_index < UX_DEVICE_CLASS_CCID_N_COMMANDS;)
                {
                    if (cmd -> bMessageType ==
                        cmd_sett -> ux_device_class_ccid_command_sett_command_type)
                        break;

                    /* Next command setting.  */
                    cmd_sett ++;
                    cmd_index ++;
                }

                /* Save command index for further actions.  */
                ccid -> ux_device_class_ccid_cmd_index = cmd_index;

                /* Next: lock and update status.  */
                ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_LOCK;
                continue;
            }

            /* Wait.  */
            return(UX_STATE_WAIT);

        case UX_DEVICE_CLASS_CCID_CMD_LOCK:
            UX_DISABLE
            if (ccid -> ux_device_class_ccid_flags & UX_DEVICE_CLASS_CCID_FLAG_LOCK)
            {
                UX_RESTORE
                return(UX_STATE_WAIT);
            }
            ccid -> ux_device_class_ccid_flags |= UX_DEVICE_CLASS_CCID_FLAG_LOCK;
            UX_RESTORE

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_CMD_PROCESS:
            cmd = (UX_DEVICE_CLASS_CCID_MESSAGE_HEADER *)
                            transfer_cmd -> ux_slave_transfer_request_data_pointer;
            cmd_index = ccid -> ux_device_class_ccid_cmd_index;
            cmd_sett = (UX_DEVICE_CLASS_CCID_COMMAND_SETT *)&_ux_device_class_ccid_command_sett[(INT)cmd_index];
            handles = (UX_DEVICE_CLASS_CCID_HANDLE *)parameter -> ux_device_class_ccid_handles;

            /* Initialize response.  */
            rsp = (UX_DEVICE_CLASS_CCID_RDR_TO_PC_SLOT_STATUS_HEADER *)
                                                ccid -> ux_device_class_ccid_header;
            _ux_utility_memory_set(rsp, 0, UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH); /* Use case of memset is verified. */
            rsp -> bMessageType = cmd_sett -> ux_device_class_ccid_command_sett_response_type;
            rsp -> bSlot        = cmd -> bSlot;
            rsp -> bSeq         = cmd -> bSeq;

            /* Check command support (0,1,0).  */
            if (rsp -> bMessageType == 0 ||
                handles[(INT)cmd_sett -> ux_device_class_ccid_command_sett_handle_index] == UX_NULL)
            {

                /* Response: command not supported (0,1,0).  */
                rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(0, 1);
                _ux_device_class_ccid_unlock(ccid);

                /* Next: response.  */
                ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_RSP_START;
                return(UX_STATE_IDLE);
            }

            /* check Slot exist (2,1,5).  */
            if (cmd -> bSlot >= parameter -> ux_device_class_ccid_max_n_slots)
            {

                /* Response: Slot not exist.  */
                rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(2, 1);
                rsp -> bError  = 5;
                _ux_device_class_ccid_unlock(ccid);

                /* Next: response.  */
                ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_RSP_START;
                return(UX_STATE_IDLE);
            }

            /* Get slot instance for later usage (optimized only 1 slot).  */
            slot = ccid -> ux_device_class_ccid_slots;

            /* Initialize response status from slot status.  */
            rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(
                                slot -> ux_device_class_ccid_slot_icc_status, 0);

            /* Initialize response clock status.  */
            if (cmd_sett -> ux_device_class_ccid_command_sett_response_type == 0x81)
                rsp -> bClockStatus = slot -> ux_device_class_ccid_slot_clock_status;

            /* Abort command
                - return slot status(OK) anyway
                - clear aborting status
               Aborting
                - return slot status(ABORTED)
               Other command (except SetDataRateAndClockFrequency)
                - Check busy  */

            /* Abort command is handled differently.  */
            if (cmd -> bMessageType != UX_DEVICE_CLASS_CCID_PC_TO_RDR_ABORT ||
                !slot -> ux_device_class_ccid_slot_aborting)
            {

                /* Check if slot is idle (optimized one slot).  */
                runner = ccid -> ux_device_class_ccid_runners;
                if (ccid -> ux_device_class_ccid_n_busy == 0)
                {

                    /* It's not possible no runner found here, just execute runner.  */

                    /* Runner is busy now.  */
                    runner -> ux_device_class_ccid_runner_slot = (CHAR)cmd->bSlot;
                    runner -> ux_device_class_ccid_runner_command_index = cmd_index;
                    ccid -> ux_device_class_ccid_n_busy ++;

                    /* Create a copy of command and response header.  */
                    _ux_utility_memory_copy(runner -> ux_device_class_ccid_runner_command,
                                            cmd,
                                            transfer_cmd -> ux_slave_transfer_request_actual_length); /* Use case of memcpy is verified. */
                    _ux_utility_memory_copy(runner -> ux_device_class_ccid_runner_response,
                                            rsp, UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH); /* Use case of memcpy is verified. */

                    /* Pre-process of command done.  */
                    _ux_device_class_ccid_unlock(ccid);

                    /* Update runner state to start.  */
                    runner -> ux_device_class_ccid_runner_state = UX_DEVICE_CLASS_CCID_RUNNER_START;

                    /* Command processed, command is allowed when runner is executing.  */
                    ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_START;
                    return(UX_STATE_WAIT);
                }

                /* Response: Slot Status(busy), optimized for 1 slot.  */
                rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(
                                slot -> ux_device_class_ccid_slot_icc_status, 1);
                rsp -> bError = UX_DEVICE_CLASS_CCID_CMD_SLOT_BUSY;
                _ux_device_class_ccid_unlock(ccid);

                /* Next: response (status busy).  */
                ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_RSP_START;
                continue;
            }

            /* We are here when we see Abort command, or aborting.
                - Abort command : slot status (ok/fail)
                - Aborting : slot status (CMD_ABORTED)
            */

            /* Abort command.  */
            if (cmd -> bMessageType == UX_DEVICE_CLASS_CCID_PC_TO_RDR_ABORT)
            {

                /* Check sequence.  */
                if (cmd -> bSeq != slot -> ux_device_class_ccid_slot_aborting_seq)
                {

                    /* Response: sequence error.  */
                    rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(
                                    slot -> ux_device_class_ccid_slot_icc_status, 1);
                    rsp -> bError  = 6;
                }
                else
                {

                    /* Aborting.  */
                    if (slot -> ux_device_class_ccid_slot_aborting)
                    {

                        /* Call abort handle.  */
                        messages.ux_device_class_ccid_messages_pc_to_rdr = (VOID *)cmd;
                        messages.ux_device_class_ccid_messages_rdr_to_pc = (VOID *)rsp;
                        messages.ux_device_class_ccid_messages_rdr_to_pc_length = 0;
                        parameter -> ux_device_class_ccid_handles ->
                                ux_device_class_ccid_handles_abort(cmd -> bSlot, &messages);

                        /* Status(OK)  */
                        rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(
                                        slot -> ux_device_class_ccid_slot_icc_status, 0);

                        /* Free runner (optimized only 1 slot).  */
                        runner = ccid -> ux_device_class_ccid_runners;
                        runner -> ux_device_class_ccid_runner_slot = -1;
                        ccid -> ux_device_class_ccid_n_busy --;

                        /* Clear slot busy and aborting.  */
                        slot -> ux_device_class_ccid_slot_runner = -1;
                        slot -> ux_device_class_ccid_slot_aborting = UX_FALSE;
                    }
                    else
                    {

                        /* Status(CMD_NOT_ABORTED)?  */
                        rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(
                                        slot -> ux_device_class_ccid_slot_icc_status, 1);
                        rsp -> bError  = UX_DEVICE_CLASS_CCID_CMD_SLOT_BUSY;
                    }
                }

                _ux_device_class_ccid_unlock(ccid);

                /* Next: response.  */
                ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_RSP_START;
                return(UX_STATE_IDLE);
            }

            /* Aborting.  */

            /* Response: Slot Status(aborted).  */
            rsp -> bStatus = UX_DEVICE_CLASS_CCID_SLOT_STATUS(
                                slot -> ux_device_class_ccid_slot_icc_status, 1);
            rsp -> bError = UX_DEVICE_CLASS_CCID_CMD_ABORTED;

            _ux_device_class_ccid_unlock(ccid);

            /* Next: response.  */
            ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_RSP_START;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_CMD_RSP_START:

            /* Wait until rsponse task is idle.  */
            if (ccid -> ux_device_class_ccid_rsp_state != UX_DEVICE_CLASS_CCID_RSP_IDLE)
                return(UX_STATE_WAIT);

            /* Start response.  */
            _ux_device_class_ccid_response(ccid,
                                    ccid -> ux_device_class_ccid_header,
                                    UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH);

            /* Command is idle and started after response sent.  */
            ccid -> ux_device_class_ccid_flags |= UX_DEVICE_CLASS_CCID_FLAG_CMD_RSP;
            ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_IDLE;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_CMD_IDLE:
            return(UX_STATE_IDLE);

        default:
            break;
        }

        /* Break the loop.  */
        immediate_state = UX_FALSE;
    }

    return(UX_STATE_WAIT);
}
static inline UINT _ux_device_class_ccid_rsp_task(UX_DEVICE_CLASS_CCID *ccid)
{
UX_INTERRUPT_SAVE_AREA
INT                                                 immediate_state = UX_TRUE;
UINT                                                status;
UX_SLAVE_ENDPOINT                                   *endpoint;
UX_SLAVE_TRANSFER                                   *transfer_rsp;
ULONG                                               length;

    /* Check endpoint.  */
    endpoint = ccid -> ux_device_class_ccid_endpoint_in;
    if (endpoint == UX_NULL)
    {
        ccid -> ux_device_class_ccid_cmd_state = UX_STATE_RESET;
        return(UX_STATE_EXIT);
    }
    transfer_rsp = &endpoint -> ux_slave_endpoint_transfer_request;

    while(immediate_state)
    {

        switch(ccid -> ux_device_class_ccid_rsp_state)
        {

        case UX_DEVICE_CLASS_CCID_RSP_IDLE:
            return(UX_STATE_IDLE);

        case UX_DEVICE_CLASS_CCID_RSP_START:
            ccid -> ux_device_class_ccid_rsp_state = UX_DEVICE_CLASS_CCID_RSP_WAIT;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_RSP_WAIT:

            /* Run bulk IN transfer.  */
            length = transfer_rsp -> ux_slave_transfer_request_requested_length;
            status = _ux_device_stack_transfer_run(transfer_rsp, length, length);

            /* Error/success case.  */
            if (status <= UX_STATE_NEXT)
            {

                /* Transfer is done.  */

                /* Check if it's command response or runner response.
                   After runner response status needs update (optimized 1 slot).  */
                if (ccid -> ux_device_class_ccid_flags & UX_DEVICE_CLASS_CCID_FLAG_CMD_RSP)
                {
                    ccid -> ux_device_class_ccid_flags &= ~UX_DEVICE_CLASS_CCID_FLAG_CMD_RSP;

                    /* CMD -RSP: done.  */
                    ccid -> ux_device_class_ccid_rsp_state = UX_DEVICE_CLASS_CCID_RSP_DONE;
                    continue;
                }

                /* CMD - RUNNER - RSP : Update status.  */
                ccid -> ux_device_class_ccid_rsp_state = UX_DEVICE_CLASS_CCID_RSP_LOCK;
                continue;

            }

            /* Wait transfer.  */
            return(UX_STATE_WAIT);

        case UX_DEVICE_CLASS_CCID_RSP_LOCK:
            UX_DISABLE
            if (ccid -> ux_device_class_ccid_flags & UX_DEVICE_CLASS_CCID_FLAG_LOCK)
            {
                UX_RESTORE
                return(UX_STATE_WAIT);
            }
            ccid -> ux_device_class_ccid_flags |= UX_DEVICE_CLASS_CCID_FLAG_LOCK;
            UX_RESTORE

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_RSP_UPDATE:

            /* Free runner and clear busy slot (optimized 1 slot).  */
            if (ccid -> ux_device_class_ccid_n_busy > 0)
                ccid -> ux_device_class_ccid_n_busy --;

            /* Unlock.  */
            ccid -> ux_device_class_ccid_flags &= ~UX_DEVICE_CLASS_CCID_FLAG_LOCK;

            /* Fall through.  */
        case UX_DEVICE_CLASS_CCID_RSP_DONE:

            /* Start command.  */
            if (ccid -> ux_device_class_ccid_cmd_state == UX_DEVICE_CLASS_CCID_CMD_IDLE)
                ccid -> ux_device_class_ccid_cmd_state = UX_DEVICE_CLASS_CCID_CMD_START;

            /* Next: idle.  */
            ccid -> ux_device_class_ccid_rsp_state = UX_DEVICE_CLASS_CCID_RSP_IDLE;
            return(UX_STATE_IDLE);

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