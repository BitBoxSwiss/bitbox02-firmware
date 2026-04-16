/* USER CODE BEGIN Header */
/**
 ******************************************************************************
 * @file           : board.c
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
#include "board.h"
#include "adc.h"
#include "dsihost.h"
#include "flash.h"
#include "gpu2d.h"
#include "hspi.h"
#include "i2c.h"
#include "icache.h"
#include "ltdc.h"
#include "octospi.h"
#include "sdmmc.h"
#include "usart.h"
#include "app_usbx_device.h"
#include "gpio.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */
#include <stdio.h>
#include <string.h>
#include <aps512xx.h>
#include <stdint.h>
#include "ux_device_customhid.h"

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
/* USER CODE BEGIN PFP */

//static void setPanelConfig(void);
static void print_s(char *str);
static void print_hex(uint8_t* data, size_t data_len);
static void print_hexln(uint8_t* data, size_t data_len);
static void memtest(uint32_t* addr, uint32_t words);

/* USER CODE END PFP */

/* Private user code ---------------------------------------------------------*/
/* USER CODE BEGIN 0 */

static uint8_t usb_report_rx_buf[64] = {0};
static int usb_report_rx_len = 0;
static uint8_t got_data = 0;
static uint16_t usb_report_tx_counter = 0;

void got_report(uint8_t* data, int len) {
    usb_report_rx_len = len;
    memcpy(usb_report_rx_buf, data, len);
    got_data =1;
}

/* USER CODE END 0 */

/**
  * @brief  The application entry point.
  * @retval int
  */
int board_init(void)
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
  MX_GPU2D_Init();
  MX_DSIHOST_DSI_Init();
  MX_LTDC_Init();
  MX_USBX_Device_Init();
  MX_FLASH_Init();
  /* USER CODE BEGIN 2 */
  // ^- The call to MX_DSIHOST_DSI_Init() must be done before MX_LTDC_Init().
  

  // setPanelConfig();
  // uint8_t buf[2] = {0x55, 0x55};
  // if(APS512XX_ReadID(&hxspi1, buf, APS512XX_READ_LATENCY_6) != APS512XX_OK) {
  //     Error_Handler();
  // }
  // print_s("memory id: ");
  // print_hex(buf, 2);


  //static unsigned char tx_buf[256] = {0};
  //int count = 0;

  /* USER CODE END 2 */

  /* Infinite loop */
  /* USER CODE BEGIN WHILE */
  //int len = snprintf(tx_buf, sizeof(tx_buf), "Hello world\r\n");
  //HAL_UART_Transmit(&huart1, tx_buf, len, 1000);
  //uint32_t prev = HAL_GetTick();
  //while (1) {
  //    uint32_t now = HAL_GetTick();
  //    uint32_t elapsed = now - prev;
  //    if(elapsed/1000 == 1) {
  //       uint8_t sample_report[64] = {0};
  //       //int len = snprintf(tx_buf, sizeof(tx_buf), "count: %d\r\n", count++);
  //       //HAL_UART_Transmit(&huart1, tx_buf, len, 1000);
  //       sample_report[0] = 0xA5;
  //       sample_report[1] = 0x5A;
  //       sample_report[2] = usb_report_tx_counter & 0xff;
  //       sample_report[3] = (usb_report_tx_counter>>8) & 0xff;
  //       usb_report_tx_counter++;
  //       sample_report[4] = 0;
  //       sample_report[5] = 0;
  //       sample_report[6] = 0;
  //       for (uint32_t i = 7; i < sizeof(sample_report); ++i) {
  //           sample_report[i] = (uint8_t)i;
  //       }
  //       if(USBD_Custom_HID_SendReport(sample_report, sizeof(sample_report)) != UX_SUCCESS) {
  //          // uint8_t buf[] = "error\r\n";
  //          //HAL_UART_Transmit(&huart1, buf, sizeof(buf)-1, 1000);

  //       }
  //       prev = now;
  //    }
  //    if(got_data) {
  //       //int len = snprintf(tx_buf, sizeof(tx_buf), "got data, %d\r\n", usb_report_rx_len);
  //       //HAL_UART_Transmit(&huart1, tx_buf, len, 1000);
  //       //print_hexln(usb_report_rx_buf, usb_report_rx_len);
  //       got_data = 0;
  //       //USBD_HID_SendReport();
  //    }
  //  /* USER CODE END WHILE */

  //  /* USER CODE BEGIN 3 */
  //  USBX_Device_Process();
  //}
  /* USER CODE END 3 */
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

/* USER CODE BEGIN 4 */

typedef struct {
    int cmd;                /*<! The specific LCD command */
    const void *data;       /*<! Buffer that holds the command specific data */
    size_t data_bytes;      /*<! Size of `data` in memory, in bytes */
    unsigned int delay_ms;  /*<! Delay in milliseconds after this command */
} st7701_lcd_init_cmd_t;

char * dbg(uint8_t* data, size_t data_len) {
    static char buf[1024] = {0};
    char* p = buf;
    for(int i=0; i<data_len; ++i) {
        int len = snprintf(p, sizeof(buf)-strlen(p), "%02x", data[i]);
        p += len;
    }
    return buf;
}

static void print_hex(uint8_t* data, size_t data_len) {
	(void)data;
	(void)data_len;
    //char tmp[128];
    //size_t len = snprintf(tmp, sizeof(tmp), "0x%s", dbg(data,data_len));
    //HAL_UART_Transmit(&huart1, (uint8_t*)tmp, len, 1000);
}
static void print_hexln(uint8_t* data, size_t data_len) {
	(void)data;
	(void)data_len;
    //char tmp[256];
    //size_t len = snprintf(tmp, sizeof(tmp), "0x%s\r\n", dbg(data,data_len));
    //HAL_UART_Transmit(&huart1, (uint8_t*)tmp, len, 1000);
}

static void print_s(char *str) {
	(void)str;
    //HAL_UART_Transmit(&huart1, (uint8_t*)str, strlen(str), 1000);
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

//static void setPanelConfig() {
//    memset(fbuf, 0, 480*800*4);
//    for(int i =0; i<800; ++i) {
//        int w = i % 480;
//        memset16(&fbuf[480*2*i], 0xffff, w);
//    }
//    //fbuf[0] = 0xff;
//    //fbuf[1] = 0xff;
//    //fbuf[2] = 0xff;
//    //fbuf[3] = 0xff;
//    //memset(fbuf, 0xff, 480*4);
//    //memset(&fbuf[480*3*2], 0xff, 480*3);
//    //memset(&fbuf[480*3*4], 0xff, 480*3);
//    //memset(&fbuf[480*3*6], 0xff, 480*3);
//    //memset32(fbuf, 0xffffffff, 480);
//    //memset32(&fbuf[480*4], 0xffffffff, 480);
//    //memset32(&fbuf[797*480*4], 0xffffffff, 480);
//    //memset32(&fbuf[796*480*4], 0xffff0000, 480);
//    //memset32(&fbuf[797*480*4], 0xffff0000, 480);
//    //memset32(&fbuf[798*480*4], 0xffff0000, 480);
//    //memset32(&fbuf[799*480*4], 0xffff0000, 480);
//    //memset32(&fbuf[799*480*4], 0xffff0000, 480);
//    //memset32((uint8_t*)0x20000000, 0xff00ff00, 480);
//  if(HAL_DSI_Start(&hdsi) != HAL_OK) {
//      Error_Handler();
//  }
//  // SWRESET
//  //if(HAL_DSI_ShortWrite(&hdsi, 0, DSI_DCS_SHORT_PKT_WRITE_P0, 0x01, 0) != HAL_OK) {
//  //    Error_Handler();
//  //}
//  //HAL_Delay(150);
//  //if(HAL_DSI_LongWrite(&hdsi, 0, DSI_DCS_LONG_PKT_WRITE, 5, 0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x00}) != HAL_OK) {
//  //    Error_Handler();
//  //}
//   {
//     uint8_t buf[64] = {0};
//     if(HAL_DSI_Read(&hdsi, 0, buf, 4, DSI_DCS_SHORT_PKT_READ, 0x04, 0) == HAL_OK)
//     {
//       print_s("rddid: ");
//       print_hexln(buf, 8);
//     } else {
//       print_s("no_response\r\n");
//     }
//   }
//   //{
//   //  uint8_t buf[64] = {0};
//   //  if(HAL_DSI_Read(&hdsi, 0, buf, 1, DSI_DCS_SHORT_PKT_READ, 0x0A, 0) == HAL_OK)
//   //  {
//   //    print_s("rddpm: ");
//   //    print_hex(buf, 2);
//   //  } else {
//   //    print_s("no_response\r\n");
//   //  }
//   //}
//
//  const st7701_lcd_init_cmd_t lh397k_display_init_sequence[] = {
//    //  {cmd, { data }, data_size, delay_ms}
//    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x00}, 5, 0},  // Regular command function
//    {0x13, (uint8_t[]){0x00}, 0, 0},                          // Turn on normal display mode
//    {0xEF, (uint8_t[]){0x08}, 1, 0},                          //??
//
//    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x10}, 5, 0},  // Command 2 BK0 function
//    {0xC0, (uint8_t[]){0x63, 0x00}, 2, 0},                    // LNESET (Display Line Setting): (0x63+1)*8 = 800 lines
//    {0xC1, (uint8_t[]){0x10, 0x0C}, 2, 0},                    // PORCTRL (Porch Control): VBP = 16, VFP = 12
//    {0xC2, (uint8_t[]){0x37, 0x08}, 2, 0},  // INVSET (Inversion sel. & frame rate control): PCLK=512+(8*16) = 640
//    {0xCC, (uint8_t[]){0x38}, 1, 0},        //
//    {0xB0, (uint8_t[]){0x40, 0xC9, 0x90, 0x0D, 0x0F, 0x04, 0x00, 0x07, 0x07, 0x1C, 0x04, 0x52, 0x0F, 0xDF, 0x26, 0xCF},
//     16, 0},  // PVGAMCTRL
//    {0xB1, (uint8_t[]){0x40, 0xC9, 0xCF, 0x0C, 0x90, 0x04, 0x00, 0x07, 0x08, 0x1B, 0x06, 0x55, 0x13, 0x62, 0xE7, 0xCF},
//     16, 0},  // NVGAMCTRL
//
//    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x11}, 5, 0},  // Command 2 BK1 function
//    {0xB0, (uint8_t[]){0x5D}, 1, 0},                          // VRHS
//    {0xB1, (uint8_t[]){0x2D}, 1, 0},                          // VCOMS
//    {0xB2, (uint8_t[]){0x07}, 1, 0},                          // VGH
//    {0xB3, (uint8_t[]){0x80}, 1, 0},                          // TESTCMD
//    {0xB5, (uint8_t[]){0x08}, 1, 0},                          // VGLS
//    {0xB7, (uint8_t[]){0x85}, 1, 0},                          // PWCTRL1
//    {0xB8, (uint8_t[]){0x20}, 1, 0},                          // PWCTRL2
//    {0xB9, (uint8_t[]){0x10}, 1, 0},                          // DGMLUTR
//    {0xC1, (uint8_t[]){0x78}, 1, 0},                          // SPD1
//    {0xC2, (uint8_t[]){0x78}, 1, 0},                          // SPD2
//    {0xD0, (uint8_t[]){0x88}, 1, 100},                        // MIPISET1
//    {0xE0, (uint8_t[]){0x00, 0x19, 0x02}, 3, 0},              //
//    {0xE1, (uint8_t[]){0x05, 0xA0, 0x07, 0xA0, 0x04, 0xA0, 0x06, 0xA0, 0x00, 0x44, 0x44}, 11, 0},              //
//    {0xE2, (uint8_t[]){0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}, 13, 0},  //
//    {0xE3, (uint8_t[]){0x00, 0x00, 0x33, 0x33}, 5, 0},                                                         //
//    {0xE4, (uint8_t[]){0x44, 0x44}, 2, 0},                                                                     //
//    {0xE5, (uint8_t[]){0x0D, 0x31, 0xC8, 0xAF, 0x0F, 0x33, 0xC8, 0xAF, 0x09, 0x2D, 0xC8, 0xAF, 0x0B, 0x2F, 0xC8, 0xAF},
//     16, 0},                                            //
//    {0xE6, (uint8_t[]){0x00, 0x00, 0x33, 0x33}, 4, 0},  //
//    {0xE7, (uint8_t[]){0x44, 0x44}, 2, 0},              //
//    {0xE8, (uint8_t[]){0x0C, 0x30, 0xC8, 0xAF, 0x0E, 0x32, 0xC8, 0xAF, 0x08, 0x2C, 0xC8, 0xAF, 0x0A, 0x2E, 0xC8, 0xAF},
//     16, 0},                                                              //
//    {0xEB, (uint8_t[]){0x02, 0x00, 0xE4, 0xE4, 0x44, 0x00, 0x40}, 7, 0},  //
//    {0xEC, (uint8_t[]){0x3C, 0x00}, 2, 0},                                //
//    {0xED, (uint8_t[]){0xAB, 0x89, 0x76, 0x54, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x10, 0x45, 0x67, 0x98, 0xBA},
//     16, 0},  //
//
//    {0xFF, (uint8_t[]){0x77, 0x01, 0x00, 0x00, 0x00}, 5, 0},  // Regular command function
//    {0x11, (uint8_t[]){0x00}, 0, 120},                        // Exit sleep mode
//    {0x3A, (uint8_t[]){0x66}, 1, 0},                          // RGB666
//    {0x51, (uint8_t[]){0xff}, 1, 0},                          // brightness
//    {0x29, (uint8_t[]){0x00}, 0, 0},                          // Display on (enable frame buffer output)
//  };
//
//  for(int i=0; i<sizeof(lh397k_display_init_sequence)/sizeof(st7701_lcd_init_cmd_t); ++i) {
//    const st7701_lcd_init_cmd_t* cmd = &lh397k_display_init_sequence[i];
//    if (cmd->data_bytes > 1) {
//      if(HAL_DSI_LongWrite(&hdsi, 0, DSI_DCS_LONG_PKT_WRITE, cmd->data_bytes, cmd->cmd, cmd->data)!= HAL_OK) {
//          Error_Handler();
//      }
//    } else if(cmd->data_bytes == 1) {
//      if(HAL_DSI_ShortWrite(&hdsi, 0, DSI_DCS_SHORT_PKT_WRITE_P1, cmd->cmd, ((uint8_t*)cmd->data)[0]) != HAL_OK) {
//          Error_Handler();
//      }
//    } else {
//      if(HAL_DSI_ShortWrite(&hdsi, 0, DSI_DCS_SHORT_PKT_WRITE_P0, cmd->cmd, 0) != HAL_OK) {
//          Error_Handler();
//      }
//    }
//    if(cmd->delay_ms > 0) {
//        HAL_Delay(cmd->delay_ms);
//    }
//    char buf[10];
//    size_t len = snprintf(buf, sizeof(buf), "%d ", i);
//    HAL_UART_Transmit(&huart1, (uint8_t*)buf, len, 1000);
//  }
//  HAL_UART_Transmit(&huart1, (uint8_t*)"\r\n", 2, 1000);
//  //   uint8_t buf[64] = {0};
//  //   if(HAL_DSI_Read(&hdsi, 0, buf, 1, DSI_DCS_SHORT_PKT_READ, 0x0f, 0) == HAL_OK)
//  //   {
//  //     print_s("rddsdr: ");
//  //     print_hex(buf, 2);
//  //   } else {
//  //     print_s("no_response\r\n");
//  //   }
//  // {
//  //   uint8_t buf[64] = {0};
//  //   if(HAL_DSI_Read(&hdsi, 0, buf, 1, DSI_DCS_SHORT_PKT_READ, 0x0A, 0) == HAL_OK)
//  //   {
//  //     print_s("rddpm: ");
//  //     print_hex(buf, 2);
//  //   } else {
//  //     print_s("no_response\r\n");
//  //   }
//  // }
//}

#define ITERATIONS 100

static void memtest(uint32_t* addr, uint32_t words) {
	(void)addr;
	(void)words;
  //int start = HAL_GetTick();
  //for(int i=0; i<ITERATIONS; i++) {
  //  for(uint32_t* p = addr; p<addr+words; p++) {
  //    *p = (uint32_t)p;
  //  }
  //}
  //int duration = HAL_GetTick()-start;
  //double bandwidth = ((double)words*ITERATIONS*4/1024/1024)*1000/duration;
  //{
  //  char buf[64];
  //  snprintf(buf, 64, "WRITE ticks/iter: %d, bandwidth: %d.%d MB/s\r\n", duration/ITERATIONS, (int)bandwidth, ((int)(bandwidth*100)%100));
  //  print_s(buf);
  //}

  //start = HAL_GetTick();
  //for(int i=0; i<ITERATIONS; i++) {
  //  for(uint32_t* p = addr; p<addr+words; p++) {
  //    if(*p != (uint32_t)p) {
  //      Error_Handler();
  //    }
  //  }
  //}
  //duration = HAL_GetTick()-start;
  //bandwidth = ((double)words*ITERATIONS*4/1024/1024)*1000/duration;
  //{
  //  char buf[64];
  //  snprintf(buf, 64, "READ ticks/iter: %d, bandwidth: %d.%d MB/s\r\n", duration/ITERATIONS, (int)bandwidth, ((int)(bandwidth*100)%100));
  //  print_s(buf);
  //}
}

/* USER CODE END 4 */

/**
  * @brief  This function is executed in case of error occurrence.
  * @retval None
  */
void Error_Handler(void)
{
  /* USER CODE BEGIN Error_Handler_Debug */
    __BKPT(0);
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
