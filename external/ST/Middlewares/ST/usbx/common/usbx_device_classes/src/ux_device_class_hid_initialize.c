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
/**   Device HID Class                                                    */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_hid.h"
#include "ux_device_stack.h"


/**************************************************************************/ 
/*                                                                        */ 
/*  FUNCTION                                               RELEASE        */ 
/*                                                                        */ 
/*    _ux_device_class_hid_initialize                     PORTABLE C      */ 
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */ 
/*    This function initializes the USB HID device.                       */ 
/*    This function is called by the class register function. It is only  */ 
/*    done once.                                                          */ 
/*                                                                        */ 
/*  INPUT                                                                 */ 
/*                                                                        */ 
/*    command                              Pointer to hid command         */ 
/*                                                                        */ 
/*  OUTPUT                                                                */ 
/*                                                                        */ 
/*    Completion Status                                                   */ 
/*                                                                        */ 
/*  CALLS                                                                 */ 
/*                                                                        */ 
/*    _ux_utility_memory_allocate           Allocate memory               */
/*    _ux_utility_memory_free               Free memory                   */
/*    _ux_device_thread_create              Create thread                 */
/*    _ux_device_thread_delete              Delete thread                 */
/*    _ux_utility_event_flags_create        Create event flags group      */
/*                                                                        */ 
/*  CALLED BY                                                             */ 
/*                                                                        */ 
/*    USBX Source Code                                                    */ 
/*                                                                        */ 
/*  RELEASE HISTORY                                                       */ 
/*                                                                        */ 
/*    DATE              NAME                      DESCRIPTION             */ 
/*                                                                        */ 
/*  05-19-2020     Chaoqiong Xiao           Initial Version 6.0           */
/*  09-30-2020     Chaoqiong Xiao           Modified comment(s),          */
/*                                            used UX prefix to refer to  */
/*                                            TX symbols instead of using */
/*                                            them directly,              */
/*                                            resulting in version 6.1    */
/*  01-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            added interrupt OUT support,*/
/*                                            resulting in version 6.1.10 */
/*  04-25-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            resulting in version 6.1.11 */
/*  07-29-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone receiver,  */
/*                                            fixed parameter/variable    */
/*                                            names conflict C++ keyword, */
/*                                            resulting in version 6.1.12 */
/*  10-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            fixed compile warnings,     */
/*                                            resulting in version 6.2.0  */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added zero copy support,    */
/*                                            added a new mode to manage  */
/*                                            endpoint buffer in classes, */
/*                                            checked compile options,    */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT  _ux_device_class_hid_initialize(UX_SLAVE_CLASS_COMMAND *command)
{
                                          
UX_SLAVE_CLASS_HID                      *hid;
UX_SLAVE_CLASS_HID_PARAMETER            *hid_parameter;
UX_SLAVE_CLASS                          *class_ptr;
UINT                                    status = UX_SUCCESS;
ULONG                                   array_memory_size;
#if (UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1) && defined(UX_DEVICE_CLASS_HID_ZERO_COPY)
UINT                                    i;
UCHAR                                   *buffer;
#endif


    /* Compile option checks.  */
    UX_ASSERT(UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH <= UX_SLAVE_REQUEST_CONTROL_MAX_LENGTH);
#if UX_DEVICE_ENDPOINT_BUFFER_OWNER == 0
    UX_ASSERT(UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH <= UX_SLAVE_REQUEST_DATA_MAX_LENGTH);
#endif


    /* Get the pointer to the application parameters for the hid class.  */
    hid_parameter =  command -> ux_slave_class_command_parameter;

    /* Get the class container.  */
    class_ptr =  command -> ux_slave_class_command_class_ptr;

    /* Create an instance of the device hid class.  */
    hid =  _ux_utility_memory_allocate(UX_NO_ALIGN, UX_REGULAR_MEMORY, sizeof(UX_SLAVE_CLASS_HID));

    /* Check for successful allocation.  */
    if (hid == UX_NULL)
        return(UX_MEMORY_INSUFFICIENT);

    /* Save the address of the HID instance inside the HID container.  */
    class_ptr -> ux_slave_class_instance = (VOID *) hid;

#if defined(UX_DEVICE_CLASS_HID_OWN_ENDPOINT_BUFFER)

    /* Allocate buffer(s) for endpoint(s).  */
    UX_ASSERT(!UX_DEVICE_CLASS_HID_ENDPOINT_BUFFER_SIZE_CALC_OVERFLOW);
    hid -> ux_device_class_hid_endpoint_buffer = _ux_utility_memory_allocate(
                            UX_NO_ALIGN, UX_CACHE_SAFE_MEMORY,
                            UX_DEVICE_CLASS_HID_ENDPOINT_BUFFER_SIZE);
    if (hid -> ux_device_class_hid_endpoint_buffer == UX_NULL)
    {
        _ux_utility_memory_free(hid);
        return(UX_MEMORY_INSUFFICIENT);
    }
#endif

#if !defined(UX_DEVICE_STANDALONE)

    /* Allocate some memory for the thread stack. */
    class_ptr -> ux_slave_class_thread_stack =  
            _ux_utility_memory_allocate(UX_NO_ALIGN, UX_REGULAR_MEMORY, UX_DEVICE_CLASS_HID_THREAD_STACK_SIZE);
    
    /* Check for successful allocation.  */
    if (class_ptr -> ux_slave_class_thread_stack == UX_NULL)
        status = UX_MEMORY_INSUFFICIENT;

    /* This instance needs to be running in a different thread. So start
       a new thread. We pass a pointer to the class to the new thread.  This thread
       does not start until we have a instance of the class. */
    if (status == UX_SUCCESS)
        status =  _ux_device_thread_create(&class_ptr -> ux_slave_class_thread, "ux_slave_hid_thread", 
                    _ux_device_class_hid_interrupt_thread,
                    (ULONG) (ALIGN_TYPE) class_ptr, (VOID *) class_ptr -> ux_slave_class_thread_stack,
                    UX_DEVICE_CLASS_HID_THREAD_STACK_SIZE, UX_THREAD_PRIORITY_CLASS,
                    UX_THREAD_PRIORITY_CLASS, UX_NO_TIME_SLICE, UX_DONT_START);
#else

#if defined(UX_DEVICE_CLASS_HID_FLEXIBLE_EVENTS_QUEUE)

    /* Set event buffer.  */
    hid -> ux_device_class_hid_event.ux_device_class_hid_event_buffer =
                                    UX_DEVICE_CLASS_HID_INTERRUPTIN_BUFFER(hid);
#endif

    /* Set task function.  */
    class_ptr -> ux_slave_class_task_function = _ux_device_class_hid_tasks_run;
#endif

    /* Check the creation of this thread.  */
    if (status == UX_SUCCESS)
    {

#if !defined(UX_DEVICE_STANDALONE)
        UX_THREAD_EXTENSION_PTR_SET(&(class_ptr -> ux_slave_class_thread), class_ptr)
#endif


        /* Store all the application parameter information about the report.  */
        hid -> ux_device_class_hid_report_address             = hid_parameter -> ux_device_class_hid_parameter_report_address;
        hid -> ux_device_class_hid_report_length              = hid_parameter -> ux_device_class_hid_parameter_report_length;
        hid -> ux_device_class_hid_report_id                  = hid_parameter -> ux_device_class_hid_parameter_report_id;

        /* Store the callback function.  */
        hid -> ux_device_class_hid_callback                   = hid_parameter -> ux_device_class_hid_parameter_callback;
        hid -> ux_device_class_hid_get_callback               = hid_parameter -> ux_device_class_hid_parameter_get_callback;

#if defined(UX_DEVICE_CLASS_HID_FLEXIBLE_EVENTS_QUEUE)

        /* If event length is invalid, UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH is used.  */
        if (UX_DEVICE_CLASS_HID_PARAM_EVENT_MAX_LENGTH(hid_parameter) == 0 ||
            UX_DEVICE_CLASS_HID_PARAM_EVENT_MAX_LENGTH(hid_parameter) > UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH)
            UX_DEVICE_CLASS_HID_PARAM_EVENT_MAX_LENGTH(hid_parameter) = UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH;

        /* If event queue size is invalid, UX_DEVICE_CLASS_HID_MAX_EVENTS_QUEUE is used.  */
        if (UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter) < 2 ||
            UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter) > UX_DEVICE_CLASS_HID_MAX_EVENTS_QUEUE)
            UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter) = UX_DEVICE_CLASS_HID_MAX_EVENTS_QUEUE;

        /* Save event size.  */
        UX_DEVICE_CLASS_HID_EVENT_MAX_LENGTH(hid) = UX_DEVICE_CLASS_HID_PARAM_EVENT_MAX_LENGTH(hid_parameter);
#endif

        /* Create the event array.  */
        UX_ASSERT(!UX_OVERFLOW_CHECK_MULC_ULONG(
                    UX_DEVICE_CLASS_HID_EVENT_QUEUE_ITEM_SIZE(hid),
                    UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter)));
        array_memory_size = UX_DEVICE_CLASS_HID_EVENT_QUEUE_ITEM_SIZE(hid) * UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter);
        hid -> ux_device_class_hid_event_array =  _ux_utility_memory_allocate(UX_NO_ALIGN,
                                        UX_REGULAR_MEMORY, array_memory_size);

        /* Do we need event buffer?
         * 1. Even zero copy, report copy is kept to avoid keep buffers in application.
         * 2. Other cases, buffer must be allocated.
         */
        /* Allocate buffer if needed.  */
        {

#if (UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1) && defined(UX_DEVICE_CLASS_HID_ZERO_COPY)

            /* Allocate cache safe event buffers.  */
            buffer = _ux_utility_memory_allocate_mulv_safe(UX_NO_ALIGN, UX_CACHE_SAFE_MEMORY,
                        UX_DEVICE_CLASS_HID_PARAM_EVENT_MAX_LENGTH(hid_parameter),
                        UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter));

            /* Allocation error check.  */
            if (buffer == UX_NULL)
            {
                if (hid -> ux_device_class_hid_event_array != UX_NULL)
                {
                    _ux_utility_memory_free(hid -> ux_device_class_hid_event_array);
                    hid -> ux_device_class_hid_event_array = UX_NULL;
                }
            }
            else
            {

                /* Assign event buffers.  */
                for (i = 0; i < UX_DEVICE_CLASS_HID_PARAM_EVENT_QUEUE_SIZE(hid_parameter); i ++)
                {
                    hid -> ux_device_class_hid_event_array[i].ux_device_class_hid_event_buffer = buffer;
                    buffer += UX_DEVICE_CLASS_HID_PARAM_EVENT_MAX_LENGTH(hid_parameter);
                }
            }
#else

            /* Regular event place data following id,type and length.  */
#endif
        }

        /* Check for successful allocation.  */
        if (hid -> ux_device_class_hid_event_array != UX_NULL)
        {

            /* Initialize the head and tail of the notification round robin buffers. 
               At first, the head and tail are pointing to the beginning of the array.  */
            hid -> ux_device_class_hid_event_array_head =  hid -> ux_device_class_hid_event_array;
            hid -> ux_device_class_hid_event_array_tail =  hid -> ux_device_class_hid_event_array;
            hid -> ux_device_class_hid_event_array_end  =  (UX_DEVICE_CLASS_HID_EVENT*)((UCHAR*)hid -> ux_device_class_hid_event_array + array_memory_size);

            /* Store the start and stop signals if needed by the application.  */
            hid -> ux_slave_class_hid_instance_activate = hid_parameter -> ux_slave_class_hid_instance_activate;
            hid -> ux_slave_class_hid_instance_deactivate = hid_parameter -> ux_slave_class_hid_instance_deactivate;

            /* By default no event wait timeout.  */
            hid -> ux_device_class_hid_event_wait_timeout = UX_WAIT_FOREVER;

#if !defined(UX_DEVICE_STANDALONE)

            /* Create a event flag group for the hid class to synchronize with the event interrupt thread.  */
            status =  _ux_utility_event_flags_create(&hid -> ux_device_class_hid_event_flags_group, "ux_device_class_hid_event_flag");

            /* Check status.  */
            if (status != UX_SUCCESS)
                status = UX_EVENT_ERROR;
            else
#endif
            {
#if defined(UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT)

#if !defined(UX_DEVICE_STANDALONE)

                /* Create a mutex for reading reentry check.  */
                status = _ux_utility_mutex_create(&hid -> ux_device_class_hid_read_mutex,
                                                  "ux_device_class_hid_read_mutex");
                if (status == UX_SUCCESS)
                {
#endif

                    /* If receiver is enabled by parameter, initialize it.  */
                    if (hid_parameter -> ux_device_class_hid_parameter_receiver_initialize)
                    {

                        /* Allocate buffer for receiver and receiver events.  */
                        status = hid_parameter ->
                                ux_device_class_hid_parameter_receiver_initialize(hid,
                                                hid_parameter,
                                                &hid -> ux_device_class_hid_receiver);
                    }

                    /* Done success, return.  */
                    if (status == UX_SUCCESS)
                        return(status);

#if !defined(UX_DEVICE_STANDALONE)

                    /* There is error, delete mutex.  */
                    _ux_device_mutex_delete(&hid -> ux_device_class_hid_read_mutex);
                }
                else
                    status = UX_MUTEX_ERROR;

                /* There is error, delete event flags.  */
                _ux_utility_event_flags_delete(&hid -> ux_device_class_hid_event_flags_group);
#endif
#else
                return(status);
#endif

            }

#if !defined(UX_DEVICE_STANDALONE) || defined(UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT)

            /* There is still initialization activities after array creation,
             * and some error occurs in this stage.  */
            /* Free allocated event array memory.  */
#if (UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1) && defined(UX_DEVICE_CLASS_HID_ZERO_COPY)
            _ux_utility_memory_free(hid -> ux_device_class_hid_event_array -> ux_device_class_hid_event_buffer);
#endif
            _ux_utility_memory_free(hid -> ux_device_class_hid_event_array);
#endif

        }
        else
            status =  UX_MEMORY_INSUFFICIENT;

#if !defined(UX_DEVICE_STANDALONE)

        /* Delete thread.  */
        _ux_device_thread_delete(&class_ptr -> ux_slave_class_thread);
#endif
    }
    else
        status = (UX_THREAD_ERROR);

#if !defined(UX_DEVICE_STANDALONE)

    /* Free stack. */
    if (class_ptr -> ux_slave_class_thread_stack)
        _ux_utility_memory_free(class_ptr -> ux_slave_class_thread_stack);
#endif

#if defined(UX_DEVICE_CLASS_HID_OWN_ENDPOINT_BUFFER)
    _ux_utility_memory_free(hid -> ux_device_class_hid_endpoint_buffer);
#endif

    /* Unmount instance. */
    class_ptr -> ux_slave_class_instance =  UX_NULL;

    /* Free HID instance. */
    _ux_utility_memory_free(hid);

    /* Return completion status.  */
    return(status);
}


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_hid_initialize                    PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in HID initialize function call.        */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    command                               Pointer to hid command        */ 
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_hid_initialize       Initialize HID instance       */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Device Stack                                                        */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Chaoqiong Xiao           Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
UINT  _uxe_device_class_hid_initialize(UX_SLAVE_CLASS_COMMAND *command)
{

UX_SLAVE_CLASS_HID_PARAMETER            *hid_parameter;

    /* Get the pointer to the application parameters for the hid class.  */
    hid_parameter =  command -> ux_slave_class_command_parameter;

    /* Check input parameters.  */
    if ((hid_parameter -> ux_device_class_hid_parameter_report_address == UX_NULL) ||
        (hid_parameter -> ux_device_class_hid_parameter_report_length == 0) ||
        (hid_parameter -> ux_device_class_hid_parameter_report_length > UX_SLAVE_REQUEST_CONTROL_MAX_LENGTH))
    {
        return(UX_INVALID_PARAMETER);
    }

    /* Invoke initialize function.  */
    return(_ux_device_class_hid_initialize(command));
}
