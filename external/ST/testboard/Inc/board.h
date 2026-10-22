/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : board.h
  * @brief          : Header for board.c file.
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
#ifndef __BOARD_H
#define __BOARD_H

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
#define ACC_INT1_Pin GPIO_PIN_0
#define ACC_INT1_GPIO_Port GPIOF
#define ACC_INT2_Pin GPIO_PIN_1
#define ACC_INT2_GPIO_Port GPIOF
#define OSC_EN_Pin GPIO_PIN_0
#define OSC_EN_GPIO_Port GPIOC
#define NFC_IRQ_Pin GPIO_PIN_2
#define NFC_IRQ_GPIO_Port GPIOC
#define PWDN_Pin GPIO_PIN_3
#define PWDN_GPIO_Port GPIOC
#define TP_RST_Pin GPIO_PIN_2
#define TP_RST_GPIO_Port GPIOB
#define BLE_RST_Pin GPIO_PIN_0
#define BLE_RST_GPIO_Port GPIOG
#define uSD_CardDet_Pin GPIO_PIN_1
#define uSD_CardDet_GPIO_Port GPIOG
#define nPM_INT_Pin GPIO_PIN_10
#define nPM_INT_GPIO_Port GPIOD
#define nPM_RST_Pin GPIO_PIN_11
#define nPM_RST_GPIO_Port GPIOD
#define S1E1_Pin GPIO_PIN_12
#define S1E1_GPIO_Port GPIOD
#define S1E2_Pin GPIO_PIN_13
#define S1E2_GPIO_Port GPIOD
#define S1E3_Pin GPIO_PIN_14
#define S1E3_GPIO_Port GPIOD
#define S1E4_Pin GPIO_PIN_15
#define S1E4_GPIO_Port GPIOD
#define SLS_RST_Pin GPIO_PIN_0
#define SLS_RST_GPIO_Port GPIOD
#define RAM_RST_Pin GPIO_PIN_1
#define RAM_RST_GPIO_Port GPIOD
#define CAM_RST_Pin GPIO_PIN_4
#define CAM_RST_GPIO_Port GPIOD
#define LCD_RST_Pin GPIO_PIN_5
#define LCD_RST_GPIO_Port GPIOD
#define LRA_ERM_Pin GPIO_PIN_9
#define LRA_ERM_GPIO_Port GPIOG
#define MOTOR_PWM_Pin GPIO_PIN_10
#define MOTOR_PWM_GPIO_Port GPIOG
#define MOTOR_EN_Pin GPIO_PIN_12
#define MOTOR_EN_GPIO_Port GPIOG
#define BL_PWM_Pin GPIO_PIN_8
#define BL_PWM_GPIO_Port GPIOB
#define BL_EN_Pin GPIO_PIN_9
#define BL_EN_GPIO_Port GPIOB

/* USER CODE BEGIN Private defines */

/* USER CODE END Private defines */

#ifdef __cplusplus
}
#endif

#endif /* __BOARD_H */
