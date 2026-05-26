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
/**   Device Audio Class                                                  */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_audio.h"
#include "ux_device_stack.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_audio_stream_get                   PORTABLE C      */
/*                                                           6.2.1        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function get the stream instance of Audio class.               */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    audio                                 Address of audio class        */
/*                                            instance                    */
/*    stream_index                          Stream instance index 0 based */
/*    stream                                Pointer to buffer to fill     */
/*                                            pointer to stream instance  */
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
/*  05-19-2020     Chaoqiong Xiao           Initial Version 6.0           */
/*  09-30-2020     Chaoqiong Xiao           Modified comment(s),          */
/*                                            resulting in version 6.1    */
/*  01-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            resulting in version 6.1.10 */
/*  03-08-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added error checks support, */
/*                                            resulting in version 6.2.1  */
/*                                                                        */
/**************************************************************************/
UINT    _ux_device_class_audio_stream_get(UX_DEVICE_CLASS_AUDIO *audio,
        ULONG stream_index, UX_DEVICE_CLASS_AUDIO_STREAM **stream)
{

    /* Store the stream instance found.  */
    *stream = audio -> ux_device_class_audio_streams + stream_index;

    /* Return completion status.  */
    return(UX_SUCCESS);
}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_audio_stream_get                  PORTABLE C      */
/*                                                           6.2.1        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in stream instance getting function.    */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    audio                                 Address of audio class        */
/*                                            instance                    */
/*    stream_index                          Stream instance index 0 based */
/*    stream                                Pointer to buffer to fill     */
/*                                            pointer to stream instance  */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_audio_stream_get     Get stream instance           */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Application                                                         */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  03-08-2023     Chaoqiong Xiao           Initial Version 6.2.1         */
/*                                                                        */
/**************************************************************************/
UINT    _uxe_device_class_audio_stream_get(UX_DEVICE_CLASS_AUDIO *audio,
        ULONG stream_index, UX_DEVICE_CLASS_AUDIO_STREAM **stream)
{

    /* Sanity check.  */
    if (audio == UX_NULL)
        return(UX_INVALID_PARAMETER);

    /* Index validation.  */
    if (stream_index >= audio -> ux_device_class_audio_streams_nb)
        return(UX_INVALID_PARAMETER);

    /* Store the stream instance found.  */
    if (stream == UX_NULL)
        return(UX_INVALID_PARAMETER);

    /* Get audio stream instance.  */
    return(_ux_device_class_audio_stream_get(audio, stream_index, stream));
}
