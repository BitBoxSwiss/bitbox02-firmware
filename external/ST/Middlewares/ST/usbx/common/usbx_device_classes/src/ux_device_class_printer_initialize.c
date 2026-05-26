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
/**   Device Printer Class                                                */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

#define UX_SOURCE_CODE


/* Include necessary system files.  */

#include "ux_api.h"
#include "ux_device_class_printer.h"
#include "ux_device_stack.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_device_class_printer_initialize                 PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function initializes the USB Printer device.                   */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    command                               Pointer to printer command    */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_memory_allocate           Allocate memory               */
/*    _ux_utility_memory_free               Free memory                   */
/*    _ux_utility_mutex_create              Create mutex                  */
/*    _ux_device_mutex_delete               Delete mutex                  */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Device Stack                                                        */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  01-31-2022     Chaoqiong Xiao           Initial Version 6.1.10        */
/*  04-25-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            fixed standalone compile,   */
/*                                            resulting in version 6.1.11 */
/*  10-31-2022     Yajun Xia                Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.2.0  */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added a new mode to manage  */
/*                                            endpoint buffer in classes, */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
UINT  _ux_device_class_printer_initialize(UX_SLAVE_CLASS_COMMAND *command)
{

UX_DEVICE_CLASS_PRINTER                 *printer;
UX_DEVICE_CLASS_PRINTER_PARAMETER       *printer_parameter;
UX_SLAVE_CLASS                          *printer_class;
#if !defined(UX_DEVICE_STANDALONE)
UINT                                    status;
#endif

    /* Get the class container.  */
    printer_class =  command -> ux_slave_class_command_class_ptr;

    /* Create an instance of the device printer class.  */
    printer =  _ux_utility_memory_allocate(UX_NO_ALIGN, UX_REGULAR_MEMORY, sizeof(UX_DEVICE_CLASS_PRINTER));

    /* Check for successful allocation.  */
    if (printer == UX_NULL)
        return(UX_MEMORY_INSUFFICIENT);

    /* Save the address of the Printer instance inside the Printer container.  */
    printer_class -> ux_slave_class_instance = (VOID *) printer;

    /* Get the pointer to the application parameters for the printer class.  */
    printer_parameter =  command -> ux_slave_class_command_parameter;

    /* Store the start and stop signals if needed by the application.  */
    printer -> ux_device_class_printer_parameter.ux_device_class_printer_device_id           = printer_parameter -> ux_device_class_printer_device_id;
    printer -> ux_device_class_printer_parameter.ux_device_class_printer_instance_activate   = printer_parameter -> ux_device_class_printer_instance_activate;
    printer -> ux_device_class_printer_parameter.ux_device_class_printer_instance_deactivate = printer_parameter -> ux_device_class_printer_instance_deactivate;
    printer -> ux_device_class_printer_parameter.ux_device_class_printer_soft_reset          = printer_parameter -> ux_device_class_printer_soft_reset;

#if defined(UX_DEVICE_CLASS_PRINTER_OWN_ENDPOINT_BUFFER)

    /* Allocate endpoint buffer.  */
    UX_ASSERT(!UX_DEVICE_CLASS_PRINTER_ENDPOINT_BUFFER_SIZE_CALC_OVERFLOW);
    printer -> ux_device_class_printer_endpoint_buffer = _ux_utility_memory_allocate(UX_NO_ALIGN,
            UX_CACHE_SAFE_MEMORY, UX_DEVICE_CLASS_PRINTER_ENDPOINT_BUFFER_SIZE);
    if (printer -> ux_device_class_printer_endpoint_buffer == UX_NULL)
    {
        _ux_utility_memory_free(printer);
        return(UX_MEMORY_INSUFFICIENT);
    }
#endif

#if !defined(UX_DEVICE_STANDALONE)
    /* Create the Mutex for each endpoint as multiple threads cannot access each pipe at the same time.  */
    status =  _ux_utility_mutex_create(&printer -> ux_device_class_printer_endpoint_in_mutex, "ux_device_class_printer_in_mutex");

    /* Check Mutex creation error.  */
    if(status != UX_SUCCESS)
    {

        /* Free the resources.  */
#if defined(UX_DEVICE_CLASS_PRINTER_OWN_ENDPOINT_BUFFER)
        _ux_utility_memory_free(printer -> ux_device_class_printer_endpoint_buffer);
#endif
        _ux_utility_memory_free(printer);

        /* Return fatal error.  */
        return(UX_MUTEX_ERROR);
    }

    /* Out Mutex. */
    status =  _ux_utility_mutex_create(&printer -> ux_device_class_printer_endpoint_out_mutex, "ux_device_class_printer_out_mutex");

    /* Check Mutex creation error.  */
    if(status != UX_SUCCESS)
    {

        /* Delete the endpoint IN mutex.  */
        _ux_device_mutex_delete(&printer -> ux_device_class_printer_endpoint_in_mutex);

        /* Free the resources.  */
#if defined(UX_DEVICE_CLASS_PRINTER_OWN_ENDPOINT_BUFFER)
        _ux_utility_memory_free(printer -> ux_device_class_printer_endpoint_buffer);
#endif
        _ux_utility_memory_free(printer);

        /* Return fatal error.  */
        return(UX_MUTEX_ERROR);
    }
#else
    printer -> ux_device_class_printer_write_state = UX_STATE_RESET;
    printer -> ux_device_class_printer_read_state = UX_STATE_RESET;
#endif

    /* Reset port status.  */
    printer -> ux_device_class_printer_port_status = 0;

    /* Return completion status.  */
    return(UX_SUCCESS);
}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _uxe_device_class_printer_initialize                  PORTABLE C    */
/*                                                           6.2.1        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Yajun Xia, Microsoft Corporation                                    */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function checks errors in printer initialization function call.*/
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    command                               Pointer to printer command    */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    Completion Status                                                   */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_device_class_printer_initialize   Initialize printer instance   */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    Device Stack                                                        */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  03-08-2023     Yajun Xia                Initial Version 6.2.1         */
/*                                                                        */
/**************************************************************************/
UINT  _uxe_device_class_printer_initialize(UX_SLAVE_CLASS_COMMAND *command)
{
UX_DEVICE_CLASS_PRINTER_PARAMETER       *printer_parameter;
ULONG length;

    /* Get the pointer to the application parameters for the printer class.  */
    printer_parameter =  command -> ux_slave_class_command_parameter;

    /* Sanity checks.  */

    /* Length of data (first two bytes in big endian).  */
    length = _ux_utility_short_get_big_endian(printer_parameter -> ux_device_class_printer_device_id);

    if (length > UX_SLAVE_REQUEST_CONTROL_MAX_LENGTH)
    {
        return(UX_INVALID_PARAMETER);
    }

    return (_ux_device_class_printer_initialize(command));
}
