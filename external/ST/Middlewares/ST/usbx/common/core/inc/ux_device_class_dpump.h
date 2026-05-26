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
/**   Device Data Pump Class                                              */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/


/**************************************************************************/ 
/*                                                                        */ 
/*  COMPONENT DEFINITION                                   RELEASE        */ 
/*                                                                        */ 
/*    ux_device_class_dpump.h                             PORTABLE C      */ 
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */ 
/*    This file contains all the header and extern functions used by the  */
/*    USBX device dpump class.                                            */ 
/*                                                                        */ 
/*  RELEASE HISTORY                                                       */ 
/*                                                                        */ 
/*    DATE              NAME                      DESCRIPTION             */ 
/*                                                                        */ 
/*  05-19-2020     Chaoqiong Xiao           Initial Version 6.0           */
/*  09-30-2020     Chaoqiong Xiao           Modified comment(s),          */
/*                                            resulting in version 6.1    */
/*  08-02-2021     Wen Wang                 Modified comment(s),          */
/*                                            added extern "C" keyword    */
/*                                            for compatibility with C++, */
/*                                            resulting in version 6.1.8  */
/*  01-31-2022     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added standalone support,   */
/*                                            resulting in version 6.1.10 */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            added a new mode to manage  */
/*                                            endpoint buffer in classes, */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/

#ifndef UX_DEVICE_CLASS_DPUMP_H
#define UX_DEVICE_CLASS_DPUMP_H

/* Determine if a C++ compiler is being used.  If so, ensure that standard 
   C is used to process the API information.  */ 

#ifdef   __cplusplus 

/* Yes, C++ compiler is present.  Use standard C.  */ 
extern   "C" { 

#endif  


/* Bulk out endpoint / read buffer size, must be larger than max packet size in framework, and aligned in 4-bytes.  */
#define UX_DEVICE_CLASS_DPUMP_READ_BUFFER_SIZE                  UX_SLAVE_REQUEST_DATA_MAX_LENGTH

/* Bulk in endpoint / write buffer size, must be larger than max packet size in framework, and aligned in 4-bytes.  */
#define UX_DEVICE_CLASS_DPUMP_WRITE_BUFFER_SIZE                 UX_SLAVE_REQUEST_DATA_MAX_LENGTH


/* Define Storage Class USB Class constants.  */

#define UX_SLAVE_CLASS_DPUMP_CLASS                              0x99
#define UX_SLAVE_CLASS_DPUMP_SUBCLASS                           0x99
#define UX_SLAVE_CLASS_DPUMP_PROTOCOL                           0x99

/* Define Data Pump Class packet equivalences.  */
#define UX_DEVICE_CLASS_DPUMP_PACKET_SIZE                       128


/* Define Slave DPUMP Class Calling Parameter structure */

typedef struct UX_SLAVE_CLASS_DPUMP_PARAMETER_STRUCT
{
    VOID                    (*ux_slave_class_dpump_instance_activate)(VOID *);
    VOID                    (*ux_slave_class_dpump_instance_deactivate)(VOID *);

} UX_SLAVE_CLASS_DPUMP_PARAMETER;

/* Define Slave Data Pump Class structure.  */

typedef struct UX_SLAVE_CLASS_DPUMP_STRUCT
{
    UX_SLAVE_INTERFACE                  *ux_slave_class_dpump_interface;
    UX_SLAVE_CLASS_DPUMP_PARAMETER      ux_slave_class_dpump_parameter;
    UX_SLAVE_ENDPOINT                   *ux_slave_class_dpump_bulkin_endpoint;
    UX_SLAVE_ENDPOINT                   *ux_slave_class_dpump_bulkout_endpoint;
#if UX_DEVICE_ENDPOINT_BUFFER_OWNER == 1
    UCHAR                               *ux_device_class_dpump_endpoint_buffer;
#endif
    ULONG                               ux_slave_class_dpump_alternate_setting;
#if defined(UX_DEVICE_STANDALONE)
    UCHAR                               *ux_device_class_dpump_write_buffer;
    ULONG                               ux_device_class_dpump_write_requested_length;
    ULONG                               ux_device_class_dpump_write_transfer_length;
    ULONG                               ux_device_class_dpump_write_actual_length;
    UINT                                ux_device_class_dpump_write_state;
    UINT                                ux_device_class_dpump_write_status;
    UCHAR                               *ux_device_class_dpump_read_buffer;
    ULONG                               ux_device_class_dpump_read_requested_length;
    ULONG                               ux_device_class_dpump_read_transfer_length;
    ULONG                               ux_device_class_dpump_read_actual_length;
    UINT                                ux_device_class_dpump_read_state;
    UINT                                ux_device_class_dpump_read_status;
#endif
} UX_SLAVE_CLASS_DPUMP;

/* Defined for endpoint buffer settings (when DPUMP owns buffer).  */
#define UX_DEVICE_CLASS_DPUMP_ENDPOINT_BUFFER_SIZE_CALC_OVERFLOW                \
    (UX_OVERFLOW_CHECK_ADD_ULONG(UX_DEVICE_CLASS_DPUMP_READ_BUFFER_SIZE,        \
                                 UX_DEVICE_CLASS_DPUMP_WRITE_BUFFER_SIZE))
#define UX_DEVICE_CLASS_DPUMP_ENDPOINT_BUFFER_SIZE  (UX_DEVICE_CLASS_DPUMP_READ_BUFFER_SIZE + UX_DEVICE_CLASS_DPUMP_WRITE_BUFFER_SIZE)
#define UX_DEVICE_CLASS_DPUMP_READ_BUFFER(dpump)    ((dpump)->ux_device_class_dpump_endpoint_buffer)
#define UX_DEVICE_CLASS_DPUMP_WRITE_BUFFER(dpump)   (UX_DEVICE_CLASS_DPUMP_READ_BUFFER(dpump) + UX_DEVICE_CLASS_DPUMP_READ_BUFFER_SIZE)


/* Define Device Data Pump Class prototypes.  */

UINT    _ux_device_class_dpump_initialize(UX_SLAVE_CLASS_COMMAND *command);
UINT    _ux_device_class_dpump_activate(UX_SLAVE_CLASS_COMMAND *command);
UINT    _ux_device_class_dpump_deactivate(UX_SLAVE_CLASS_COMMAND *command);
UINT    _ux_device_class_dpump_entry(UX_SLAVE_CLASS_COMMAND *command);
UINT    _ux_device_class_dpump_read(UX_SLAVE_CLASS_DPUMP *dpump, UCHAR *buffer, 
                                ULONG requested_length, ULONG *actual_length);
UINT    _ux_device_class_dpump_read_run(UX_SLAVE_CLASS_DPUMP *dpump, UCHAR *buffer, 
                                ULONG requested_length, ULONG *actual_length);
UINT    _ux_device_class_dpump_write(UX_SLAVE_CLASS_DPUMP *dpump, UCHAR *buffer, 
                                ULONG requested_length, ULONG *actual_length);
UINT    _ux_device_class_dpump_write_run(UX_SLAVE_CLASS_DPUMP *dpump, UCHAR *buffer, 
                                ULONG requested_length, ULONG *actual_length);
UINT    _ux_device_class_dpump_change(UX_SLAVE_CLASS_COMMAND *command);
                                
/* Define Device DPUMP Class API prototypes.  */

#define ux_device_class_dpump_entry                               _ux_device_class_dpump_entry
#define ux_device_class_dpump_read                                _ux_device_class_dpump_read
#define ux_device_class_dpump_read_run                            _ux_device_class_dpump_read_run
#define ux_device_class_dpump_write                               _ux_device_class_dpump_write
#define ux_device_class_dpump_write_run                           _ux_device_class_dpump_write_run

/* Determine if a C++ compiler is being used.  If so, complete the standard 
   C conditional started above.  */   
#ifdef __cplusplus
} 
#endif

#endif
