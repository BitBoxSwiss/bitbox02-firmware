/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    ux_device_customhid.c
  * @author  MCD Application Team
  * @brief   USBX Device Custom HID applicative source file
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

/* Includes ------------------------------------------------------------------*/
#include "ux_device_customhid.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

#include "board.h"
#include "ux_system.h"

/* USER CODE END Includes */

/* Private typedef -----------------------------------------------------------*/
/* USER CODE BEGIN PTD */

/* USER CODE END PTD */

/* Private define ------------------------------------------------------------*/
/* USER CODE BEGIN PD */

/* USER CODE END PD */

/* Private macro -------------------------------------------------------------*/
/* USER CODE BEGIN PM */

/* USER CODE END PM */

/* Private variables ---------------------------------------------------------*/
/* USER CODE BEGIN PV */
static UX_SLAVE_CLASS_HID *custom_hid = UX_NULL;

/* USER CODE END PV */

/* Private function prototypes -----------------------------------------------*/
/* USER CODE BEGIN PFP */

/* USER CODE END PFP */

/* Private user code ---------------------------------------------------------*/
/* USER CODE BEGIN 0 */

static int _custom_hid_enabled(void)
{
  if ((custom_hid == UX_NULL) || (_ux_system_slave == UX_NULL))
  {
    return 0;
  }

  return _ux_system_slave->ux_system_slave_device.ux_slave_device_state == UX_DEVICE_CONFIGURED;
}

static int _custom_hid_read_ready(void)
{
  UX_DEVICE_CLASS_HID_RECEIVER *receiver;
  UX_DEVICE_CLASS_HID_RECEIVED_EVENT *pos;

  if (!_custom_hid_enabled())
  {
    return 0;
  }

  receiver = custom_hid->ux_device_class_hid_receiver;
  if (receiver == UX_NULL)
  {
    return 0;
  }

  pos = receiver->ux_device_class_hid_receiver_event_read_pos;
  return pos->ux_device_class_hid_received_event_length != 0U;
}

/* USER CODE END 0 */

/**
  * @brief  USBD_Custom_HID_Activate
  *         This function is called when insertion of a Custom HID device.
  * @param  hid_instance: Pointer to the hid class instance.
  * @retval none
  */
VOID USBD_Custom_HID_Activate(VOID *hid_instance)
{
  /* USER CODE BEGIN USBD_Custom_HID_Activate */
  custom_hid = (UX_SLAVE_CLASS_HID *)hid_instance;
  /* USER CODE END USBD_Custom_HID_Activate */

  return;
}

/**
  * @brief  USBD_Custom_HID_Deactivate
  *         This function is called when extraction of a Custom HID device.
  * @param  hid_instance: Pointer to the hid class instance.
  * @retval none
  */
VOID USBD_Custom_HID_Deactivate(VOID *hid_instance)
{
  /* USER CODE BEGIN USBD_Custom_HID_Deactivate */
  UX_PARAMETER_NOT_USED(hid_instance);
  custom_hid = UX_NULL;
  /* USER CODE END USBD_Custom_HID_Deactivate */

  return;
}

/**
  * @brief  USBD_Custom_HID_SetFeature
  *         This function is invoked when the host sends a HID SET_REPORT
  *         to the application over Endpoint 0 (Set Feature).
  * @param  hid_instance: Pointer to the hid class instance.
  * @param  hid_event: Pointer to structure of the hid event.
  * @retval status
  */
UINT USBD_Custom_HID_SetFeature(UX_SLAVE_CLASS_HID *hid_instance,
                                UX_SLAVE_CLASS_HID_EVENT *hid_event)
{
  UINT status = UX_SUCCESS;

  /* USER CODE BEGIN USBD_Custom_HID_SetFeature */
  UX_PARAMETER_NOT_USED(hid_instance);
  UX_PARAMETER_NOT_USED(hid_event);
  /* USER CODE END USBD_Custom_HID_SetFeature */

  return status;
}

/**
  * @brief  USBD_Custom_HID_GetReport
  *         This function is invoked when host is requesting event through
  *         control GET_REPORT request.
  * @param  hid_instance: Pointer to the hid class instance.
  * @param  hid_event: Pointer to structure of the hid event.
  * @retval status
  */
UINT USBD_Custom_HID_GetReport(UX_SLAVE_CLASS_HID *hid_instance,
                               UX_SLAVE_CLASS_HID_EVENT *hid_event)
{
  UINT status = UX_SUCCESS;

  /* USER CODE BEGIN USBD_Custom_HID_GetReport */
  UX_PARAMETER_NOT_USED(hid_instance);
  UX_PARAMETER_NOT_USED(hid_event);
  /* USER CODE END USBD_Custom_HID_GetReport */

  return status;
}

#ifdef UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT

/**
  * @brief  USBD_Custom_HID_SetReport
  *         This function is invoked when the host sends a HID SET_REPORT
  *         to the application over Endpoint OUT (Set Report).
  * @param  hid_instance: Pointer to the hid class instance.
  * @retval none
  */
VOID USBD_Custom_HID_SetReport(struct UX_SLAVE_CLASS_HID_STRUCT *hid_instance)
{
  /* USER CODE BEGIN USBD_Custom_HID_SetReport */
  UX_PARAMETER_NOT_USED(hid_instance);
  /* Reports remain queued in USBX until Rust consumes them through the helper API below. */
  /* USER CODE END USBD_Custom_HID_SetReport */

  return;
}

/**
  * @brief  USBD_Custom_HID_EventMaxNumber
  *         This function to set receiver event max number parameter.
  * @param  none
  * @retval receiver event max number
  */
ULONG USBD_Custom_HID_EventMaxNumber(VOID)
{
  ULONG max_number = 0U;

  /* USER CODE BEGIN USBD_Custom_HID_EventMaxNumber */

  max_number = 1U;

  /* USER CODE END USBD_Custom_HID_EventMaxNumber */

  return max_number;
}

/**
  * @brief  USBD_Custom_HID_EventMaxLength
  *         This function to set receiver event max length parameter.
  * @param  none
  * @retval receiver event max length
  */
ULONG USBD_Custom_HID_EventMaxLength(VOID)
{
  ULONG max_length = 0U;

  /* USER CODE BEGIN USBD_Custom_HID_EventMaxLength */

  max_length = 64U;

  /* USER CODE END USBD_Custom_HID_EventMaxLength */

  return max_length;
}

#endif /* UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT */

/* USER CODE BEGIN 1 */
UINT USBD_Custom_HID_SendReport(UCHAR *report, ULONG length)
{
  UX_SLAVE_CLASS_HID_EVENT hid_event;

  if ((custom_hid == UX_NULL) || (report == UX_NULL) || (length == 0U) ||
      (length > UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH))
  {
    return UX_ERROR;
  }

  hid_event.ux_device_class_hid_event_report_id = 0U;
  hid_event.ux_device_class_hid_event_report_type = 0U;
  hid_event.ux_device_class_hid_event_length = length;
  _ux_utility_memory_copy(hid_event.ux_device_class_hid_event_buffer, report, length);

  return ux_device_class_hid_event_set(custom_hid, &hid_event);
}

int bitbox_usbx_custom_hid_enabled(void)
{
  return _custom_hid_enabled();
}

int bitbox_usbx_custom_hid_read_ready(void)
{
  return _custom_hid_read_ready();
}

int bitbox_usbx_custom_hid_can_write(void)
{
  if (!_custom_hid_enabled())
  {
    return 0;
  }

  return custom_hid->ux_device_class_hid_event_array_head ==
         custom_hid->ux_device_class_hid_event_array_tail;
}

UINT bitbox_usbx_custom_hid_read(UCHAR *dst, ULONG max_length, ULONG *out_length)
{
  UX_DEVICE_CLASS_HID_RECEIVED_EVENT hid_event;

  if ((dst == UX_NULL) || (out_length == UX_NULL))
  {
    return BITBOX_USBX_CUSTOM_HID_READ_DISABLED;
  }

  if (!_custom_hid_enabled())
  {
    return BITBOX_USBX_CUSTOM_HID_READ_DISABLED;
  }

  if (ux_device_class_hid_receiver_event_get(custom_hid, &hid_event) != UX_SUCCESS)
  {
    return BITBOX_USBX_CUSTOM_HID_READ_EMPTY;
  }

  *out_length = hid_event.ux_device_class_hid_received_event_length;
  if (*out_length > max_length)
  {
    ux_device_class_hid_receiver_event_free(custom_hid);
    return BITBOX_USBX_CUSTOM_HID_READ_OVERFLOW;
  }

  _ux_utility_memory_copy(
      dst,
      hid_event.ux_device_class_hid_received_event_data,
      hid_event.ux_device_class_hid_received_event_length);
  ux_device_class_hid_receiver_event_free(custom_hid);
  return BITBOX_USBX_CUSTOM_HID_READ_OK;
}

UINT bitbox_usbx_custom_hid_write(const UCHAR *report, ULONG length)
{
  UX_SLAVE_CLASS_HID_EVENT hid_event;

  if (!_custom_hid_enabled() || !bitbox_usbx_custom_hid_can_write() ||
      (report == UX_NULL) || (length == 0U) ||
      (length > UX_DEVICE_CLASS_HID_EVENT_BUFFER_LENGTH))
  {
    return UX_ERROR;
  }

  hid_event.ux_device_class_hid_event_report_id = 0U;
  hid_event.ux_device_class_hid_event_report_type = 0U;
  hid_event.ux_device_class_hid_event_length = length;
  _ux_utility_memory_copy(hid_event.ux_device_class_hid_event_buffer, report, length);

  return ux_device_class_hid_event_set(custom_hid, &hid_event);
}


/* USER CODE END 1 */
