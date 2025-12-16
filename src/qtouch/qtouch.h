/*============================================================================
Filename : touch.h
Project : QTouch Modular Library
Purpose : configuation macros for touch library

This file is part of QTouch Modular Library Release 5.1 example application.

Important Note: This file was created using the QTouch Configurator within
                Atmel Start and then patched.

Usage License: Refer license.h file for license information
Support: Visit http://www.microchip.com/support/hottopics.aspx
               to create MySupport case.

------------------------------------------------------------------------------
Copyright (c) 2017 Microchip. All rights reserved.
------------------------------------------------------------------------------
============================================================================*/

#ifndef TOUCH_H
#define TOUCH_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/*----------------------------------------------------------------------------
 *     include files
 *----------------------------------------------------------------------------*/

#include "touch_api_ptc.h"

typedef struct {
    uint16_t start_key;
    uint8_t number_of_keys;
    uint8_t resolution;
    uint16_t touch_threshold;
    uint16_t untouch_threshold;
    uint8_t deadband;
    uint8_t hysteresis;
    uint8_t touch_count_in;
} scroller_config_t;

typedef struct {
    int16_t* deltas;
    uint16_t touch_area;
    uint16_t raw_position;
    uint16_t position;
    uint8_t active;
    uint8_t hyst_left;
    uint8_t hyst_right;
    uint8_t touch_count_in;
} scroller_data_t;

typedef struct {
    scroller_config_t* config;
    scroller_data_t* data;
} scroller_control_t;

/**********************************************************/
/******************* Acquisition controls *****************/
/**********************************************************/
/* Defines the Measurement Time in milli seconds.
 * Range: 1 to 255.
 * Default value: 20.
 */
#define DEF_TOUCH_MEASUREMENT_PERIOD_MS 20

/* Defines the Type of sensor
 * Default value: NODE_MUTUAL.
 */
#define DEF_SENSOR_TYPE NODE_SELFCAP

/* Set sensor calibration mode for charge share delay ,Prescaler or series
 * resistor. Range: CAL_AUTO_TUNE_NONE / CAL_AUTO_TUNE_RSEL / CAL_AUTO_TUNE_PRSC
 * / CAL_AUTO_TUNE_CSD Default value: CAL_AUTO_TUNE_NONE.
 */
#define DEF_PTC_CAL_OPTION CAL_AUTO_TUNE_NONE

/* Defines the interrupt priority for the PTC. Set low priority to PTC interrupt
 * for applications having interrupt time constraints. Range: 0 to 2 Default: 2
 * (Lowest Priority)
 */
#define DEF_PTC_INTERRUPT_PRIORITY 2

/* Calibration option to ensure full charge transfer */
/* Bits 7:0 = XX | TT SELECT_TAU | X | CAL_OPTION */
#define DEF_PTC_TAU_TARGET CAL_CHRG_5TAU
#define DEF_PTC_CAL_AUTO_TUNE \
    (uint8_t)((DEF_PTC_TAU_TARGET << CAL_CHRG_TIME_POS) | DEF_PTC_CAL_OPTION)

/* Set default bootup acquisition frequency.
 * Range: FREQ_SEL_0 - FREQ_SEL_15 , FREQ_SEL_SPREAD
 * Default value: FREQ_SEL_0.
 */
#define DEF_SEL_FREQ_INIT FREQ_SEL_0

/*----------------------------------------------------------------------------
 *     defines
 *----------------------------------------------------------------------------*/

/**********************************************************/
/***************** Node Params   ******************/
/**********************************************************/
/* Acquisition Set 1 */
/* Defines the number of sensor nodes in the acquisition set
 * Range: 1 to 65535.
 * Default value: 1
 */
#define DEF_NUM_CHANNELS (8)

/* Defines self-cap node parameter setting
 * {X-line, Y-line, Charge Share Delay, Prescaler, NODE_G(Analog Gain , Digital
 * Gain), filter level}
 */
#define NODE_0_PARAMS \
    {X_NONE, Y_LINE(26), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_2, GAIN_8), FILTER_LEVEL_128}
#define NODE_1_PARAMS \
    {X_NONE, Y_LINE(27), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_1, GAIN_8), FILTER_LEVEL_128}
#define NODE_2_PARAMS \
    {X_NONE, Y_LINE(28), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_2, GAIN_8), FILTER_LEVEL_128}
#define NODE_3_PARAMS \
    {X_NONE, Y_LINE(29), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_1, GAIN_8), FILTER_LEVEL_64}
#define NODE_4_PARAMS \
    {X_NONE, Y_LINE(30), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_1, GAIN_8), FILTER_LEVEL_64}
#define NODE_5_PARAMS \
    {X_NONE, Y_LINE(31), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_1, GAIN_8), FILTER_LEVEL_64}
#define NODE_6_PARAMS \
    {X_NONE, Y_LINE(20), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_1, GAIN_8), FILTER_LEVEL_64}
#define NODE_7_PARAMS \
    {X_NONE, Y_LINE(21), 0, PRSC_DIV_SEL_4, NODE_GAIN(GAIN_1, GAIN_8), FILTER_LEVEL_64}

/**********************************************************/
/***************** Key Params   ******************/
/**********************************************************/
/* Defines the number of key sensors
 * Range: 1 to 65535.
 * Default value: 1
 */
#define DEF_NUM_SENSORS (8)

/* Defines Key Sensor setting
 * {Sensor Threshold, Sensor Hysterisis, Sensor AKS}
 */
#define KEY_0_PARAMS {15, HYST_25, AKS_GROUP_1}
#define KEY_1_PARAMS {15, HYST_25, AKS_GROUP_1}
#define KEY_2_PARAMS {15, HYST_25, AKS_GROUP_1}
#define KEY_3_PARAMS {15, HYST_25, AKS_GROUP_1}
#define KEY_4_PARAMS {12, HYST_25, AKS_GROUP_2}
#define KEY_5_PARAMS {15, HYST_25, AKS_GROUP_2}
#define KEY_6_PARAMS {16, HYST_25, AKS_GROUP_2}
#define KEY_7_PARAMS {12, HYST_25, AKS_GROUP_2}

/* De-bounce counter for additional measurements to confirm touch detection
 * Range: 0 to 255.
 * Default value: 4.
 */
#define DEF_TOUCH_DET_INT 1

/* De-bounce counter for additional measurements to confirm away from touch
 * signal to initiate Away from touch re-calibration. Range: 0 to 255. Default
 * value: 5.
 */
#define DEF_ANTI_TCH_DET_INT 1

/* Threshold beyond with automatic sensor recalibration is initiated.
 * Range: RECAL_100/ RECAL_50 / RECAL_25 / RECAL_12_5 / RECAL_6_25 / MAX_RECAL
 * Default value: RECAL_100.
 */
#define DEF_ANTI_TCH_RECAL_THRSHLD RECAL_100

/* Rate at which sensor reference value is adjusted towards sensor signal value
 * when signal value is greater than reference.
 * Units: 200ms
 * Range: 0-255
 * Default value: 20u = 4 seconds.
 */
#define DEF_TCH_DRIFT_RATE 1

/* Rate at which sensor reference value is adjusted towards sensor signal value
 * when signal value is less than reference.
 * Units: 200ms
 * Range: 0-255
 * Default value: 5u = 1 second.
 */
#define DEF_ANTI_TCH_DRIFT_RATE 1

/* Time to restrict drift on all sensor when one or more sensors are activated.
 * Units: 200ms
 * Range: 0-255
 * Default value: 20u = 4 seconds.
 */
#define DEF_DRIFT_HOLD_TIME 1

/* Set mode for additional sensor measurements based on touch activity.
 * Range: REBURST_NONE / REBURST_UNRESOLVED / REBURST_ALL
 * Default value: REBURST_UNRESOLVED
 */
#define DEF_REBURST_MODE REBURST_NONE

/* Sensor maximum ON duration upon touch.
 * Range: 0-255
 * Default value: 0
 */
#define DEF_MAX_ON_DURATION 0

/**********************************************************/
/***************** Slider/Wheel Parameters ****************/
/**********************************************************/
/*
 * Do not use qtouch scroller module. The button readings need
 * a custom post-filter to reduce noise. The output of the custom
 * filter is then fed into a custom scroller implementation.
 * This allows low noise button readings while keeping
 * fast responsiveness.
 */
#define DEF_NUM_SCROLLERS 2 // Number of scrollers (sliders or wheels)
#define DEF_SCROLLER_NUM_CHANNELS 4 // Number of channels per scroller
#define DEF_SCROLLER_OFFSET_0 4 // Index of first button in scroller
#define DEF_SCROLLER_OFFSET_1 0 // Index of first button in scroller
#define DEF_SCROLLER_RESOLUTION 8 // Scroller resolution in bits
#define DEF_SCROLLER_TOUCH_THRESHOLD 35 // Scroller active threshold
#define DEF_SCROLLER_UNTOUCH_THRESHOLD 20 // Scroller active threshold
#define DEF_SCROLLER_DEADBAND \
    10 // everything below deadband is locked to 0 and above max-deadband is locked to max
#define DEF_SCROLLER_HYSTERESIS 12 // Position needs to move at least this much
#define DEF_SCROLLER_TOUCH_DRIFT_IN 2 // number of counts in touch before being active

#ifdef __cplusplus
}
#endif // __cplusplus
#endif // TOUCH_C
