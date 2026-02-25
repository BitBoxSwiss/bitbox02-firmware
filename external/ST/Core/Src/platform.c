/* USER CODE BEGIN Header */
/**
 ******************************************************************************
 * @file           : main.c
 * @brief          : Main program body
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
#include "platform.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */
#include <stdio.h>
#include <string.h>
#include <aps512xx.h>
#include <stdint.h>

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
ADC_HandleTypeDef hadc4;

DSI_HandleTypeDef hdsi;

GPU2D_HandleTypeDef hgpu2d;

XSPI_HandleTypeDef hxspi1;

I2C_HandleTypeDef hi2c3;
I2C_HandleTypeDef hi2c5;

LTDC_HandleTypeDef hltdc;

OSPI_HandleTypeDef hospi1;

MMC_HandleTypeDef hmmc1;

UART_HandleTypeDef huart1;

HCD_HandleTypeDef hhcd_USB_OTG_HS;

/* USER CODE BEGIN PV */

struct psram_config {
    uint8_t LatencyType;
    uint8_t BurstType;
    uint8_t BurstLength;
    uint8_t ReadLatencyCode;
    uint8_t WriteLatencyCode;
    uint8_t IOMode;
};

const struct psram_config psram_config = {
    .LatencyType = 0, /* 0 = Variable latency (default), 0x20 = Fixed */
    .BurstType = 0, /* 0 = Linear, 0x04 = Hybrid */
    .BurstLength = 1, /* 1 = 32 bytes (dcache line), Not applicable for Hybrid?  */
    .ReadLatencyCode =  0x10, /* 0x10 = RLC7 */
    .WriteLatencyCode =  0x20, /* 0x20 = WLC7 */
    .IOMode = 0, /* 0 = X8, 0x40 = X16 */
};

/* USER CODE END PV */

/* Private function prototypes -----------------------------------------------*/
void SystemClock_Config(void);
void PeriphCommonClock_Config(void);
static void SystemPower_Config(void);
static void MX_GPIO_Init(void);
static void MX_ADC4_Init(void);
static void MX_USART1_UART_Init(void);
static void MX_HSPI1_Init(void);
static void MX_I2C3_Init(void);
static void MX_I2C5_Init(void);
static void MX_ICACHE_Init(void);
static void MX_OCTOSPI1_Init(void);
static void MX_SDMMC1_MMC_Init(void);
static void MX_USB_OTG_HS_HCD_Init(void);
static void MX_GPU2D_Init(void);
static void MX_DSIHOST_DSI_Init(void);
static void MX_LTDC_Init(void);
/* USER CODE BEGIN PFP */

static void setPanelConfig(void);
static void print_s(char *str);
//static void print_hex(uint8_t* data, size_t data_len);
//static void print_hexln(uint8_t* data, size_t data_len);
static void memtest(uint32_t* addr, uint32_t words);

/* USER CODE END PFP */

/* Private user code ---------------------------------------------------------*/
/* USER CODE BEGIN 0 */

/* USER CODE END 0 */

/**
  * @brief  The application entry point.
  * @retval int
  */
int platform_init(void)
{

  /* USER CODE BEGIN 1 */

  /* USER CODE END 1 */

  /* MCU Configuration--------------------------------------------------------*/

  /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
  HAL_Init();

  /* USER CODE BEGIN Init */

  /* USER CODE END Init */

  /* Configure the System Power */
  SystemPower_Config();

  /* Configure the system clock */
  SystemClock_Config();

  /* Configure the peripherals common clocks */
  PeriphCommonClock_Config();

  /* USER CODE BEGIN SysInit */

  /* USER CODE END SysInit */

  /* Initialize all configured peripherals */
  MX_GPIO_Init();
  MX_ADC4_Init();
  MX_USART1_UART_Init();
  MX_HSPI1_Init();
  MX_I2C3_Init();
  MX_I2C5_Init();
  MX_ICACHE_Init();
  MX_OCTOSPI1_Init();
  MX_SDMMC1_MMC_Init();
  MX_USB_OTG_HS_HCD_Init();
  MX_GPU2D_Init();
  MX_DSIHOST_DSI_Init();
  MX_LTDC_Init();
  /* USER CODE BEGIN 2 */
  // ^- The call to MX_DSIHOST_DSI_Init() must be done before MX_LTDC_Init().
  
  setPanelConfig();

  static unsigned char tx_buf[] = "Hello World\r\n";

  /* USER CODE END 2 */
}

/**
  * @brief System Clock Configuration
  * @retval None
  */
void SystemClock_Config(void)
{
  RCC_OscInitTypeDef RCC_OscInitStruct = {0};
  RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};

  /** Configure the main internal regulator output voltage
  */
  if (HAL_PWREx_ControlVoltageScaling(PWR_REGULATOR_VOLTAGE_SCALE1) != HAL_OK)
  {
    Error_Handler();
  }

  /** Initializes the CPU, AHB and APB buses clocks
  */
  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI|RCC_OSCILLATORTYPE_HSE
                              |RCC_OSCILLATORTYPE_MSI;
  RCC_OscInitStruct.HSEState = RCC_HSE_ON;
  RCC_OscInitStruct.HSIState = RCC_HSI_ON;
  RCC_OscInitStruct.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;
  RCC_OscInitStruct.MSIState = RCC_MSI_ON;
  RCC_OscInitStruct.MSICalibrationValue = RCC_MSICALIBRATION_DEFAULT;
  RCC_OscInitStruct.MSIClockRange = RCC_MSIRANGE_0;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
  RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSE;
  RCC_OscInitStruct.PLL.PLLMBOOST = RCC_PLLMBOOST_DIV1;
  RCC_OscInitStruct.PLL.PLLM = 1;
  RCC_OscInitStruct.PLL.PLLN = 10;
  RCC_OscInitStruct.PLL.PLLP = 8;
  RCC_OscInitStruct.PLL.PLLQ = 2;
  RCC_OscInitStruct.PLL.PLLR = 1;
  RCC_OscInitStruct.PLL.PLLRGE = RCC_PLLVCIRANGE_1;
  RCC_OscInitStruct.PLL.PLLFRACN = 0;
  if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  /** Initializes the CPU, AHB and APB buses clocks
  */
  RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_HCLK|RCC_CLOCKTYPE_SYSCLK
                              |RCC_CLOCKTYPE_PCLK1|RCC_CLOCKTYPE_PCLK2
                              |RCC_CLOCKTYPE_PCLK3;
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV1;
  RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV1;
  RCC_ClkInitStruct.APB3CLKDivider = RCC_HCLK_DIV1;

  if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_4) != HAL_OK)
  {
    Error_Handler();
  }
}

/**
  * @brief Peripherals Common Clock Configuration
  * @retval None
  */
void PeriphCommonClock_Config(void)
{
  RCC_PeriphCLKInitTypeDef PeriphClkInit = {0};

  /** Initializes the common periph clock
  */
  PeriphClkInit.PeriphClockSelection = RCC_PERIPHCLK_LTDC|RCC_PERIPHCLK_DSI;
  PeriphClkInit.DsiClockSelection = RCC_DSICLKSOURCE_PLL3;
  PeriphClkInit.LtdcClockSelection = RCC_LTDCCLKSOURCE_PLL3;
  PeriphClkInit.PLL3.PLL3Source = RCC_PLLSOURCE_HSE;
  PeriphClkInit.PLL3.PLL3M = 1;
  PeriphClkInit.PLL3.PLL3N = 9;
  PeriphClkInit.PLL3.PLL3P = 3;
  PeriphClkInit.PLL3.PLL3Q = 2;
  PeriphClkInit.PLL3.PLL3R = 5;
  PeriphClkInit.PLL3.PLL3RGE = RCC_PLLVCIRANGE_1;
  PeriphClkInit.PLL3.PLL3FRACN = 3072;
  PeriphClkInit.PLL3.PLL3ClockOut = RCC_PLL3_DIVP|RCC_PLL3_DIVR;
  if (HAL_RCCEx_PeriphCLKConfig(&PeriphClkInit) != HAL_OK)
  {
    Error_Handler();
  }
}

/**
  * @brief Power Configuration
  * @retval None
  */
static void SystemPower_Config(void)
{

  /*
   * Disable the internal Pull-Up in Dead Battery pins of UCPD peripheral
   */
  HAL_PWREx_DisableUCPDDeadBattery();

  /*
   * Switch to SMPS regulator instead of LDO
   */
  if (HAL_PWREx_ConfigSupply(PWR_SMPS_SUPPLY) != HAL_OK)
  {
    Error_Handler();
  }
/* USER CODE BEGIN PWR */
/* USER CODE END PWR */
}

/**
  * @brief ADC4 Initialization Function
  * @param None
  * @retval None
  */
static void MX_ADC4_Init(void)
{

  /* USER CODE BEGIN ADC4_Init 0 */

  /* USER CODE END ADC4_Init 0 */

  ADC_ChannelConfTypeDef sConfig = {0};

  /* USER CODE BEGIN ADC4_Init 1 */

  /* USER CODE END ADC4_Init 1 */

  /** Common config
  */
  hadc4.Instance = ADC4;
  hadc4.Init.ClockPrescaler = ADC_CLOCK_ASYNC_DIV1;
  hadc4.Init.Resolution = ADC_RESOLUTION_12B;
  hadc4.Init.DataAlign = ADC_DATAALIGN_RIGHT;
  hadc4.Init.ScanConvMode = ADC4_SCAN_DISABLE;
  hadc4.Init.EOCSelection = ADC_EOC_SINGLE_CONV;
  hadc4.Init.LowPowerAutoPowerOff = ADC_LOW_POWER_NONE;
  hadc4.Init.LowPowerAutoWait = DISABLE;
  hadc4.Init.ContinuousConvMode = DISABLE;
  hadc4.Init.NbrOfConversion = 1;
  hadc4.Init.DiscontinuousConvMode = DISABLE;
  hadc4.Init.ExternalTrigConv = ADC_SOFTWARE_START;
  hadc4.Init.ExternalTrigConvEdge = ADC_EXTERNALTRIGCONVEDGE_NONE;
  hadc4.Init.DMAContinuousRequests = DISABLE;
  hadc4.Init.TriggerFrequencyMode = ADC_TRIGGER_FREQ_LOW;
  hadc4.Init.Overrun = ADC_OVR_DATA_PRESERVED;
  hadc4.Init.SamplingTimeCommon1 = ADC4_SAMPLETIME_1CYCLE_5;
  hadc4.Init.SamplingTimeCommon2 = ADC4_SAMPLETIME_1CYCLE_5;
  hadc4.Init.OversamplingMode = DISABLE;
  if (HAL_ADC_Init(&hadc4) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Regular Channel
  */
  sConfig.Channel = ADC_CHANNEL_6;
  sConfig.Rank = ADC4_REGULAR_RANK_1;
  sConfig.SamplingTime = ADC4_SAMPLINGTIME_COMMON_1;
  sConfig.OffsetNumber = ADC_OFFSET_NONE;
  sConfig.Offset = 0;
  if (HAL_ADC_ConfigChannel(&hadc4, &sConfig) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN ADC4_Init 2 */

  /* USER CODE END ADC4_Init 2 */

}

/**
  * @brief DSIHOST Initialization Function
  * @param None
  * @retval None
  */
static void MX_DSIHOST_DSI_Init(void)
{

  /* USER CODE BEGIN DSIHOST_Init 0 */

  /* USER CODE END DSIHOST_Init 0 */

  DSI_PLLInitTypeDef PLLInit = {0};
  DSI_HOST_TimeoutTypeDef HostTimeouts = {0};
  DSI_PHY_TimerTypeDef PhyTimings = {0};
  DSI_VidCfgTypeDef VidCfg = {0};

  /* USER CODE BEGIN DSIHOST_Init 1 */

  /* USER CODE END DSIHOST_Init 1 */
  hdsi.Instance = DSI;
  hdsi.Init.AutomaticClockLaneControl = DSI_AUTO_CLK_LANE_CTRL_DISABLE;
  hdsi.Init.TXEscapeCkdiv = 4;
  hdsi.Init.NumberOfLanes = DSI_TWO_DATA_LANES;
  hdsi.Init.PHYFrequencyRange = DSI_DPHY_FRANGE_240MHZ_320MHZ;
  hdsi.Init.PHYLowPowerOffset = PHY_LP_OFFSSET_0_CLKP;
  PLLInit.PLLNDIV = 16;
  PLLInit.PLLIDF = DSI_PLL_IN_DIV1;
  PLLInit.PLLODF = DSI_PLL_OUT_DIV2;
  PLLInit.PLLVCORange = DSI_DPHY_VCO_FRANGE_800MHZ_1GHZ;
  PLLInit.PLLChargePump = DSI_PLL_CHARGE_PUMP_2000HZ_4400HZ;
  PLLInit.PLLTuning = DSI_PLL_LOOP_FILTER_2000HZ_4400HZ;
  if (HAL_DSI_Init(&hdsi, &PLLInit) != HAL_OK)
  {
    Error_Handler();
  }
  HostTimeouts.TimeoutCkdiv = 1;
  HostTimeouts.HighSpeedTransmissionTimeout = 0;
  HostTimeouts.LowPowerReceptionTimeout = 0;
  HostTimeouts.HighSpeedReadTimeout = 0;
  HostTimeouts.LowPowerReadTimeout = 0;
  HostTimeouts.HighSpeedWriteTimeout = 0;
  HostTimeouts.HighSpeedWritePrespMode = DSI_HS_PM_DISABLE;
  HostTimeouts.LowPowerWriteTimeout = 0;
  HostTimeouts.BTATimeout = 0;
  if (HAL_DSI_ConfigHostTimeouts(&hdsi, &HostTimeouts) != HAL_OK)
  {
    Error_Handler();
  }
  PhyTimings.ClockLaneHS2LPTime = 8;
  PhyTimings.ClockLaneLP2HSTime = 0;
  PhyTimings.DataLaneHS2LPTime = 0;
  PhyTimings.DataLaneLP2HSTime = 0;
  PhyTimings.DataLaneMaxReadTime = 0;
  PhyTimings.StopWaitTime = 0;
  if (HAL_DSI_ConfigPhyTimer(&hdsi, &PhyTimings) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_DSI_ConfigFlowControl(&hdsi, DSI_FLOW_CONTROL_BTA) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_DSI_ConfigErrorMonitor(&hdsi, HAL_DSI_ERROR_NONE) != HAL_OK)
  {
    Error_Handler();
  }
  VidCfg.ColorCoding = DSI_RGB666;
  VidCfg.LooselyPacked = DSI_LOOSELY_PACKED_DISABLE;
  VidCfg.Mode = DSI_VID_MODE_BURST;
  VidCfg.PacketSize = 480;
  VidCfg.NumberOfChunks = 0;
  VidCfg.NullPacketSize = 0;
  VidCfg.HSPolarity = DSI_HSYNC_ACTIVE_LOW;
  VidCfg.VSPolarity = DSI_VSYNC_ACTIVE_LOW;
  VidCfg.DEPolarity = DSI_DATA_ENABLE_ACTIVE_HIGH;
  VidCfg.HorizontalSyncActive = 68;
  VidCfg.HorizontalBackPorch = 67;
  VidCfg.HorizontalLine = 985;
  VidCfg.VerticalSyncActive = 21;
  VidCfg.VerticalBackPorch = 16;
  VidCfg.VerticalFrontPorch = 12;
  VidCfg.VerticalActive = 800;
  VidCfg.LPCommandEnable = DSI_LP_COMMAND_ENABLE;
  VidCfg.LPLargestPacketSize = 0;
  VidCfg.LPVACTLargestPacketSize = 0;
  VidCfg.LPHorizontalFrontPorchEnable = DSI_LP_HFP_ENABLE;
  VidCfg.LPHorizontalBackPorchEnable = DSI_LP_HBP_ENABLE;
  VidCfg.LPVerticalActiveEnable = DSI_LP_VACT_ENABLE;
  VidCfg.LPVerticalFrontPorchEnable = DSI_LP_VFP_ENABLE;
  VidCfg.LPVerticalBackPorchEnable = DSI_LP_VBP_ENABLE;
  VidCfg.LPVerticalSyncActiveEnable = DSI_LP_VSYNC_ENABLE;
  VidCfg.FrameBTAAcknowledgeEnable = DSI_FBTAA_ENABLE;
  if (HAL_DSI_ConfigVideoMode(&hdsi, &VidCfg) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_DSI_SetGenericVCID(&hdsi, 0) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN DSIHOST_Init 2 */

  
  RCC_PeriphCLKInitTypeDef  DSIPHYInitPeriph;

  /* Switch to DSI PHY PLL clock */
  DSIPHYInitPeriph.PeriphClockSelection = RCC_PERIPHCLK_DSI;
  DSIPHYInitPeriph.DsiClockSelection    = RCC_DSICLKSOURCE_DSIPHY;

  HAL_RCCEx_PeriphCLKConfig(&DSIPHYInitPeriph);

  HAL_Delay(15);
  HAL_GPIO_WritePin(DSI_RESETn_GPIO_Port, DSI_RESETn_Pin, GPIO_PIN_SET);
  HAL_Delay(150);




  /* USER CODE END DSIHOST_Init 2 */

}

/**
  * @brief GPU2D Initialization Function
  * @param None
  * @retval None
  */
static void MX_GPU2D_Init(void)
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

/**
  * @brief HSPI1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_HSPI1_Init(void)
{

  /* USER CODE BEGIN HSPI1_Init 0 */

  /* USER CODE END HSPI1_Init 0 */

  /* USER CODE BEGIN HSPI1_Init 1 */

  hxspi1.Init.DelayBlockBypass = HAL_XSPI_DELAY_BLOCK_ON;
  // Broken?
  //uint32_t hspi_clk = HAL_RCCEx_GetPeriphCLKFreq(RCC_PERIPHCLK_HSPI);
  uint32_t hspi_clk = 200000000U;
  uint8_t clock_prescaler = 1;

  /* USER CODE END HSPI1_Init 1 */
  /* HSPI1 parameter configuration*/
  hxspi1.Instance = HSPI1;
  hxspi1.Init.FifoThresholdByte = 2;
  hxspi1.Init.MemoryMode = HAL_XSPI_SINGLE_MEM;
  hxspi1.Init.MemoryType = HAL_XSPI_MEMTYPE_APMEM;
  hxspi1.Init.MemorySize = HAL_XSPI_SIZE_512MB;
  hxspi1.Init.ChipSelectHighTimeCycle = 1;
  hxspi1.Init.FreeRunningClock = HAL_XSPI_FREERUNCLK_DISABLE;
  hxspi1.Init.ClockMode = HAL_XSPI_CLOCK_MODE_0;
  hxspi1.Init.WrapSize = HAL_XSPI_WRAP_32_BYTES;
  hxspi1.Init.ClockPrescaler = clock_prescaler;
  hxspi1.Init.SampleShifting = HAL_XSPI_SAMPLE_SHIFT_NONE;
  hxspi1.Init.DelayHoldQuarterCycle = HAL_XSPI_DHQC_DISABLE;
  hxspi1.Init.ChipSelectBoundary = HAL_XSPI_BONDARYOF_2KB;
  hxspi1.Init.MaxTran = 0;
  hxspi1.Init.Refresh = ((2U * (hspi_clk / (clock_prescaler + 1U))) / 1000000U) - 4U;
  if (HAL_XSPI_Init(&hxspi1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN HSPI1_Init 2 */
  {
      if(APS512XX_Reset(&hxspi1) != APS512XX_OK) {
          Error_Handler();
      }
      // APS read/write latanecies default to 5
      // Configure RAM
      uint8_t reg[2];

      // Modify MR0 (read latency)
      if(APS512XX_ReadReg(&hxspi1, APS512XX_MR0_ADDRESS, reg, APS512XX_READ_REG_LATENCY(APS512XX_READ_LATENCY_5)) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr0: ");
      //print_hexln(reg, 1);
      MODIFY_REG(reg[0], APS512XX_MR0_READ_LATENCY_CODE, APS512XX_MR0_RLC_7);
      if(APS512XX_WriteReg(&hxspi1, APS512XX_MR0_ADDRESS, reg[0]) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr0: ");
      //print_hexln(reg, 1);

      // Modify MR4 (write latency)
      if(APS512XX_ReadReg(&hxspi1, APS512XX_MR4_ADDRESS, reg, APS512XX_READ_REG_LATENCY(APS512XX_READ_LATENCY_7)) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr4: ");
      //print_hexln(reg, 1);
      MODIFY_REG(reg[0], APS512XX_MR4_WRITE_LATENCY_CODE, APS512XX_MR4_WLC_7);
      if(APS512XX_WriteReg(&hxspi1, APS512XX_MR4_ADDRESS, reg[0]) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr4: ");
      //print_hexln(reg, 1);

      // Modify MR8 (burst length)
      if(APS512XX_ReadReg(&hxspi1, APS512XX_MR8_ADDRESS, reg, APS512XX_READ_REG_LATENCY(APS512XX_READ_LATENCY_7)) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr8: ");
      //print_hexln(reg, 1);
      MODIFY_REG(reg[0], APS512XX_MR8_BL, APS512XX_MR8_BL_32_BYTES);
      if(APS512XX_WriteReg(&hxspi1, APS512XX_MR8_ADDRESS, reg[0]) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr8: ");
      //print_hexln(reg, 1);

      clock_prescaler = 3;
      hxspi1.Init.ClockPrescaler = clock_prescaler;
      hxspi1.Init.Refresh = ((2U * (hspi_clk / (clock_prescaler + 1U))) / 1000000U) - 4U;

      //Increase clock frequency
      //HAL_XSPI_SetClockPrescaler(&hxspi1, 0);
      if(HAL_XSPI_DeInit(&hxspi1) != HAL_OK) {
          Error_Handler();
      }
      if(HAL_XSPI_Init(&hxspi1) != HAL_OK) {
          Error_Handler();
      }
      if(APS512XX_ReadReg(&hxspi1, APS512XX_MR0_ADDRESS, reg, APS512XX_READ_REG_LATENCY(APS512XX_READ_LATENCY_7)) != APS512XX_OK) {
          Error_Handler();
      }
      //print_s("mr0: ");
      //print_hexln(reg, 1);

      uint8_t buf[32];
      if(APS512XX_Read(&hxspi1, buf, 0, 32, APS512XX_READ_LATENCY_7, 0, 1) != HAL_OK) {
          Error_Handler();
      }
      //print_s("buf: ");
      //print_hexln(buf, 32);

      for(int i = 0; i<32; i++) {
          buf[i] = i;
      }

      if(APS512XX_Write(&hxspi1, buf, 0, 32, APS512XX_WRITE_LATENCY_7, 0, 1) != HAL_OK) {
          Error_Handler();
      }
      memset(buf, 0, 32);

      if(APS512XX_Read(&hxspi1, buf, 0, 32, APS512XX_READ_LATENCY_7, 0, 1) != HAL_OK) {
          Error_Handler();
      }
      //print_s("buf: ");
      //print_hexln(buf, 32);


      // Memory mapped
      if(APS512XX_EnableMemoryMappedMode(
          &hxspi1,
          APS512XX_READ_LATENCY(psram_config.ReadLatencyCode, psram_config.LatencyType),
          APS512XX_WRITE_LATENCY(psram_config.WriteLatencyCode),
          psram_config.IOMode,
          psram_config.BurstType
      ) != APS512XX_OK) {
          Error_Handler();
      }

      uint32_t* psram = (uint32_t*)HSPI1_BASE;
      uint32_t* ram = (uint32_t*)(0x20000000 + (768+64)*1024);
      //print_s("RAM\r\n");
      //memtest(ram, 480*800/2);
      //print_s("PSRAM\r\n");
      //memtest(psram, 480*800/2);

      //uint32_t* p = psram;
      //for(uint32_t* p = psram; p<psram+8; p++) {
      //    print_hex((uint8_t*)(uint32_t)p, 4);
      //  print_s(" ");
      //}
      //print_s("\r\n");

      //for(uint32_t* p = psram; p<psram+8; p++) {
      //    print_hexln((uint8_t*)p, 32);
      //}
      //*psram = 0xcafebabe;
      //if (*psram != 0xcafebabe) {
      //    Error_Handler();
      //}
  }

  /* USER CODE END HSPI1_Init 2 */

}

/**
  * @brief I2C3 Initialization Function
  * @param None
  * @retval None
  */
static void MX_I2C3_Init(void)
{

  /* USER CODE BEGIN I2C3_Init 0 */

  /* USER CODE END I2C3_Init 0 */

  /* USER CODE BEGIN I2C3_Init 1 */

  /* USER CODE END I2C3_Init 1 */
  hi2c3.Instance = I2C3;
  hi2c3.Init.Timing = 0x30909DEC;
  hi2c3.Init.OwnAddress1 = 0;
  hi2c3.Init.AddressingMode = I2C_ADDRESSINGMODE_7BIT;
  hi2c3.Init.DualAddressMode = I2C_DUALADDRESS_DISABLE;
  hi2c3.Init.OwnAddress2 = 0;
  hi2c3.Init.OwnAddress2Masks = I2C_OA2_NOMASK;
  hi2c3.Init.GeneralCallMode = I2C_GENERALCALL_DISABLE;
  hi2c3.Init.NoStretchMode = I2C_NOSTRETCH_DISABLE;
  if (HAL_I2C_Init(&hi2c3) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Analogue filter
  */
  if (HAL_I2CEx_ConfigAnalogFilter(&hi2c3, I2C_ANALOGFILTER_ENABLE) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Digital filter
  */
  if (HAL_I2CEx_ConfigDigitalFilter(&hi2c3, 0) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN I2C3_Init 2 */

  /* USER CODE END I2C3_Init 2 */

}

/**
  * @brief I2C5 Initialization Function
  * @param None
  * @retval None
  */
static void MX_I2C5_Init(void)
{

  /* USER CODE BEGIN I2C5_Init 0 */

  /* USER CODE END I2C5_Init 0 */

  /* USER CODE BEGIN I2C5_Init 1 */

  /* USER CODE END I2C5_Init 1 */
  hi2c5.Instance = I2C5;
  hi2c5.Init.Timing = 0x30909DEC;
  hi2c5.Init.OwnAddress1 = 0;
  hi2c5.Init.AddressingMode = I2C_ADDRESSINGMODE_7BIT;
  hi2c5.Init.DualAddressMode = I2C_DUALADDRESS_DISABLE;
  hi2c5.Init.OwnAddress2 = 0;
  hi2c5.Init.OwnAddress2Masks = I2C_OA2_NOMASK;
  hi2c5.Init.GeneralCallMode = I2C_GENERALCALL_DISABLE;
  hi2c5.Init.NoStretchMode = I2C_NOSTRETCH_DISABLE;
  if (HAL_I2C_Init(&hi2c5) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Analogue filter
  */
  if (HAL_I2CEx_ConfigAnalogFilter(&hi2c5, I2C_ANALOGFILTER_ENABLE) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure Digital filter
  */
  if (HAL_I2CEx_ConfigDigitalFilter(&hi2c5, 0) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN I2C5_Init 2 */

  /* USER CODE END I2C5_Init 2 */

}

/**
  * @brief ICACHE Initialization Function
  * @param None
  * @retval None
  */
static void MX_ICACHE_Init(void)
{

  /* USER CODE BEGIN ICACHE_Init 0 */

  /* USER CODE END ICACHE_Init 0 */

  /* USER CODE BEGIN ICACHE_Init 1 */

  /* USER CODE END ICACHE_Init 1 */

  /** Enable instruction cache in 1-way (direct mapped cache)
  */
  if (HAL_ICACHE_ConfigAssociativityMode(ICACHE_1WAY) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_ICACHE_Enable() != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN ICACHE_Init 2 */

  /* USER CODE END ICACHE_Init 2 */

}

/**
  * @brief LTDC Initialization Function
  * @param None
  * @retval None
  */
static void MX_LTDC_Init(void)
{

  /* USER CODE BEGIN LTDC_Init 0 */

  /* USER CODE END LTDC_Init 0 */

  LTDC_LayerCfgTypeDef pLayerCfg = {0};

  /* USER CODE BEGIN LTDC_Init 1 */

  /* USER CODE END LTDC_Init 1 */
  hltdc.Instance = LTDC;
  hltdc.Init.HSPolarity = LTDC_HSPOLARITY_AL;
  hltdc.Init.VSPolarity = LTDC_VSPOLARITY_AL;
  hltdc.Init.DEPolarity = LTDC_DEPOLARITY_AL;
  hltdc.Init.PCPolarity = LTDC_PCPOLARITY_IPC;
  hltdc.Init.HorizontalSync = 40;
  hltdc.Init.VerticalSync = 20;
  hltdc.Init.AccumulatedHBP = 80;
  hltdc.Init.AccumulatedVBP = 36;
  hltdc.Init.AccumulatedActiveW = 560;
  hltdc.Init.AccumulatedActiveH = 836;
  hltdc.Init.TotalWidth = 590;
  hltdc.Init.TotalHeigh = 848;
  hltdc.Init.Backcolor.Blue = 255;
  hltdc.Init.Backcolor.Green = 0;
  hltdc.Init.Backcolor.Red = 0;
  if (HAL_LTDC_Init(&hltdc) != HAL_OK)
  {
    Error_Handler();
  }
  pLayerCfg.WindowX0 = 0;
  pLayerCfg.WindowX1 = 480;
  pLayerCfg.WindowY0 = 0;
  pLayerCfg.WindowY1 = 800;
  pLayerCfg.PixelFormat = LTDC_PIXEL_FORMAT_RGB565;
  pLayerCfg.Alpha = 255;
  pLayerCfg.Alpha0 = 0;
  pLayerCfg.BlendingFactor1 = LTDC_BLENDING_FACTOR1_CA;
  pLayerCfg.BlendingFactor2 = LTDC_BLENDING_FACTOR2_CA;
  pLayerCfg.FBStartAdress = 0xA0000000;
  pLayerCfg.ImageWidth = 480;
  pLayerCfg.ImageHeight = 800;
  pLayerCfg.Backcolor.Blue = 0;
  pLayerCfg.Backcolor.Green = 0;
  pLayerCfg.Backcolor.Red = 0;
  if (HAL_LTDC_ConfigLayer(&hltdc, &pLayerCfg, 0) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN LTDC_Init 2 */

  /* USER CODE END LTDC_Init 2 */

}

/**
  * @brief OCTOSPI1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_OCTOSPI1_Init(void)
{

  /* USER CODE BEGIN OCTOSPI1_Init 0 */

  /* USER CODE END OCTOSPI1_Init 0 */

  OSPIM_CfgTypeDef sOspiManagerCfg = {0};

  /* USER CODE BEGIN OCTOSPI1_Init 1 */

  /* USER CODE END OCTOSPI1_Init 1 */
  /* OCTOSPI1 parameter configuration*/
  hospi1.Instance = OCTOSPI1;
  hospi1.Init.FifoThreshold = 1;
  hospi1.Init.DualQuad = HAL_OSPI_DUALQUAD_DISABLE;
  hospi1.Init.MemoryType = HAL_OSPI_MEMTYPE_MICRON;
  hospi1.Init.DeviceSize = 32;
  hospi1.Init.ChipSelectHighTime = 1;
  hospi1.Init.FreeRunningClock = HAL_OSPI_FREERUNCLK_DISABLE;
  hospi1.Init.ClockMode = HAL_OSPI_CLOCK_MODE_0;
  hospi1.Init.WrapSize = HAL_OSPI_WRAP_NOT_SUPPORTED;
  hospi1.Init.ClockPrescaler = 1;
  hospi1.Init.SampleShifting = HAL_OSPI_SAMPLE_SHIFTING_NONE;
  hospi1.Init.DelayHoldQuarterCycle = HAL_OSPI_DHQC_DISABLE;
  hospi1.Init.ChipSelectBoundary = 0;
  hospi1.Init.DelayBlockBypass = HAL_OSPI_DELAY_BLOCK_BYPASSED;
  hospi1.Init.MaxTran = 0;
  hospi1.Init.Refresh = 0;
  if (HAL_OSPI_Init(&hospi1) != HAL_OK)
  {
    Error_Handler();
  }
  sOspiManagerCfg.ClkPort = 1;
  sOspiManagerCfg.DQSPort = 1;
  sOspiManagerCfg.NCSPort = 1;
  sOspiManagerCfg.IOLowPort = HAL_OSPIM_IOPORT_1_LOW;
  sOspiManagerCfg.IOHighPort = HAL_OSPIM_IOPORT_1_HIGH;
  if (HAL_OSPIM_Config(&hospi1, &sOspiManagerCfg, HAL_OSPI_TIMEOUT_DEFAULT_VALUE) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN OCTOSPI1_Init 2 */

  /* USER CODE END OCTOSPI1_Init 2 */

}

/**
  * @brief SDMMC1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_SDMMC1_MMC_Init(void)
{

  /* USER CODE BEGIN SDMMC1_Init 0 */

  /* USER CODE END SDMMC1_Init 0 */

  /* USER CODE BEGIN SDMMC1_Init 1 */

  /* USER CODE END SDMMC1_Init 1 */
  hmmc1.Instance = SDMMC1;
  hmmc1.Init.ClockEdge = SDMMC_CLOCK_EDGE_RISING;
  hmmc1.Init.ClockPowerSave = SDMMC_CLOCK_POWER_SAVE_DISABLE;
  hmmc1.Init.BusWide = SDMMC_BUS_WIDE_8B;
  hmmc1.Init.HardwareFlowControl = SDMMC_HARDWARE_FLOW_CONTROL_DISABLE;
  hmmc1.Init.ClockDiv = 0;
  if (HAL_MMC_Init(&hmmc1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN SDMMC1_Init 2 */

  /* USER CODE END SDMMC1_Init 2 */

}

/**
  * @brief USART1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_USART1_UART_Init(void)
{

  /* USER CODE BEGIN USART1_Init 0 */

  /* USER CODE END USART1_Init 0 */

  /* USER CODE BEGIN USART1_Init 1 */

  /* USER CODE END USART1_Init 1 */
  huart1.Instance = USART1;
  huart1.Init.BaudRate = 115200;
  huart1.Init.WordLength = UART_WORDLENGTH_8B;
  huart1.Init.StopBits = UART_STOPBITS_1;
  huart1.Init.Parity = UART_PARITY_NONE;
  huart1.Init.Mode = UART_MODE_TX_RX;
  huart1.Init.HwFlowCtl = UART_HWCONTROL_NONE;
  huart1.Init.OverSampling = UART_OVERSAMPLING_16;
  huart1.Init.OneBitSampling = UART_ONE_BIT_SAMPLE_DISABLE;
  huart1.Init.ClockPrescaler = UART_PRESCALER_DIV1;
  huart1.AdvancedInit.AdvFeatureInit = UART_ADVFEATURE_NO_INIT;
  if (HAL_UART_Init(&huart1) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_UARTEx_SetTxFifoThreshold(&huart1, UART_TXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_UARTEx_SetRxFifoThreshold(&huart1, UART_RXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_UARTEx_DisableFifoMode(&huart1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN USART1_Init 2 */

  print_s("\r\n");

  /* USER CODE END USART1_Init 2 */

}

/**
  * @brief USB_OTG_HS Initialization Function
  * @param None
  * @retval None
  */
static void MX_USB_OTG_HS_HCD_Init(void)
{

  /* USER CODE BEGIN USB_OTG_HS_Init 0 */

  /* USER CODE END USB_OTG_HS_Init 0 */

  /* USER CODE BEGIN USB_OTG_HS_Init 1 */

  /* USER CODE END USB_OTG_HS_Init 1 */
  hhcd_USB_OTG_HS.Instance = USB_OTG_HS;
  hhcd_USB_OTG_HS.Init.Host_channels = 16;
  hhcd_USB_OTG_HS.Init.speed = HCD_SPEED_HIGH;
  hhcd_USB_OTG_HS.Init.dma_enable = DISABLE;
  hhcd_USB_OTG_HS.Init.phy_itface = USB_OTG_HS_EMBEDDED_PHY;
  hhcd_USB_OTG_HS.Init.Sof_enable = DISABLE;
  hhcd_USB_OTG_HS.Init.low_power_enable = DISABLE;
  hhcd_USB_OTG_HS.Init.use_external_vbus = ENABLE;
  if (HAL_HCD_Init(&hhcd_USB_OTG_HS) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN USB_OTG_HS_Init 2 */

  /* USER CODE END USB_OTG_HS_Init 2 */

}

/**
  * @brief GPIO Initialization Function
  * @param None
  * @retval None
  */
static void MX_GPIO_Init(void)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
  /* USER CODE BEGIN MX_GPIO_Init_1 */

  /* USER CODE END MX_GPIO_Init_1 */

  /* GPIO Ports Clock Enable */
  __HAL_RCC_GPIOE_CLK_ENABLE();
  __HAL_RCC_GPIOC_CLK_ENABLE();
  __HAL_RCC_GPIOA_CLK_ENABLE();
  __HAL_RCC_GPIOB_CLK_ENABLE();
  __HAL_RCC_GPIOD_CLK_ENABLE();
  __HAL_RCC_GPIOH_CLK_ENABLE();
  __HAL_RCC_GPIOI_CLK_ENABLE();
  __HAL_RCC_GPIOJ_CLK_ENABLE();
  __HAL_RCC_GPIOF_CLK_ENABLE();
  __HAL_RCC_GPIOG_CLK_ENABLE();

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(GPIOE, LED_GREEN_Pin|LED_RED_Pin|UCPD_DBn_Pin|TOF_LPN_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(DSI_RESETn_GPIO_Port, DSI_RESETn_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(eMMC_RSTn_GPIO_Port, eMMC_RSTn_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(DSI_BL_CTRL_GPIO_Port, DSI_BL_CTRL_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pins : LED_GREEN_Pin LED_RED_Pin UCPD_DBn_Pin TOF_LPN_Pin */
  GPIO_InitStruct.Pin = LED_GREEN_Pin|LED_RED_Pin|UCPD_DBn_Pin|TOF_LPN_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(GPIOE, &GPIO_InitStruct);

  /*Configure GPIO pin : TOF_INTN_Pin */
  GPIO_InitStruct.Pin = TOF_INTN_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(TOF_INTN_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pin : DSI_RESETn_Pin */
  GPIO_InitStruct.Pin = DSI_RESETn_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(DSI_RESETn_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pin : eMMC_RSTn_Pin */
  GPIO_InitStruct.Pin = eMMC_RSTn_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(eMMC_RSTn_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pin : DSI_BL_CTRL_Pin */
  GPIO_InitStruct.Pin = DSI_BL_CTRL_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(DSI_BL_CTRL_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pin : USER_Button_Pin */
  GPIO_InitStruct.Pin = USER_Button_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(USER_Button_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pin : TEMP_INTN_Pin */
  GPIO_InitStruct.Pin = TEMP_INTN_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(TEMP_INTN_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pins : DSI_TOUCH_INT_Pin UCPD_FLT_Pin */
  GPIO_InitStruct.Pin = DSI_TOUCH_INT_Pin|UCPD_FLT_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(GPIOE, &GPIO_InitStruct);

  /* USER CODE BEGIN MX_GPIO_Init_2 */

  /* USER CODE END MX_GPIO_Init_2 */
}

/* USER CODE BEGIN 4 */

typedef struct {
    int cmd;                /*<! The specific LCD command */
    const void *data;       /*<! Buffer that holds the command specific data */
    size_t data_bytes;      /*<! Size of `data` in memory, in bytes */
    unsigned int delay_ms;  /*<! Delay in milliseconds after this command */
} st7701_lcd_init_cmd_t;

// char * dbg(uint8_t* data, size_t data_len) {
//     static char buf[1024] = {0};
//     char* p = buf;
//     for(int i=0; i<data_len; ++i) {
//         int len = snprintf(p, sizeof(buf)-strlen(p), "%02x", data[i]);
//         p += len;
//     }
//     return buf;
// }
// 
// static void print_hex(uint8_t* data, size_t data_len) {
//     char tmp[128];
//     size_t len = snprintf(tmp, sizeof(tmp), "0x%s", dbg(data,data_len));
//     HAL_UART_Transmit(&huart1, (uint8_t*)tmp, len, 1000);
// }
// static void print_hexln(uint8_t* data, size_t data_len) {
//     char tmp[128];
//     size_t len = snprintf(tmp, sizeof(tmp), "0x%s\r\n", dbg(data,data_len));
//     HAL_UART_Transmit(&huart1, (uint8_t*)tmp, len, 1000);
// }

static void print_s(char *str) {
    HAL_UART_Transmit(&huart1, (uint8_t*)str, strlen(str), 1000);
}
void *memset16(void *m, uint16_t val, size_t count)
{
    uint16_t *buf = m;

    while(count--) *buf++ = val;
    return m;
}

void *memset32(void *m, uint32_t val, size_t count)
{
    uint32_t *buf = m;

    while(count--) *buf++ = val;
    return m;
}
void *memset24(void *m, uint32_t val, size_t count)
{
    uint8_t *buf = m;

    while(count--) {
        *buf++ = val&0xff;
        *buf++ = (val>>8) & 0xff;
        *buf++ = (val>>16) & 0xff;
    }
    return m;
}

//uint8_t* fbuf = (uint8_t*)0x20000000;
uint8_t* fbuf = (uint8_t*)HSPI1_BASE;

static void setPanelConfig() {
    memset(fbuf, 0, 480*800*4);
    for(int i =0; i<800; ++i) {
        int w = i % 480;
        memset16(&fbuf[480*2*i], 0xffff, w);
    }
  if(HAL_DSI_Start(&hdsi) != HAL_OK) {
      Error_Handler();
  }

  const st7701_lcd_init_cmd_t lh397k_display_init_sequence[] = {
    //  {cmd, { data }, data_size, delay_ms}
    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x00}, 5, 0},  // Regular command function
    {0x13, (uint8_t[]){0x00}, 0, 0},                          // Turn on normal display mode
    {0xEF, (uint8_t[]){0x08}, 1, 0},                          //??

    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x10}, 5, 0},  // Command 2 BK0 function
    {0xC0, (uint8_t[]){0x63, 0x00}, 2, 0},                    // LNESET (Display Line Setting): (0x63+1)*8 = 800 lines
    {0xC1, (uint8_t[]){0x10, 0x0C}, 2, 0},                    // PORCTRL (Porch Control): VBP = 16, VFP = 12
    {0xC2, (uint8_t[]){0x37, 0x08}, 2, 0},  // INVSET (Inversion sel. & frame rate control): PCLK=512+(8*16) = 640
    {0xCC, (uint8_t[]){0x38}, 1, 0},        //
    {0xB0, (uint8_t[]){0x40, 0xC9, 0x90, 0x0D, 0x0F, 0x04, 0x00, 0x07, 0x07, 0x1C, 0x04, 0x52, 0x0F, 0xDF, 0x26, 0xCF},
     16, 0},  // PVGAMCTRL
    {0xB1, (uint8_t[]){0x40, 0xC9, 0xCF, 0x0C, 0x90, 0x04, 0x00, 0x07, 0x08, 0x1B, 0x06, 0x55, 0x13, 0x62, 0xE7, 0xCF},
     16, 0},  // NVGAMCTRL

    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x11}, 5, 0},  // Command 2 BK1 function
    {0xB0, (uint8_t[]){0x5D}, 1, 0},                          // VRHS
    {0xB1, (uint8_t[]){0x2D}, 1, 0},                          // VCOMS
    {0xB2, (uint8_t[]){0x07}, 1, 0},                          // VGH
    {0xB3, (uint8_t[]){0x80}, 1, 0},                          // TESTCMD
    {0xB5, (uint8_t[]){0x08}, 1, 0},                          // VGLS
    {0xB7, (uint8_t[]){0x85}, 1, 0},                          // PWCTRL1
    {0xB8, (uint8_t[]){0x20}, 1, 0},                          // PWCTRL2
    {0xB9, (uint8_t[]){0x10}, 1, 0},                          // DGMLUTR
    {0xC1, (uint8_t[]){0x78}, 1, 0},                          // SPD1
    {0xC2, (uint8_t[]){0x78}, 1, 0},                          // SPD2
    {0xD0, (uint8_t[]){0x88}, 1, 100},                        // MIPISET1
    {0xE0, (uint8_t[]){0x00, 0x19, 0x02}, 3, 0},              //
    {0xE1, (uint8_t[]){0x05, 0xA0, 0x07, 0xA0, 0x04, 0xA0, 0x06, 0xA0, 0x00, 0x44, 0x44}, 11, 0},              //
    {0xE2, (uint8_t[]){0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}, 13, 0},  //
    {0xE3, (uint8_t[]){0x00, 0x00, 0x33, 0x33}, 5, 0},                                                         //
    {0xE4, (uint8_t[]){0x44, 0x44}, 2, 0},                                                                     //
    {0xE5, (uint8_t[]){0x0D, 0x31, 0xC8, 0xAF, 0x0F, 0x33, 0xC8, 0xAF, 0x09, 0x2D, 0xC8, 0xAF, 0x0B, 0x2F, 0xC8, 0xAF},
     16, 0},                                            //
    {0xE6, (uint8_t[]){0x00, 0x00, 0x33, 0x33}, 4, 0},  //
    {0xE7, (uint8_t[]){0x44, 0x44}, 2, 0},              //
    {0xE8, (uint8_t[]){0x0C, 0x30, 0xC8, 0xAF, 0x0E, 0x32, 0xC8, 0xAF, 0x08, 0x2C, 0xC8, 0xAF, 0x0A, 0x2E, 0xC8, 0xAF},
     16, 0},                                                              //
    {0xEB, (uint8_t[]){0x02, 0x00, 0xE4, 0xE4, 0x44, 0x00, 0x40}, 7, 0},  //
    {0xEC, (uint8_t[]){0x3C, 0x00}, 2, 0},                                //
    {0xED, (uint8_t[]){0xAB, 0x89, 0x76, 0x54, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x10, 0x45, 0x67, 0x98, 0xBA},
     16, 0},  //

    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x00}, 5, 0},  // Regular command function
    {0x11, (uint8_t[]){0x00}, 0, 120},                        // Exit sleep mode
    {0x3A, (uint8_t[]){0x66}, 1, 0},                          // RGB666
    {0x51, (uint8_t[]){0xff}, 1, 0},                          // brightness
    {0x29, (uint8_t[]){0x00}, 0, 0},                          // Display on (enable frame buffer output)
  };

  for(int i=0; i<sizeof(lh397k_display_init_sequence)/sizeof(st7701_lcd_init_cmd_t); ++i) {
    const st7701_lcd_init_cmd_t* cmd = &lh397k_display_init_sequence[i];
    if (cmd->data_bytes > 1) {
      if(HAL_DSI_LongWrite(&hdsi, 0, DSI_DCS_LONG_PKT_WRITE, cmd->data_bytes, cmd->cmd, cmd->data)!= HAL_OK) {
          Error_Handler();
      }
    } else if(cmd->data_bytes == 1) {
      if(HAL_DSI_ShortWrite(&hdsi, 0, DSI_DCS_SHORT_PKT_WRITE_P1, cmd->cmd, ((uint8_t*)cmd->data)[0]) != HAL_OK) {
          Error_Handler();
      }
    } else {
      if(HAL_DSI_ShortWrite(&hdsi, 0, DSI_DCS_SHORT_PKT_WRITE_P0, cmd->cmd, 0) != HAL_OK) {
          Error_Handler();
      }
    }
    if(cmd->delay_ms > 0) {
        HAL_Delay(cmd->delay_ms);
    }
  }
  HAL_UART_Transmit(&huart1, (uint8_t*)"\r\n", 2, 1000);
}

/* USER CODE END 4 */

/**
  * @brief  This function is executed in case of error occurrence.
  * @retval None
  */
void Error_Handler(void)
{
  /* USER CODE BEGIN Error_Handler_Debug */
  /* User can add his own implementation to report the HAL error return state */
  __disable_irq();
  while (1) {
  }
  /* USER CODE END Error_Handler_Debug */
}
#ifdef USE_FULL_ASSERT
/**
  * @brief  Reports the name of the source file and the source line number
  *         where the assert_param error has occurred.
  * @param  file: pointer to the source file name
  * @param  line: assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* USER CODE BEGIN 6 */
  /* User can add his own implementation to report the file name and line
     number, ex: printf("Wrong parameters value: file %s on line %d\r\n", file,
     line) */
  /* USER CODE END 6 */
}
#endif /* USE_FULL_ASSERT */
