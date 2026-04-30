/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    ux_device_customhid.h
  * @author  MCD Application Team
  * @brief   USBX Device Custom HID applicative header file
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2022 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */
/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __UX_DEVICE_CUSTOMHID_H__
#define __UX_DEVICE_CUSTOMHID_H__

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "ux_api.h"
#include "ux_device_class_hid.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

/* Exported types ------------------------------------------------------------*/
/* USER CODE BEGIN ET */

/* USER CODE END ET */

/* Exported constants --------------------------------------------------------*/
/* USER CODE BEGIN EC */

/* USER CODE END EC */

/* Exported macro ------------------------------------------------------------*/
/* USER CODE BEGIN EM */

/* USER CODE END EM */

/* Exported functions prototypes ---------------------------------------------*/
VOID USBD_Custom_HID_Activate(VOID *hid_instance);
VOID USBD_Custom_HID_Deactivate(VOID *hid_instance);
UINT USBD_Custom_HID_SetFeature(UX_SLAVE_CLASS_HID *hid_instance,
                                UX_SLAVE_CLASS_HID_EVENT *hid_event);
UINT USBD_Custom_HID_GetReport(UX_SLAVE_CLASS_HID *hid_instance,
                               UX_SLAVE_CLASS_HID_EVENT *hid_event);

#ifdef UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT
VOID USBD_Custom_HID_SetReport(struct UX_SLAVE_CLASS_HID_STRUCT *hid_instance);
ULONG USBD_Custom_HID_EventMaxNumber(VOID);
ULONG USBD_Custom_HID_EventMaxLength(VOID);
#endif /* UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT */

/* USER CODE BEGIN EFP */
UINT USBD_Custom_HID_SendReport(UCHAR *report, ULONG length);

#define BITBOX_USBX_CUSTOM_HID_READ_OK        0U
#define BITBOX_USBX_CUSTOM_HID_READ_EMPTY     1U
#define BITBOX_USBX_CUSTOM_HID_READ_OVERFLOW  2U
#define BITBOX_USBX_CUSTOM_HID_READ_DISABLED  3U

int bitbox_usbx_custom_hid_enabled(void);
int bitbox_usbx_custom_hid_read_ready(void);
int bitbox_usbx_custom_hid_can_write(void);
UINT bitbox_usbx_custom_hid_read(UCHAR *dst, ULONG max_length, ULONG *out_length);
UINT bitbox_usbx_custom_hid_write(const UCHAR *report, ULONG length);

/* USER CODE END EFP */

/* Private defines -----------------------------------------------------------*/
/* USER CODE BEGIN PD */

/* USER CODE END PD */

/* USER CODE BEGIN 1 */

/* USER CODE END 1 */

#ifdef __cplusplus
}
#endif
#endif  /* __UX_DEVICE_CUSTOMHID_H__ */
