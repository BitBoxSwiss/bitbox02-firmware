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
/*    _ux_device_class_video_write_payload_get            PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function get a writable payload buffer in the Video class.     */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    stream                                Address of video stream       */
/*                                            instance                    */
/*    payload                               Pointer to buffer to save     */
/*                                            payload buffer pointer      */
/*    length                                Pointer to save payload       */
/*                                            buffer length in bytes      */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
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
UINT _ux_device_class_video_write_payload_get(UX_DEVICE_CLASS_VIDEO_STREAM *stream, UCHAR **payload, ULONG *length)
{

UX_SLAVE_ENDPOINT           *endpoint;
UX_SLAVE_DEVICE             *device;


    /* Get the pointer to the device.  */
    device =  &_ux_system_slave -> ux_system_slave_device;

    /* As long as the device is in the CONFIGURED state.  */
    if (device -> ux_slave_device_state != UX_DEVICE_CONFIGURED)
    {

        /* Cannot proceed with command, the interface is down.  */
        return(UX_CONFIGURATION_HANDLE_UNKNOWN);
    }

    /* Check if endpoint is available.  */
    endpoint = stream -> ux_device_class_video_stream_endpoint;
    if (endpoint == UX_NULL)
        return(UX_ERROR);

    /* Check if endpoint direction is OK.  */
    if ((endpoint -> ux_slave_endpoint_descriptor.bEndpointAddress & UX_ENDPOINT_DIRECTION) == UX_ENDPOINT_OUT)
        return(UX_ERROR);

    /* Check overflow!!  */
    if (stream -> ux_device_class_video_stream_access_pos == stream -> ux_device_class_video_stream_transfer_pos &&
        stream -> ux_device_class_video_stream_access_pos -> ux_device_class_video_payload_length != 0)
        return(UX_BUFFER_OVERFLOW);

    /* Store payload buffer pointer.  */
    *payload = stream -> ux_device_class_video_stream_access_pos -> ux_device_class_video_payload_data;

    /* Exclude header size in payload buffer size.  */
    *length = (stream -> ux_device_class_video_stream_payload_buffer_size - 4);

    return(UX_SUCCESS);
}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_video_write_payload_get           PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in video write payload get function     */
/*    call.                                                               */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    stream                                Address of video stream       */
/*                                            instance                    */
/*    payload                               Pointer to buffer to save     */
/*                                            payload buffer pointer      */
/*    length                                Pointer to save payload       */
/*                                            buffer length in bytes      */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_video_write_payload_get                            */
/*                                          Video write payload get       */
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
UINT _uxe_device_class_video_write_payload_get(UX_DEVICE_CLASS_VIDEO_STREAM *stream, UCHAR **payload, ULONG *length)
{

    /* Sanity check. */
    if ((stream == UX_NULL) || (payload == UX_NULL) || (length == UX_NULL))
        return(UX_INVALID_PARAMETER);

    /* Call the actual video write payload get function.  */
    return(_ux_device_class_video_write_payload_get(stream, payload, length));
}