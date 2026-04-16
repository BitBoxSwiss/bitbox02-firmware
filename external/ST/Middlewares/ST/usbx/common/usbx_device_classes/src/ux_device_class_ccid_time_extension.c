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
/*    _ux_device_class_ccid_time_extension                PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function sends bulk IN time_extension of the USB CCID device.  */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    ccid                                  Pointer to ccid instance      */
/*    slot                                  Slot to extend time           */
/*    wt                                    BWT (T=1) or WWT (T=0)        */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  04-25-2022     Chaoqiong Xiao           Initial Version 6.1.11        */
/*  10-31-2023     Yajun Xia                Modified comment(s),          */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT _ux_device_class_ccid_time_extension(UX_DEVICE_CLASS_CCID *ccid, ULONG slot, ULONG wt)
{

UX_SLAVE_ENDPOINT                                   *endpoint;
UX_SLAVE_TRANSFER                                   *transfer;
UX_DEVICE_CLASS_CCID_SLOT                           *ccid_slot;
UX_DEVICE_CLASS_CCID_RUNNER                         *runner;
UCHAR                                               *rsp, *runner_rsp;
UINT                                                status;

    /* Get slot.  */
    ccid_slot  = ccid -> ux_device_class_ccid_slots;
    ccid_slot += slot;

    /* Check slot state.  */
    if ((signed char)ccid_slot -> ux_device_class_ccid_slot_runner < 0)
        return(UX_INVALID_STATE);

    /* Get runner.  */
    runner = ccid -> ux_device_class_ccid_runners;
    runner += ccid_slot -> ux_device_class_ccid_slot_runner;

    /* Get bulk IN endpoint.  */
    endpoint = ccid -> ux_device_class_ccid_endpoint_in;

    /* Get transfer request.  */
    transfer = &endpoint -> ux_slave_endpoint_transfer_request;

    /* Lock bulk IN.  */
    _ux_device_mutex_on(&ccid -> ux_device_class_ccid_response_mutex);

    /* Get response buffer.  */
    rsp = transfer -> ux_slave_transfer_request_data_pointer;
    runner_rsp = runner -> ux_device_class_ccid_runner_response;

    /* Fill response.  */
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_MESSAGE_TYPE]       = runner_rsp[UX_DEVICE_CLASS_CCID_OFFSET_MESSAGE_TYPE];
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_LENGTH]             = 0;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_LENGTH+1]           = 0;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_LENGTH+2]           = 0;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_LENGTH+3]           = 0;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_SLOT]               = runner_rsp[UX_DEVICE_CLASS_CCID_OFFSET_SLOT];
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_SEQ]                = runner_rsp[UX_DEVICE_CLASS_CCID_OFFSET_SEQ];
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_STATUS]             = runner_rsp[UX_DEVICE_CLASS_CCID_OFFSET_STATUS] |
                                                          UX_DEVICE_CLASS_CCID_SLOT_STATUS_TIME_EXTENSION;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_ERROR]              = (UCHAR)wt;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_CHAIN_PARAMETER]    = 0;
    rsp[UX_DEVICE_CLASS_CCID_OFFSET_CHAIN_PARAMETER+1]  = 0;

    /* Transfer data.  */
    status = _ux_device_stack_transfer_request(transfer,
                                UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH,
                                UX_DEVICE_CLASS_CCID_MESSAGE_HEADER_LENGTH);

    /* Unlock bulk IN.  */
    _ux_device_mutex_off(&ccid -> ux_device_class_ccid_response_mutex);

    /* Return transfer status.  */
    return(status);
}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_ccid_icc_insert                   PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in CCID device bulk IN time_extension   */
/*    send function.                                                      */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    ccid                                  Pointer to ccid instance      */
/*    slot                                  Slot to extend time           */
/*    wt                                    BWT (T=1) or WWT (T=0)        */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_ccid_time_extension  CCID device bulk IN           */
/*                                          time_extension send function. */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Yajun Xia                Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
UINT _uxe_device_class_ccid_time_extension(UX_DEVICE_CLASS_CCID *ccid, ULONG slot, ULONG wt)
{

    /* Sanity checks.  */
    if ((ccid == UX_NULL) || (slot >= ccid -> ux_device_class_ccid_parameter.ux_device_class_ccid_max_n_slots))
        return(UX_INVALID_PARAMETER);

    /* Call actual function.  */
    return(_ux_device_class_ccid_time_extension(ccid, slot, wt));
}