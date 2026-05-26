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
/**   Device Printer Class                                                */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/

/**************************************************************************/
/*                                                                        */
/*  COMPONENT DEFINITION                                   RELEASE        */
/*                                                                        */
/*    ux_device_class_printer.h                           PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This file defines the equivalences for the USBX Device Class        */
/*    Printer component.                                                  */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  01-31-2022     Chaoqiong Xiao           Initial Version 6.1.10        */
/*  04-25-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            fixed standalone compile,   */
/*                                            resulting in version 6.1.11 */
/*  07-29-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            resulting in version 6.1.12 */
/*  10-31-2022     Yajun xia                Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.2.0  */
/*  03-08-2023     Yajun xia                Modified comment(s),          */
/*                                            added error checks support, */
/*                                            resulting in version 6.2.1  */
/*  10-31-2023     Yajun Xia, CQ Xiao       Modified comment(s),          */
/*                                            added a new mode to manage  */
/*                                            endpoint buffer in classes, */
/*                                            fixed error checking issue, */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/

#ifndef UX_DEVICE_CLASS_PRINTER_H
#define UX_DEVICE_CLASS_PRINTER_H

/* Determine if a C++ compiler is being used.  If so, ensure that standard
   C is used to process the API information.  */

#ifdef   __cplusplus

/* Yes, C++ compiler is present.  Use standard C.  */
extern   "C" {

#endif

/* Internal option: enable the basic USBX error checking. This define is typically used
   while debugging application.  */
#if defined(UX_ENABLE_ERROR_CHECKING) && !defined(UX_DEVICE_CLASS_PRINTER_ENABLE_ERROR_CHECKING)
#define UX_DEVICE_CLASS_PRINTER_ENABLE_ERROR_CHECKING
#endif


/* Option: defined, it enables zero copy support (works if PRINTER owns endpoint buffer).
    Defined, it enables zero copy for bulk in/out endpoints (write/read). In this case, the endpoint
    buffer is not allocated in class, application must provide the buffer for read/write, and the
    buffer must meet device controller driver (DCD) buffer requirements (e.g., aligned and cache
    safe if buffer is for DMA).
 */
/* #define UX_DEVICE_CLASS_PRINTER_ZERO_COPY  */


/* Defined, _write is pending ZLP automatically (complete transfer) after buffer is sent.  */

/* #define UX_DEVICE_CLASS_PRINTER_WRITE_AUTO_ZLP  */


/* Option: bulk out endpoint / read buffer size, must be larger than max packet size in framework, and aligned in 4-bytes.  */
#ifndef UX_DEVICE_CLASS_PRINTER_READ_BUFFER_SIZE
#define UX_DEVICE_CLASS_PRINTER_READ_BUFFER_SIZE                         512
#endif

/* Option: bulk in endpoint / write buffer size, must be larger than max packet size in framework, and aligned in 4-bytes.  */
#ifndef UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE
#define UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE                        UX_SLAVE_REQUEST_DATA_MAX_LENGTH
#endif


/* Internal: check if class own endpoint buffer  */
#if (UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1) &&                                   \
    (!defined(UX_DEVICE_CLASS_PRINTER_ZERO_COPY))
#define UX_DEVICE_CLASS_PRINTER_OWN_ENDPOINT_BUFFER
#endif


/* Define Printer Class USB Class constants.  */
#define UX_DEVICE_CLASS_PRINTER_CLASS                                    7

#define UX_DEVICE_CLASS_PRINTER_SUBCLASS                                 1

#define UX_DEVICE_CLASS_PRINTER_PROTOCOL_UNIDIRECTIONAL                  1
#define UX_DEVICE_CLASS_PRINTER_PROTOCOL_BIDIRECTIONAL                   2
#define UX_DEVICE_CLASS_PRINTER_PROTOCOL_1284_4_COMPATIBLE_BIDIR         3


/* Device Printer Requests */
#define UX_DEVICE_CLASS_PRINTER_GET_DEVICE_ID                            0x00
#define UX_DEVICE_CLASS_PRINTER_GET_PORT_STATUS                          0x01
#define UX_DEVICE_CLASS_PRINTER_SOFT_RESET                               0x02


/* Printer Port Status.  */
#define UX_DEVICE_CLASS_PRINTER_PAPER_EMPTY                             (1u << 5)
#define UX_DEVICE_CLASS_PRINTER_SELECT                                  (1u << 4)
#define UX_DEVICE_CLASS_PRINTER_NOT_ERROR                               (1u << 3)


/* Printer IOCTL code.  */
#define UX_DEVICE_CLASS_PRINTER_IOCTL_PORT_STATUS_SET                   1
#define UX_DEVICE_CLASS_PRINTER_IOCTL_READ_TIMEOUT_SET                  2
#define UX_DEVICE_CLASS_PRINTER_IOCTL_WRITE_TIMEOUT_SET                 3

#if defined(UX_DEVICE_STANDALONE)

/* Printer read state machine states.  */
#define UX_DEVICE_CLASS_PRINTER_READ_START                              (UX_STATE_STEP + 1)
#define UX_DEVICE_CLASS_PRINTER_READ_WAIT                               (UX_STATE_STEP + 2)

/* Printer write state machine states.  */
#define UX_DEVICE_CLASS_PRINTER_WRITE_START                             (UX_STATE_STEP + 1)
#define UX_DEVICE_CLASS_PRINTER_WRITE_WAIT                              (UX_STATE_STEP + 2)
#endif

/* Define Device Printer Class Calling Parameter structure */

typedef struct UX_DEVICE_CLASS_PRINTER_PARAMETER_STRUCT
{
    UCHAR                   *ux_device_class_printer_device_id; /* IEEE 1284 string, first 2 big endian length.  */
    VOID                    (*ux_device_class_printer_instance_activate)(VOID *);
    VOID                    (*ux_device_class_printer_instance_deactivate)(VOID *);
    VOID                    (*ux_device_class_printer_soft_reset)(VOID *);
} UX_DEVICE_CLASS_PRINTER_PARAMETER;


/* Define Printer Class structure.  */

typedef struct UX_DEVICE_CLASS_PRINTER_STRUCT
{
    UX_SLAVE_INTERFACE      *ux_device_class_printer_interface;
    UX_SLAVE_ENDPOINT       *ux_device_class_printer_endpoint_out;
    UX_SLAVE_ENDPOINT       *ux_device_class_printer_endpoint_in;
#if UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1
    UCHAR                   *ux_device_class_printer_endpoint_buffer;
#endif
    ULONG                   ux_device_class_printer_port_status;
    UX_DEVICE_CLASS_PRINTER_PARAMETER
                            ux_device_class_printer_parameter;
#if !defined(UX_DEVICE_STANDALONE)
    UX_MUTEX                ux_device_class_printer_endpoint_out_mutex;
    UX_MUTEX                ux_device_class_printer_endpoint_in_mutex;
#else
    UCHAR                  *ux_device_class_printer_read_buffer;
    ULONG                   ux_device_class_printer_read_requested_length;
    ULONG                   ux_device_class_printer_read_transfer_length;
    ULONG                   ux_device_class_printer_read_actual_length;
    UINT                    ux_device_class_printer_read_status;
    UINT                    ux_device_class_printer_read_state;

    UCHAR                  *ux_device_class_printer_write_buffer;
    ULONG                   ux_device_class_printer_write_transfer_length;
    ULONG                   ux_device_class_printer_write_host_length;
    ULONG                   ux_device_class_printer_write_requested_length;
    ULONG                   ux_device_class_printer_write_actual_length;
    UINT                    ux_device_class_printer_write_status;
    UINT                    ux_device_class_printer_write_state;
#endif
} UX_DEVICE_CLASS_PRINTER;

/* Define PRINTER endpoint buffer settings (when PRINTER owns buffer).  */
#define UX_DEVICE_CLASS_PRINTER_ENDPOINT_BUFFER_SIZE_CALC_OVERFLOW \
    (UX_OVERFLOW_CHECK_ADD_ULONG(UX_DEVICE_CLASS_PRINTER_READ_BUFFER_SIZE,      \
                                 UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE))
#define UX_DEVICE_CLASS_PRINTER_ENDPOINT_BUFFER_SIZE    (UX_DEVICE_CLASS_PRINTER_READ_BUFFER_SIZE + UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER_SIZE)
#define UX_DEVICE_CLASS_PRINTER_READ_BUFFER(ecm)        ((ecm)->ux_device_class_printer_endpoint_buffer)
#define UX_DEVICE_CLASS_PRINTER_WRITE_BUFFER(ecm)       (UX_DEVICE_CLASS_PRINTER_READ_BUFFER(ecm) + UX_DEVICE_CLASS_PRINTER_READ_BUFFER_SIZE)


/* Define Device Printer Class prototypes.  */

UINT  _ux_device_class_printer_activate(UX_SLAVE_CLASS_COMMAND *command);
UINT  _ux_device_class_printer_control_request(UX_SLAVE_CLASS_COMMAND *command);
UINT  _ux_device_class_printer_deactivate(UX_SLAVE_CLASS_COMMAND *command);
UINT  _ux_device_class_printer_entry(UX_SLAVE_CLASS_COMMAND *command);
UINT  _ux_device_class_printer_initialize(UX_SLAVE_CLASS_COMMAND *command);
UINT  _ux_device_class_printer_uninitialize(UX_SLAVE_CLASS_COMMAND *command);

VOID  _ux_device_class_printer_soft_reset(UX_DEVICE_CLASS_PRINTER *printer);

UINT  _ux_device_class_printer_write(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);
UINT  _ux_device_class_printer_read(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);

UINT  _ux_device_class_printer_ioctl(UX_DEVICE_CLASS_PRINTER *printer, ULONG ioctl_function,
                                    VOID *parameter);

UINT  _ux_device_class_printer_write_run(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);
UINT  _ux_device_class_printer_read_run(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);

UINT  _uxe_device_class_printer_initialize(UX_SLAVE_CLASS_COMMAND *command);
UINT  _uxe_device_class_printer_read(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);
UINT  _uxe_device_class_printer_write(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);
UINT  _uxe_device_class_printer_ioctl(UX_DEVICE_CLASS_PRINTER *printer, ULONG ioctl_function,
                                    VOID *parameter);
UINT  _uxe_device_class_printer_write_run(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);
UINT  _uxe_device_class_printer_read_run(UX_DEVICE_CLASS_PRINTER *printer, UCHAR *buffer,
                                ULONG requested_length, ULONG *actual_length);

/* Define Device Printer Class API prototypes.  */
#if defined(UX_DEVICE_CLASS_PRINTER_ENABLE_ERROR_CHECKING)

#define ux_device_class_printer_entry               _ux_device_class_printer_entry
#define ux_device_class_printer_read                _uxe_device_class_printer_read
#define ux_device_class_printer_write               _uxe_device_class_printer_write
#define ux_device_class_printer_ioctl               _uxe_device_class_printer_ioctl
#define ux_device_class_printer_read_run            _uxe_device_class_printer_read_run
#define ux_device_class_printer_write_run           _uxe_device_class_printer_write_run

#else

#define ux_device_class_printer_entry               _ux_device_class_printer_entry
#define ux_device_class_printer_read                _ux_device_class_printer_read
#define ux_device_class_printer_write               _ux_device_class_printer_write
#define ux_device_class_printer_ioctl               _ux_device_class_printer_ioctl
#define ux_device_class_printer_read_run            _ux_device_class_printer_read_run
#define ux_device_class_printer_write_run           _ux_device_class_printer_write_run

#endif

/* Determine if a C++ compiler is being used.  If so, complete the standard
   C conditional started above.  */
#ifdef __cplusplus
}
#endif

#endif /* UX_DEVICE_CLASS_PRINTER_H */
