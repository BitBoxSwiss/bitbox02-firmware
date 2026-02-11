/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : main.h
  * @brief          : Header for main.c file.
  *                   This file contains the common defines of the application.
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

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __MAIN_H
#define __MAIN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "stm32u5xx_hal.h"

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
void Error_Handler(void);

/* USER CODE BEGIN EFP */

/* USER CODE END EFP */

/* Private defines -----------------------------------------------------------*/
#define LED_GREEN_Pin GPIO_PIN_0
#define LED_GREEN_GPIO_Port GPIOE
#define LED_RED_Pin GPIO_PIN_1
#define LED_RED_GPIO_Port GPIOE
#define TOF_INTN_Pin GPIO_PIN_5
#define TOF_INTN_GPIO_Port GPIOB
#define DSI_RESETn_Pin GPIO_PIN_5
#define DSI_RESETn_GPIO_Port GPIOD
#define eMMC_RSTn_Pin GPIO_PIN_6
#define eMMC_RSTn_GPIO_Port GPIOH
#define DSI_BL_CTRL_Pin GPIO_PIN_6
#define DSI_BL_CTRL_GPIO_Port GPIOI
#define T_VCP_RX_Pin GPIO_PIN_10
#define T_VCP_RX_GPIO_Port GPIOA
#define T_VCP_TX_Pin GPIO_PIN_9
#define T_VCP_TX_GPIO_Port GPIOA
#define USER_Button_Pin GPIO_PIN_13
#define USER_Button_GPIO_Port GPIOC
#define TEMP_INTN_Pin GPIO_PIN_2
#define TEMP_INTN_GPIO_Port GPIOF
#define DSI_TOUCH_INT_Pin GPIO_PIN_8
#define DSI_TOUCH_INT_GPIO_Port GPIOE
#define VBUS_SENSE_Pin GPIO_PIN_1
#define VBUS_SENSE_GPIO_Port GPIOG
#define UCPD_FLT_Pin GPIO_PIN_12
#define UCPD_FLT_GPIO_Port GPIOE
#define UCPD_ADC2_Pin GPIO_PIN_15
#define UCPD_ADC2_GPIO_Port GPIOF
#define UCPD_DBn_Pin GPIO_PIN_9
#define UCPD_DBn_GPIO_Port GPIOE
#define UCPD_ADC1_Pin GPIO_PIN_0
#define UCPD_ADC1_GPIO_Port GPIOG
#define TOF_LPN_Pin GPIO_PIN_14
#define TOF_LPN_GPIO_Port GPIOE

/* USER CODE BEGIN Private defines */

/* USER CODE END Private defines */

#ifdef __cplusplus
}
#endif

#endif /* __MAIN_H */
