/*
 * Code generated from Atmel Start.
 *
 * This file will be overwritten when reconfiguring your Atmel Start project.
 * Please copy examples or other code you want to keep to a separate file
 * to avoid losing it when reconfiguring.
 */
#ifndef DRIVER_INIT_INCLUDED
#define DRIVER_INIT_INCLUDED

#include "atmel_start_pins.h"

#ifdef __cplusplus
extern "C" {
#endif

#include <hal_atomic.h>
#include <hal_delay.h>
#include <hal_gpio.h>
#include <hal_init.h>
#include <hal_io.h>
#include <hal_sleep.h>

#include <hal_flash.h>

#include <hal_pac.h>

#include <string.h>
#include "CryptoLib_typedef_pb.h"
#include "CryptoLib_mapping_pb.h"
#include "CryptoLib_cf_pb.h"
#include "CryptoLib_Headers_pb.h"

#include <hal_timer.h>

#include <hal_usart_sync.h>
#include <hal_usart_async.h>
#include <hal_spi_m_sync.h>

#include <hal_i2c_m_sync.h>

#include <hal_mci_sync.h>

#include <hal_rand_sync.h>

#include "hal_usb_device.h"

extern struct flash_descriptor FLASH_0;

extern PPUKCL_PARAM            pvPUKCLParam;
extern PUKCL_PARAM             PUKCLParam;
extern struct timer_descriptor Timer;

extern struct usart_sync_descriptor  USART_0;
extern struct usart_async_descriptor USART;
extern struct spi_m_sync_descriptor  SPI_0;

extern struct i2c_m_sync_desc I2C_0;

extern struct mci_sync_desc MCI_0;

extern struct rand_sync_desc RAND_0;

void FLASH_0_init(void);
void FLASH_0_CLOCK_init(void);

void USART_0_PORT_init(void);
void USART_0_CLOCK_init(void);
void USART_0_init(void);

void USART_PORT_init(void);
void USART_CLOCK_init(void);
void USART_init(void);

void SPI_0_PORT_init(void);
void SPI_0_CLOCK_init(void);
void SPI_0_init(void);

void I2C_0_CLOCK_init(void);
void I2C_0_init(void);
void I2C_0_PORT_init(void);

void MCI_0_PORT_init(void);
void MCI_0_CLOCK_init(void);
void MCI_0_init(void);

void RAND_0_CLOCK_init(void);
void RAND_0_init(void);

void USB_0_CLOCK_init(void);
void USB_0_init(void);

/**
 * \brief Perform system initialization, initialize pins and clocks for
 * peripherals
 */
void system_init(void);

#ifdef __cplusplus
}
#endif
#endif // DRIVER_INIT_INCLUDED
