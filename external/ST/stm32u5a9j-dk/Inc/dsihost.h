/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    dsihost.h
  * @brief   This file contains all the function prototypes for
  *          the dsihost.c file
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
#ifndef __DSIHOST_H__
#define __DSIHOST_H__

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "board.h"

/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

extern DSI_HandleTypeDef hdsi;

/* USER CODE BEGIN Private defines */

/* USER CODE END Private defines */

void MX_DSIHOST_DSI_Init(void);

/* USER CODE BEGIN Prototypes */

/* USER CODE END Prototypes */

#ifdef __cplusplus
}
#endif

#endif /* __DSIHOST_H__ */

