/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    app_usbx_device.c
  * @author  MCD Application Team
  * @brief   USBX Device applicative file
  ******************************************************************************
    * @attention
  *
  * Copyright (c) 2026 STMicroelectronics.
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
#include "app_usbx_device.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */
#include "board.h"
#include "usb_otg.h"
#include "ux_dcd_stm32.h"

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

/* USER CODE BEGIN UX_Device_Memory_Buffer */

/* USER CODE END UX_Device_Memory_Buffer */
#if defined ( __ICCARM__ )
#pragma data_alignment=4
#endif
__ALIGN_BEGIN static UCHAR ux_device_byte_pool_buffer[UX_DEVICE_APP_MEM_POOL_SIZE] __ALIGN_END;

static ULONG hid_custom_interface_number;
static ULONG hid_custom_configuration_number;
static UX_SLAVE_CLASS_HID_PARAMETER custom_hid_parameter;

/* USER CODE BEGIN PV */

/* USER CODE END PV */

/* Private function prototypes -----------------------------------------------*/
static UINT USBD_ChangeFunction(ULONG Device_State);
/* USER CODE BEGIN PFP */

/* USER CODE END PFP */

/**
  * @brief  Application USBX Device Initialization.
  * @param  none
  * @retval status
  */

UINT MX_USBX_Device_Init(VOID)
{
   UINT ret = UX_SUCCESS;
  UCHAR *device_framework_high_speed;
  UCHAR *device_framework_full_speed;
  ULONG device_framework_hs_length;
  ULONG device_framework_fs_length;
  ULONG string_framework_length;
  ULONG language_id_framework_length;
  UCHAR *string_framework;
  UCHAR *language_id_framework;

  UCHAR *pointer;

  /* USER CODE BEGIN MX_USBX_Device_Init0 */

  /* USER CODE END MX_USBX_Device_Init0 */
  pointer = ux_device_byte_pool_buffer;

  /* Initialize USBX Memory */
  if (ux_system_initialize(pointer, USBX_DEVICE_MEMORY_STACK_SIZE, UX_NULL, 0) != UX_SUCCESS)
  {
    /* USER CODE BEGIN USBX_SYSTEM_INITIALIZE_ERROR */
    return UX_ERROR;
    /* USER CODE END USBX_SYSTEM_INITIALIZE_ERROR */
  }

  /* Get Device Framework High Speed and get the length */
  device_framework_high_speed = USBD_Get_Device_Framework_Speed(USBD_HIGH_SPEED,
                                                                &device_framework_hs_length);

  /* Get Device Framework Full Speed and get the length */
  device_framework_full_speed = USBD_Get_Device_Framework_Speed(USBD_FULL_SPEED,
                                                                &device_framework_fs_length);

  /* Get String Framework and get the length */
  string_framework = USBD_Get_String_Framework(&string_framework_length);

  /* Get Language Id Framework and get the length */
  language_id_framework = USBD_Get_Language_Id_Framework(&language_id_framework_length);

  /* Install the device portion of USBX */
  if (ux_device_stack_initialize(device_framework_high_speed,
                                 device_framework_hs_length,
                                 device_framework_full_speed,
                                 device_framework_fs_length,
                                 string_framework,
                                 string_framework_length,
                                 language_id_framework,
                                 language_id_framework_length,
                                 USBD_ChangeFunction) != UX_SUCCESS)
  {
    /* USER CODE BEGIN USBX_DEVICE_INITIALIZE_ERROR */
    return UX_ERROR;
    /* USER CODE END USBX_DEVICE_INITIALIZE_ERROR */
  }

  /* Initialize the hid custom class parameters for the device */
  custom_hid_parameter.ux_slave_class_hid_instance_activate         = USBD_Custom_HID_Activate;
  custom_hid_parameter.ux_slave_class_hid_instance_deactivate       = USBD_Custom_HID_Deactivate;
  custom_hid_parameter.ux_device_class_hid_parameter_report_address = USBD_HID_ReportDesc(INTERFACE_HID_CUSTOM);
  custom_hid_parameter.ux_device_class_hid_parameter_report_length  = USBD_HID_ReportDesc_length(INTERFACE_HID_CUSTOM);
  custom_hid_parameter.ux_device_class_hid_parameter_report_id      = UX_FALSE;
  custom_hid_parameter.ux_device_class_hid_parameter_callback       = USBD_Custom_HID_SetFeature;
  custom_hid_parameter.ux_device_class_hid_parameter_get_callback   = USBD_Custom_HID_GetReport;
#ifdef UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT
  custom_hid_parameter.ux_device_class_hid_parameter_receiver_initialize       = ux_device_class_hid_receiver_initialize;
  custom_hid_parameter.ux_device_class_hid_parameter_receiver_event_max_number = USBD_Custom_HID_EventMaxNumber();
  custom_hid_parameter.ux_device_class_hid_parameter_receiver_event_max_length = USBD_Custom_HID_EventMaxLength();
  custom_hid_parameter.ux_device_class_hid_parameter_receiver_event_callback   = USBD_Custom_HID_SetReport;
#endif /* UX_DEVICE_CLASS_HID_INTERRUPT_OUT_SUPPORT */

  /* USER CODE BEGIN CUSTOM_HID_PARAMETER */

  /* USER CODE END CUSTOM_HID_PARAMETER */

  /* Get Custom hid configuration number */
  hid_custom_configuration_number = USBD_Get_Configuration_Number(CLASS_TYPE_HID, INTERFACE_HID_CUSTOM);

  /* Find Custom hid interface number */
  hid_custom_interface_number = USBD_Get_Interface_Number(CLASS_TYPE_HID, INTERFACE_HID_CUSTOM);

  /* Initialize the device hid custom class */
  if (ux_device_stack_class_register(_ux_system_slave_class_hid_name,
                                     ux_device_class_hid_entry,
                                     hid_custom_configuration_number,
                                     hid_custom_interface_number,
                                     &custom_hid_parameter) != UX_SUCCESS)
  {
  /* USER CODE BEGIN USBX_DEVICE_HID_CUSTOM_REGISTER_ERROR */
  return UX_ERROR;
  /* USER CODE END USBX_DEVICE_HID_CUSTOM_REGISTER_ERROR */
  }

  /* USER CODE BEGIN MX_USBX_Device_Init1 */

  MX_USB_OTG_HS_PCD_Init();

  
/* Set the RX Fifo */

HAL_PCDEx_SetRxFiFo(&hpcd_USB_OTG_HS, 0x200);



/* Set the TX Fifo for the Control EP 0 */

HAL_PCDEx_SetTxFiFo(&hpcd_USB_OTG_HS, 0, 0x40);



/* Set the TX Fifo for the HID Mouse EP 1 */

HAL_PCDEx_SetTxFiFo(&hpcd_USB_OTG_HS, 1, 0x100);


/* Link the USB drivers with the USBX DCD and check if it return error */

if(ux_dcd_stm32_initialize((ULONG)USB_OTG_HS, (ULONG)&hpcd_USB_OTG_HS) != UX_SUCCESS)

{


          Error_Handler();

}


/* Start the PCD Peripheral */

HAL_PCD_Start(&hpcd_USB_OTG_HS);

  /* USER CODE END MX_USBX_Device_Init1 */

  return ret;
}

/**
  * @brief  _ux_utility_interrupt_disable
  *         USB utility interrupt disable.
  * @param  none
  * @retval none
  */
ALIGN_TYPE _ux_utility_interrupt_disable(VOID)
{
  UINT interrupt_save;
  /* USER CODE BEGIN _ux_utility_interrupt_disable */
  interrupt_save = __get_PRIMASK();
  __disable_irq();
  /* USER CODE END _ux_utility_interrupt_disable */

  return interrupt_save;
}

/**
  * @brief  _ux_utility_interrupt_restore
  *         USB utility interrupt restore.
  * @param  flags
  * @retval none
  */
VOID _ux_utility_interrupt_restore(ALIGN_TYPE flags)
{

  /* USER CODE BEGIN _ux_utility_interrupt_restore */
  __set_PRIMASK(flags);
  /* USER CODE END _ux_utility_interrupt_restore */
}

/**
  * @brief  _ux_utility_time_get
  *         Get Time Tick for host timing.
  * @param  none
  * @retval time tick
  */
ULONG _ux_utility_time_get(VOID)
{
  ULONG time_tick = 0U;

  /* USER CODE BEGIN _ux_utility_time_get */

  time_tick = HAL_GetTick();

  /* USER CODE END _ux_utility_time_get */

  return time_tick;
}

/**
  * @brief  USBD_ChangeFunction
  *         This function is called when the device state changes.
  * @param  Device_State: USB Device State
  * @retval status
  */
static UINT USBD_ChangeFunction(ULONG Device_State)
{
   UINT status = UX_SUCCESS;

  /* USER CODE BEGIN USBD_ChangeFunction0 */

  /* USER CODE END USBD_ChangeFunction0 */

  switch (Device_State)
  {
    case UX_DEVICE_ATTACHED:

      /* USER CODE BEGIN UX_DEVICE_ATTACHED */

      /* USER CODE END UX_DEVICE_ATTACHED */

      break;

    case UX_DEVICE_REMOVED:

      /* USER CODE BEGIN UX_DEVICE_REMOVED */

      /* USER CODE END UX_DEVICE_REMOVED */

      break;

    case UX_DCD_STM32_DEVICE_CONNECTED:

      /* USER CODE BEGIN UX_DCD_STM32_DEVICE_CONNECTED */

      /* USER CODE END UX_DCD_STM32_DEVICE_CONNECTED */

      break;

    case UX_DCD_STM32_DEVICE_DISCONNECTED:

      /* USER CODE BEGIN UX_DCD_STM32_DEVICE_DISCONNECTED */

      /* USER CODE END UX_DCD_STM32_DEVICE_DISCONNECTED */

      break;

    case UX_DCD_STM32_DEVICE_SUSPENDED:

      /* USER CODE BEGIN UX_DCD_STM32_DEVICE_SUSPENDED */

      /* USER CODE END UX_DCD_STM32_DEVICE_SUSPENDED */

      break;

    case UX_DCD_STM32_DEVICE_RESUMED:

      /* USER CODE BEGIN UX_DCD_STM32_DEVICE_RESUMED */

      /* USER CODE END UX_DCD_STM32_DEVICE_RESUMED */

      break;

    case UX_DCD_STM32_SOF_RECEIVED:

      /* USER CODE BEGIN UX_DCD_STM32_SOF_RECEIVED */

      /* USER CODE END UX_DCD_STM32_SOF_RECEIVED */

      break;

    default:

      /* USER CODE BEGIN DEFAULT */

      /* USER CODE END DEFAULT */

      break;

  }

  /* USER CODE BEGIN USBD_ChangeFunction1 */

  /* USER CODE END USBD_ChangeFunction1 */

  return status;
}
/* USER CODE BEGIN 1 */

void USBX_Device_Process(void)
{
    ux_device_stack_tasks_run();
}

/* USER CODE END 1 */
