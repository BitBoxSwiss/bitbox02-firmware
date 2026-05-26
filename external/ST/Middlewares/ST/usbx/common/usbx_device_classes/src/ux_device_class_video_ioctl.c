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
/**   Device Video Class                                                  */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_video.h"
#include "ux_device_stack.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_video_ioctl                        PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function performs certain functions on the video instance      */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    video                                 Address of video class        */
/*                                              instance                  */
/*    ioctl_function                        IOCTL function code           */
/*    parameter                             Parameter for function        */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Status                                                              */
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
UINT _ux_device_class_video_ioctl(UX_DEVICE_CLASS_VIDEO *video, ULONG ioctl_function,
                                    VOID *parameter)
{

UINT                                                status;
VOID                                                **pptr_parameter;


    /* Let's be optimist ! */
    status = UX_SUCCESS;

    /* The command request will tell us what we need to do here.  */
    switch (ioctl_function)
    {

        case UX_DEVICE_CLASS_VIDEO_IOCTL_GET_ARG:

            /* Properly cast the parameter pointer.  */
            pptr_parameter = (VOID **) parameter;

            /* Save argument.  */
            *pptr_parameter = video -> ux_device_class_video_callbacks.ux_device_class_video_arg;

            break;

        default:

            /* Function not supported. Return an error.  */
            status =  UX_FUNCTION_NOT_SUPPORTED;
    }

    /* Return status to caller.  */
    return(status);

}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_video_ioctl                       PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in video IOCTL function call.           */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    video                                 Address of video class        */
/*                                              instance                  */
/*    ioctl_function                        IOCTL function code           */
/*    parameter                             Parameter for function        */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Status                                                              */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_video_ioctl          Video IOCTL                   */
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
UINT _uxe_device_class_video_ioctl(UX_DEVICE_CLASS_VIDEO *video, ULONG ioctl_function,
                                    VOID *parameter)
{

    /* Sanity check. */
    if (video == UX_NULL)
        return(UX_INVALID_PARAMETER);

    /* Call the actual video IOCTL function.  */
    return(_ux_device_class_video_ioctl(video, ioctl_function, parameter));
}