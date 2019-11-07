#ifndef __USART_HWW_SETUP_H
#define __USART_HWW_SETUP_H

/**
 * Interrupt handlers for U2F-over-USART.
 */

#include <hal_usart_async.h>

/**
 * Registers the read and write callbacks and start listening for data.
 */
void usart_hww_init(struct usart_async_descriptor* desc);

#endif // __USART_HWW_SETUP_H
