/*============================================================================
Filename : touch.c
Project : QTouch Modular Library
Purpose : Provides Initialization, Processing and ISR handler of touch library,
          Simple API functions to get/set the key touch parameters from/to the
          touch library data structures

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
#ifndef TOUCH_C
#define TOUCH_C
/*----------------------------------------------------------------------------
 *     include files
 *----------------------------------------------------------------------------*/

#include <platform/driver_init.h>

#include "qtouch.h"

/*----------------------------------------------------------------------------
 *   prototypes
 *----------------------------------------------------------------------------*/

/*! \brief configure keys, wheels and sliders.
 */
static touch_ret_t touch_sensors_config(void);

/*! \brief Touch measure complete callback function example prototype.
 */
static void qtm_measure_complete_callback(void);

/*! \brief Touch Error callback function prototype.
 */
static void qtm_error_callback(uint8_t error);

/* Update our scroller implementation
 */
void qtouch_process_scroller_positions(scroller_control_t* scrollers);

/*----------------------------------------------------------------------------
 *     Global Variables
 *----------------------------------------------------------------------------*/

/* Flag to indicate time for touch measurement */
volatile uint8_t time_to_measure_touch_flag = 0;

/* postporcess request flag */
volatile uint8_t touch_postprocess_request = 0;

/* Measurement Done Touch Flag  */
volatile uint8_t measurement_done_touch = 0;

/* Error Handling */
uint8_t module_error_code = 0;

/* Acquisition module internal data - Size to largest acquisition set */
uint16_t touch_acq_signals_raw[DEF_NUM_CHANNELS];

/* Acquisition set 1 - General settings */
qtm_acq_node_group_config_t ptc_qtlib_acq_gen1 = {
    DEF_NUM_CHANNELS,
    DEF_SENSOR_TYPE,
    DEF_PTC_CAL_AUTO_TUNE,
    DEF_SEL_FREQ_INIT,
    DEF_PTC_INTERRUPT_PRIORITY};

/* Node status, signal, calibration values */
qtm_acq_node_data_t ptc_qtlib_node_stat1[DEF_NUM_CHANNELS];

/* Node configurations */
qtm_acq_samd51_node_config_t ptc_seq_node_cfg1[DEF_NUM_CHANNELS] = {
    NODE_0_PARAMS,
    NODE_1_PARAMS,
    NODE_2_PARAMS,
    NODE_3_PARAMS,
    NODE_4_PARAMS,
    NODE_5_PARAMS,
    NODE_6_PARAMS,
    NODE_7_PARAMS};

/* Container */
qtm_acquisition_control_t qtlib_acq_set1 = {
    &ptc_qtlib_acq_gen1,
    &ptc_seq_node_cfg1[0],
    &ptc_qtlib_node_stat1[0]};

/**********************************************************/
/*********************** Keys Module **********************/
/**********************************************************/

/* Keys set 1 - General settings */
qtm_touch_key_group_config_t qtlib_key_grp_config_set1 = {
    DEF_NUM_SENSORS,
    DEF_TOUCH_DET_INT,
    DEF_MAX_ON_DURATION,
    DEF_ANTI_TCH_DET_INT,
    DEF_ANTI_TCH_RECAL_THRSHLD,
    DEF_TCH_DRIFT_RATE,
    DEF_ANTI_TCH_DRIFT_RATE,
    DEF_DRIFT_HOLD_TIME,
    DEF_REBURST_MODE};

qtm_touch_key_group_data_t qtlib_key_grp_data_set1;

/* Key data */
qtm_touch_key_data_t qtlib_key_data_set1[DEF_NUM_SENSORS];

/* Key Configurations */
qtm_touch_key_config_t qtlib_key_configs_set1[DEF_NUM_SENSORS] = {
    KEY_0_PARAMS,
    KEY_1_PARAMS,
    KEY_2_PARAMS,
    KEY_3_PARAMS,
    KEY_4_PARAMS,
    KEY_5_PARAMS,
    KEY_6_PARAMS,
    KEY_7_PARAMS};

/* Container */
qtm_touch_key_control_t qtlib_key_set1 = {
    &qtlib_key_grp_data_set1,
    &qtlib_key_grp_config_set1,
    &qtlib_key_data_set1[0],
    &qtlib_key_configs_set1[0]};

/**********************************************************/
/***************** Scroller Module ********************/
/**********************************************************/

/*
 * Do not use qtouch scroller module. The button readings need
 * a custom post-filter to reduce noise. The output of the custom
 * filter is then fed into a custom scroller implementation.
 * This allows low noise button readings while keeping
 * fast responsiveness.
 */

/* Holds preceding touch key deltas (signal - reference) */
__extension__ int16_t scroller_deltas[DEF_NUM_SENSORS] = {0};

scroller_config_t scroller_config[2] = {
    [0] =
        {DEF_SCROLLER_OFFSET_0,
         DEF_SCROLLER_NUM_CHANNELS,
         DEF_SCROLLER_RESOLUTION,
         DEF_SCROLLER_TOUCH_THRESHOLD,
         DEF_SCROLLER_UNTOUCH_THRESHOLD,
         DEF_SCROLLER_DEADBAND,
         DEF_SCROLLER_HYSTERESIS,
         DEF_SCROLLER_TOUCH_DRIFT_IN},
    [1] =
        {DEF_SCROLLER_OFFSET_1,
         DEF_SCROLLER_NUM_CHANNELS,
         DEF_SCROLLER_RESOLUTION,
         DEF_SCROLLER_TOUCH_THRESHOLD,
         DEF_SCROLLER_UNTOUCH_THRESHOLD,
         DEF_SCROLLER_DEADBAND,
         DEF_SCROLLER_HYSTERESIS,
         DEF_SCROLLER_TOUCH_DRIFT_IN},
};
scroller_data_t scroller_data[2] = {0};

scroller_control_t scroller_control[2] = {
    [0] = {&scroller_config[0], &scroller_data[0]},
    [1] = {&scroller_config[1], &scroller_data[1]},
};

/*============================================================================
static touch_ret_t touch_sensors_config(void)
------------------------------------------------------------------------------
Purpose: Initialization of touch key sensors
Input  : none
Output : none
Notes  :
============================================================================*/
/* Touch sensors config - assign nodes to buttons / wheels / sliders / surfaces
 * / water level / etc */
static touch_ret_t touch_sensors_config(void)
{
    uint16_t sensor_nodes;
    touch_ret_t touch_ret = TOUCH_SUCCESS;

    /* Init acquisition module */
    qtm_ptc_init_acquisition_module(&qtlib_acq_set1);

    /* Init pointers to DMA sequence memory */
    qtm_ptc_qtlib_assign_signal_memory(&touch_acq_signals_raw[0]);

    /* Initialize sensor nodes */
    for (sensor_nodes = 0u; sensor_nodes < DEF_NUM_CHANNELS; sensor_nodes++) {
        /* Enable each node for measurement and mark for calibration */
        qtm_enable_sensor_node(&qtlib_acq_set1, sensor_nodes);
        qtm_calibrate_sensor_node(&qtlib_acq_set1, sensor_nodes);
    }

    /* Enable sensor keys and assign nodes */
    for (sensor_nodes = 0u; sensor_nodes < DEF_NUM_CHANNELS; sensor_nodes++) {
        qtm_init_sensor_key(&qtlib_key_set1, sensor_nodes, &ptc_qtlib_node_stat1[sensor_nodes]);
    }
    return (touch_ret);
}

/*============================================================================
static void qtm_measure_complete_callback( void )
------------------------------------------------------------------------------
Purpose: this function is called after the completion of
         measurement cycle. This function sets the post processing request
         flag to trigger the post processing.
Input  : none
Output : none
Notes  :
============================================================================*/
static void qtm_measure_complete_callback(void)
{
    touch_postprocess_request = 1u;
}

/*============================================================================
static void qtm_error_callback(uint8_t error)
------------------------------------------------------------------------------
Purpose: this function is used to report error in the modules.
Input  : error code
Output : decoded module error code
Notes  :
Derived Module_error_codes:
    Acquisition module error =1
    post processing module1 error = 2
    post processing module2 error = 3
    ... and so on

============================================================================*/
static void qtm_error_callback(uint8_t error)
{
    module_error_code = error + 1u;
}

static void scroller_init(void)
{
    for (int i = 0; i < DEF_NUM_SCROLLERS; i++) {
        scroller_config_t* config = scroller_control[i].config;
        scroller_data_t* data = scroller_control[i].data;
        data->deltas = &scroller_deltas[config->start_key];
        data->hyst_left = config->hysteresis / 2;
        data->hyst_right = config->hysteresis / 2;
        data->position = 0;
        data->raw_position = 0;
        data->touch_area = 0;
    }
}

/*============================================================================
void touch_init(void)
------------------------------------------------------------------------------
Purpose: Initialization of touch library. PTC, timer, and
         datastreamer modules are initialized in this function.
Input  : none
Output : none
Notes  :
============================================================================*/
void qtouch_init(void)
{
    qtouch_timer_config();

    /* Configure touch sensors with Application specific settings */
    touch_sensors_config();

    /* Configure scrollers */
    scroller_init();
}

/*============================================================================
void touch_process(void)
------------------------------------------------------------------------------
Purpose: Main processing function of touch library. This function initiates the
         acquisition, calls post processing after the acquistion complete and
         sets the flag for next measurement based on the sensor status.
Input  : none
Output : none
Notes  :
============================================================================*/
void qtouch_process(void)
{
    touch_ret_t touch_ret;

    /* check the time_to_measure_touch_flag flag for Touch Acquisition */
    if (time_to_measure_touch_flag == 1u) {
        /* Do the acquisition */
        touch_ret = qtm_ptc_start_measurement_seq(&qtlib_acq_set1, qtm_measure_complete_callback);

        /* if the Acquistion request was successful then clear the request flag */
        if (TOUCH_SUCCESS == touch_ret) {
            /* Clear the Measure request flag */
            time_to_measure_touch_flag = 0u;
        }
    }

    /* check the flag for node level post processing */
    if (touch_postprocess_request == 1u) {
        /* Reset the flags for node_level_post_processing */
        touch_postprocess_request = 0u;

        /* Run Acquisition module level post processing*/
        touch_ret = qtm_acquisition_process();

        /* Check the return value */
        if (TOUCH_SUCCESS == touch_ret) {
            /* Returned with success: Start module level post processing */
            touch_ret = qtm_key_sensors_process(&qtlib_key_set1);
            if (TOUCH_SUCCESS != touch_ret) {
                qtm_error_callback(1);
            }
            qtouch_process_scroller_positions(scroller_control);
        } else {
            /* Acq module Eror Detected: Issue an Acq module common error code 0x80 */
            qtm_error_callback(0);
        }

        if ((0u != (qtlib_key_set1.qtm_touch_key_group_data->qtm_keys_status & 0x80u))) {
            time_to_measure_touch_flag = 1u;
        } else {
            measurement_done_touch = 1u;
        }
    }
}

/*============================================================================
void touch_timer_handler(void)
------------------------------------------------------------------------------
Purpose: This function updates the time elapsed to the touch key module to
         synchronize the internal time counts used by the module.
Input  : none
Output : none
Notes  :
============================================================================*/
void qtouch_timer_handler(void)
{
    /* Count complete - Measure touch sensors */
    time_to_measure_touch_flag = 1u;
    qtm_update_qtlib_timer(DEF_TOUCH_MEASUREMENT_PERIOD_MS);
}

static void Timer_task_cb(const struct timer_task* const timer_task)
{
    (void)timer_task;
    qtouch_timer_handler();
}

void qtouch_timer_config(void)
{
    static struct timer_task Timer_task;
    static uint8_t timer_task_added = 0;

    if (timer_task_added) {
        timer_remove_task(&TIMER_0, &Timer_task);
    }
    Timer_task.interval = DEF_TOUCH_MEASUREMENT_PERIOD_MS;
    Timer_task.cb = Timer_task_cb;
    Timer_task.mode = TIMER_TASK_REPEAT;

    timer_add_task(&TIMER_0, &Timer_task);
    timer_task_added = 1;
    timer_start(&TIMER_0);
}

uint16_t qtouch_get_sensor_node_signal(uint16_t sensor_node)
{
    return (ptc_qtlib_node_stat1[sensor_node].node_acq_signals);
}

void qtouch_update_sensor_node_signal(uint16_t sensor_node, uint16_t new_signal)
{
    ptc_qtlib_node_stat1[sensor_node].node_acq_signals = new_signal;
}

uint16_t qtouch_get_sensor_node_reference(uint16_t sensor_node)
{
    return (qtlib_key_data_set1[sensor_node].channel_reference);
}

void qtouch_update_sensor_node_reference(uint16_t sensor_node, uint16_t new_reference)
{
    qtlib_key_data_set1[sensor_node].channel_reference = new_reference;
}

uint16_t qtouch_get_sensor_cc_val(uint16_t sensor_node)
{
    return (ptc_qtlib_node_stat1[sensor_node].node_comp_caps);
}

void qtouch_update_sensor_cc_val(uint16_t sensor_node, uint16_t new_cc_value)
{
    ptc_qtlib_node_stat1[sensor_node].node_comp_caps = new_cc_value;
}

uint8_t qtouch_get_sensor_state(uint16_t sensor_node)
{
    return (qtlib_key_set1.qtm_touch_key_data[sensor_node].sensor_state);
}

void qtouch_update_sensor_state(uint16_t sensor_node, uint8_t new_state)
{
    qtlib_key_set1.qtm_touch_key_data[sensor_node].sensor_state = new_state;
}

bool qtouch_is_scroller_active(uint16_t scroller)
{
    return scroller_control[scroller].data->active;
}
uint16_t qtouch_get_scroller_position(uint16_t scroller)
{
    return scroller_control[scroller].data->position;
}

// EMA is applied to the node readings to filter out noise. Fixed point is used to make it
// less resource intense than floating points.
//
// Exponential Moving Average with N=3, ALPHA=2/(N+1)
#define Q_FACTOR 1024
#define ALPHA 512
#define ONE_MINUS_ALPHA (Q_FACTOR - ALPHA)

void qtouch_process_scroller_positions(scroller_control_t* scrollers)
{
    scroller_control_t* scroller;
    scroller_control_t* scrollers_end = scrollers + DEF_NUM_SCROLLERS;
    for (scroller = scrollers; scroller < scrollers_end; scroller++) {
        scroller_config_t* config = scroller->config;
        scroller_data_t* data = scroller->data;
        int node_max = -1;
        int16_t node_max_value = INT16_MIN;
        data->touch_area = 0;
        bool key_active = 0;
        bool scroller_active = data->active;

        for (int i = 0; i < config->number_of_keys; i++) {
            uint8_t node = config->start_key + i;
            int32_t delta = max(
                0, qtouch_get_sensor_node_signal(node) - qtouch_get_sensor_node_reference(node));

            // Ignore small deltas
            delta = delta < 4 ? 0 : delta;

            // Some node readings have a max amplitude that is approximately 50% lower than the
            // highest reading nodes. For the slider position to be accurately estimated the
            // amplitudes need to have similar max amplitudes.

            // Increase some nodes with 50%
            if (node == 1 || node == 3 || node == 4 || node == 5 || node == 7) {
                delta += delta >> 1;
            }
            // Increase some nodes with 25%
            if (node == 0 || node == 5 || node == 6) {
                delta += delta >> 2;
            }

            // Apply EMA
            delta = (delta * ALPHA + data->deltas[i] * ONE_MINUS_ALPHA) / Q_FACTOR;

            data->deltas[i] = delta;

            // If any key is in touch, the scroller is considered in touch
            if (qtouch_get_sensor_state(node) == QTM_KEY_STATE_DETECT) {
                key_active = 1;
            }

            // Find the node with the highest signal to determine how to calculate touch area.
            if (data->deltas[i] > node_max_value) {
                node_max = i;
                node_max_value = data->deltas[i];
            }
        }

        switch (node_max) {
        case 0:
            data->touch_area = data->deltas[0] + (data->deltas[1] >> 2);
            break;
        case 1:
            data->touch_area = (data->deltas[0] >> 3) + data->deltas[1] + (data->deltas[2] >> 3);
            break;
        case 2:
            data->touch_area = (data->deltas[1] >> 3) + data->deltas[2] + (data->deltas[3] >> 3);
            break;
        case 3:
            data->touch_area = (data->deltas[2] >> 2) + data->deltas[3];
            break;
        default:
            break;
        }

        // In case no single node is in touch, but multiple nodes together are, treat that as in
        // touch
        if (data->touch_area > config->touch_threshold) {
            data->touch_count_in = min(255, data->touch_count_in + 1);
            if (data->touch_count_in > config->touch_count_in) {
                scroller_active = 1;
            }
        }
        if (data->touch_area < config->untouch_threshold) {
            scroller_active = 0;
            data->touch_count_in = 0;
        }

        bool active = key_active | scroller_active;

        // shift up all calculations to increase precision
        uint32_t fp_offset = 2;
        uint32_t raw_pos = 0;
        uint32_t weight_sum = 0;
        uint32_t sum = 0;
        // offset weights with "half distance" to avoid "centroid at the edge" problem. i.e. that
        // the weight of the zeroth node is 0 and therefore it only weakly influences the result.
        uint32_t half_distance =
            (1 << (config->resolution + fp_offset)) / (config->number_of_keys - 1) / 2;
        for (int i = 0; i < config->number_of_keys; ++i) {
            int32_t delta = data->deltas[i];
            sum += delta;
            int32_t weight =
                (i << (config->resolution + fp_offset)) / (config->number_of_keys - 1) +
                half_distance;
            raw_pos += delta * weight;
            weight_sum += weight;
        }
        raw_pos /= sum;
        raw_pos -= min(raw_pos, half_distance);
        raw_pos >>= fp_offset;

        if (active) {
            int32_t pos = data->position;

            if (data->active) {
                int32_t raw_pos_delta = raw_pos - pos;
                if (raw_pos_delta < -data->hyst_left || raw_pos_delta > data->hyst_right) {
                    pos = raw_pos;

                    /* handle hysterisis, when finger changes direction */
                    if (pos < data->position) {
                        data->hyst_left = 0;
                        data->hyst_right = config->hysteresis;
                    }
                    if (pos > data->position) {
                        data->hyst_left = config->hysteresis;
                        data->hyst_right = 0;
                    }
                }
            } else {
                pos = raw_pos;
            }

            /* Handle deadband (close to 0 and close to max resolution) */
            if (raw_pos < config->deadband) {
                pos = 0;
                data->hyst_left = 0;
                data->hyst_right = config->hysteresis / 2;
            }
            if (raw_pos >= ((1U << config->resolution) - 1) - config->deadband) {
                pos = (1 << config->resolution) - 1;
                data->hyst_left = config->hysteresis / 2;
                data->hyst_right = 0;
            }

            data->position = pos;
            data->raw_position = raw_pos;
        } else {
            data->raw_position = raw_pos;
            data->hyst_left = config->hysteresis / 2;
            data->hyst_right = config->hysteresis / 2;
        }
        data->active = active;
    }
}

void qtouch_calibrate_node(uint16_t sensor_node)
{
    /* Calibrate Node */
    qtm_calibrate_sensor_node(&qtlib_acq_set1, sensor_node);
    /* Initialize key */
    qtm_init_sensor_key(&qtlib_key_set1, sensor_node, &ptc_qtlib_node_stat1[sensor_node]);
}

/*============================================================================
ISR(ADC0_RESRDY_vect)
------------------------------------------------------------------------------
Purpose:  ADC EOC Interrupt - Call PTC handler or application
Input    :  none
Output  :  none
Notes    :  none
============================================================================*/
void ADC0_1_Handler(void)
{
    ADC0->INTFLAG.reg |= 1u;
    qtm_samd51_ptc_handler();
}

#endif /* TOUCH_C */
