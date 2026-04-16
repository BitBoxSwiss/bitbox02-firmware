/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    gpu2d.c
  * @brief   This file provides code for the configuration
  *          of the GPU2D instances.
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
#include "gpu2d.h"

/* USER CODE BEGIN 0 */

/* USER CODE END 0 */

GPU2D_HandleTypeDef hgpu2d;

/* GPU2D init function */
void MX_GPU2D_Init(void)
{

  /* USER CODE BEGIN GPU2D_Init 0 */

  /* USER CODE END GPU2D_Init 0 */

  /* USER CODE BEGIN GPU2D_Init 1 */

  /* USER CODE END GPU2D_Init 1 */
  hgpu2d.Instance = GPU2D;
  if (HAL_GPU2D_Init(&hgpu2d) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN GPU2D_Init 2 */

  /* USER CODE END GPU2D_Init 2 */

}

void HAL_GPU2D_MspInit(GPU2D_HandleTypeDef* gpu2dHandle)
{

  if(gpu2dHandle->Instance==GPU2D)
  {
  /* USER CODE BEGIN GPU2D_MspInit 0 */

  /* USER CODE END GPU2D_MspInit 0 */
    /* GPU2D clock enable */
    __HAL_RCC_GPU2D_CLK_ENABLE();
    __HAL_RCC_DCACHE2_CLK_ENABLE();

    /* GPU2D interrupt Init */
    HAL_NVIC_SetPriority(GPU2D_IRQn, 0, 0);
    HAL_NVIC_EnableIRQ(GPU2D_IRQn);
    HAL_NVIC_SetPriority(GPU2D_ER_IRQn, 0, 0);
    HAL_NVIC_EnableIRQ(GPU2D_ER_IRQn);
  /* USER CODE BEGIN GPU2D_MspInit 1 */

  /* USER CODE END GPU2D_MspInit 1 */
  }
}

void HAL_GPU2D_MspDeInit(GPU2D_HandleTypeDef* gpu2dHandle)
{

  if(gpu2dHandle->Instance==GPU2D)
  {
  /* USER CODE BEGIN GPU2D_MspDeInit 0 */

  /* USER CODE END GPU2D_MspDeInit 0 */
    /* Peripheral clock disable */
    __HAL_RCC_GPU2D_CLK_DISABLE();
    __HAL_RCC_DCACHE2_CLK_DISABLE();

    /* GPU2D interrupt Deinit */
    HAL_NVIC_DisableIRQ(GPU2D_IRQn);
    HAL_NVIC_DisableIRQ(GPU2D_ER_IRQn);
  /* USER CODE BEGIN GPU2D_MspDeInit 1 */

  /* USER CODE END GPU2D_MspDeInit 1 */
  }
}

/* USER CODE BEGIN 1 */

/* USER CODE END 1 */
