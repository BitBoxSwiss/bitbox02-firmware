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
/**   Utility                                                             */
/**                                                                       */
/**************************************************************************/
/**************************************************************************/


/* Include necessary system files.  */

#define UX_SOURCE_CODE

#include "ux_api.h"


/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_utility_descriptor_parse                        PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function will unpack a USB descriptor from the bus into a      */
/*    memory aligned structure.                                           */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    raw_descriptor                        Pointer to packed descriptor  */
/*    descriptor_structure                  Components of the descriptor  */
/*    descriptor_entries                    Number of entries in the      */
/*                                            descriptor                  */
/*    descriptor                            Pointer to the unpacked       */
/*                                            descriptor                  */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    _ux_utility_long_get                  Get 32-bit value              */
/*    _ux_utility_short_get                 Get 16-bit value              */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Components                                                     */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  05-19-2020     Chaoqiong Xiao           Initial Version 6.0           */
/*  09-30-2020     Chaoqiong Xiao           Modified comment(s),          */
/*                                            resulting in version 6.1    */
/*  10-31-2023     Chaoqiong Xiao           Modified comment(s),          */
/*                                            optimized USB descriptors,  */
/*                                            resulting in version 6.3.0  */
/*                                                                        */
/**************************************************************************/
VOID  _ux_utility_descriptor_parse(UCHAR * raw_descriptor, UCHAR * descriptor_structure,
                        UINT descriptor_entries, UCHAR * descriptor)
{

    /* Loop on all the entries in this descriptor.  */
    while(descriptor_entries--)
    {

        /* Get the length of that component.  */
        switch(*descriptor_structure++)
        {

        /* Check the size then build the component from the source and
           insert it into the target descriptor.  */
        case 4:

            /* Padding zeros so address is aligned.  */
            while((ALIGN_TYPE) descriptor & 3u)
                *descriptor++ =  0;

            /* Save the DW.  */
            *((ULONG *) descriptor) =  _ux_utility_long_get(raw_descriptor);
            raw_descriptor +=  4;
            descriptor += 4;
            break;

        case 2:

            /* Padding zeros so address is aligned.  */
            while((ALIGN_TYPE) descriptor & 1u)
                *descriptor++ =  0;

            /* Save the word.  */
            *((USHORT *) descriptor) = (USHORT) _ux_utility_short_get(raw_descriptor);
            raw_descriptor += 2;
            descriptor += 2;
            break;

        default:

            /* Save the byte.  */
            *((UCHAR *) descriptor) =  (UCHAR) *raw_descriptor;
            raw_descriptor++;
            descriptor ++;
        }
    }

    /* Return to caller.  */
    return;
}

/**************************************************************************/
/*                                                                        */
/*  FUNCTION                                               RELEASE        */
/*                                                                        */
/*    _ux_utility_descriptor_parse_size                   PORTABLE C      */
/*                                                           6.3.0        */
/*  AUTHOR                                                                */
/*                                                                        */
/*    Chaoqiong Xiao, Microsoft Corporation                               */
/*                                                                        */
/*  DESCRIPTION                                                           */
/*                                                                        */
/*    This function will calculate the size of a parsed USB descriptor.   */
/*                                                                        */
/*  INPUT                                                                 */
/*                                                                        */
/*    descriptor_structure                  Components of the descriptor  */
/*    descriptor_entries                    Number of entries in the      */
/*                                            descriptor                  */
/*    size_align_mask                       Size alignment mask           */
/*                                                                        */
/*  OUTPUT                                                                */
/*                                                                        */
/*    size                                  Size of the parsed descriptor */
/*                                                                        */
/*  CALLS                                                                 */
/*                                                                        */
/*    None                                                                */
/*                                                                        */
/*  CALLED BY                                                             */
/*                                                                        */
/*    USBX Components                                                     */
/*                                                                        */
/*  RELEASE HISTORY                                                       */
/*                                                                        */
/*    DATE              NAME                      DESCRIPTION             */
/*                                                                        */
/*  10-31-2023     Chaoqiong Xiao           Initial Version 6.3.0         */
/*                                                                        */
/**************************************************************************/
ULONG _ux_utility_descriptor_parse_size(UCHAR * descriptor_structure, UINT descriptor_entries, UINT size_align_mask)
{

ULONG           size = 0;
ULONG           entry_size;

    /* Loop on all the entries in this descriptor.  */
    while(descriptor_entries--)
    {

        /* Get entry size.  */
        entry_size = (ULONG)*descriptor_structure ++;

        /* Check the size then build the component from the source and
           insert it into the target descriptor.  */
        switch(entry_size)
        {

        case 4: /* Fall through.  */
        case 2:

            /* Padding zeros so address is aligned.  */
            while(size & (entry_size - 1))
                size++;

            /* Add to the size.  */
            size += entry_size;
            break;

        case 1:

            /* Add to the size.  */
            size += 1;
            break;

        default:

            /* Invalid entry size.  */
            return(0);
        }
    }

    /* Align the size.  */
    size = (size + size_align_mask) & (~size_align_mask);

    /* Return the size.  */
    return(size);
}
