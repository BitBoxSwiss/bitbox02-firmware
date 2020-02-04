#ifndef __BOOTLOADER_USB_H
#define __BOOTLOADER_USB_H

#include <stdbool.h>

// API return codes
#define OP_STATUS_OK ((uint8_t)0)
#define OP_STATUS_ERR ((uint8_t)'Z')
#define OP_STATUS_ERR_VERSION ((uint8_t)'V')
#define OP_STATUS_ERR_LEN ((uint8_t)'N')
#define OP_STATUS_ERR_MACRO ((uint8_t)'M')
#define OP_STATUS_ERR_WRITE ((uint8_t)'W')
#define OP_STATUS_ERR_CHECK ((uint8_t)'C')
#define OP_STATUS_ERR_ABORT ((uint8_t)'A')
#define OP_STATUS_ERR_ERASE ((uint8_t)'E')
#define OP_STATUS_ERR_LOAD_FLAG ((uint8_t)'L')
#define OP_STATUS_ERR_INVALID_CMD ((uint8_t)'I')
#define OP_STATUS_ERR_UNLOCK ((uint8_t)'U')
#define OP_STATUS_ERR_LOCK ((uint8_t)'K')

/**
 * Start the USB stack for the bootloader.
 */
bool bootloader_usb_start(void);

#endif // __BOOTLOADER_USB_H
