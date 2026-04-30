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
/*    _ux_device_class_ccid_icc_insert                    PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function indicates card insertion of the USB CCID device.      */
/*                                                                        */
/*    Note if seq_start is TRUE, application must invoke _auto_seq_done   */
/*    later to indicate the sequence end, with final card status.         */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    ccid                                  Pointer to ccid instance      */
/*    slot                                  Slot inserted                 */
/*    seq_start                             Auto activation sequence on   */
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
/*  03-08-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.2.1  */
/*  10-31-2023     Yajun Xia                Modified comment(s),          */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT _ux_device_class_ccid_icc_insert(UX_DEVICE_CLASS_CCID *ccid, ULONG slot, ULONG seq_start)
{

UX_DEVICE_CLASS_CCID_SLOT       *ccid_slot;

    /* Sanity check.  */
    if (slot >= ccid -> ux_device_class_ccid_parameter.ux_device_class_ccid_max_n_slots)
        return(UX_INVALID_PARAMETER);

    /* Get slot instance.  */
    ccid_slot  = ccid -> ux_device_class_ccid_slots;
    ccid_slot += slot;

    /* Lock states.  */
    _ux_device_class_ccid_lock(ccid);

    /* Return success if already card inserted.  */
    if (ccid_slot -> ux_device_class_ccid_slot_icc_status != UX_DEVICE_CLASS_CCID_ICC_NOT_PRESENT)
    {
        _ux_device_class_ccid_unlock(ccid);
        return(UX_SUCCESS);
    }

    /* Update card status (INACTIVE).  */
    ccid_slot -> ux_device_class_ccid_slot_icc_status = UX_DEVICE_CLASS_CCID_SLOT_STATUS_ICC_INACTIVE;

    /* Auto sequencing started?  */
    if (seq_start)
        ccid_slot -> ux_device_class_ccid_slot_flags |= UX_DEVICE_CLASS_CCID_FLAG_AUTO_SEQUENCING;

    /* Notify if interrupt endpoint exists.  */
    if (ccid -> ux_device_class_ccid_endpoint_notify)
    {
        ccid_slot -> ux_device_class_ccid_slot_flags |= UX_DEVICE_CLASS_CCID_FLAG_NOTIFY_CHANGE;

        /* Unlock states.  */
        _ux_device_class_ccid_unlock(ccid);

        /* Wakeup interrupt notification.  */
        _ux_device_semaphore_put(&ccid -> ux_device_class_ccid_notify_semaphore);

#if defined(UX_DEVICE_STANDALONE)
        if (ccid -> ux_device_class_ccid_notify_state == UX_DEVICE_CLASS_CCID_NOTIFY_IDLE)
            ccid -> ux_device_class_ccid_notify_state = UX_DEVICE_CLASS_CCID_NOTIFY_LOCK;
#endif
        return(UX_SUCCESS);
    }

    /* Unlock states.  */
    _ux_device_class_ccid_unlock(ccid);

    /* Return transfer status.  */
    return(UX_SUCCESS);
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
/*    This function checks errors in CCID card insertion function.        */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    ccid                                  Pointer to ccid instance      */
/*    slot                                  Slot inserted                 */
/*    seq_start                             Auto activation sequence on   */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_ccid_icc_insert      CCID card insertion function. */
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
UINT _uxe_device_class_ccid_icc_insert(UX_DEVICE_CLASS_CCID *ccid, ULONG slot, ULONG seq_start)
{

    /* Sanity checks.  */
    if (ccid == UX_NULL)
        return(UX_INVALID_PARAMETER);

    return(_ux_device_class_ccid_icc_insert(ccid, slot, seq_start));
}