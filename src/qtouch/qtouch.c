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

#include "qtouch.h"
#include "license.h"
#include "touch_api_ptc.h"
#include "util.h"
#include <driver_init.h>
#include <platform_config.h>

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
void qtouch_process_scroller_positions(void);

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

/* Stores recent unfiltered scroller positions for custom filter */
__extension__ static uint16_t scroller_previous_position[][DEF_SCROLLER_NUM_PREV_POS] = {
    [0 ...(DEF_NUM_SCROLLERS - 1)][0 ...(DEF_SCROLLER_NUM_PREV_POS - 1)] = 0};

/* Current scroller position consisting of a moving-average of preceding positions for custom filter
 */
__extension__ static uint16_t scroller_position[] = {[0 ...(DEF_NUM_SCROLLERS - 1)] = 0};

/* Whether or not scroller reading exceeds threshold for custom filter */
__extension__ static bool scroller_active[DEF_NUM_SCROLLERS] = {[0 ...(DEF_NUM_SCROLLERS - 1)] = 0};

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

/*============================================================================
static touch_ret_t touch_sensors_config(void)
------------------------------------------------------------------------------
Purpose: Initialization of touch key sensors
Input  : none
Output : none
Notes  :
============================================================================*/
/* Touch sensors config - assign nodes to buttons / wheels / sliders / surfaces / water level / etc
 */
static touch_ret_t touch_sensors_config(void)
{
    uint16_t sensor_nodes;
    touch_ret_t touch_ret = TOUCH_SUCCESS;

    /* Init acquisition module */
    qtm_ptc_init_acquisition_module(&qtlib_acq_set1);

    /* Init pointers to DMA sequence memory */
    qtm_ptc_qtlib_assign_signal_memory(&touch_acq_signals_raw[0]);

    /* Initialize sensor nodes */
    for (sensor_nodes = 0U; sensor_nodes < DEF_NUM_CHANNELS; sensor_nodes++) {
        /* Enable each node for measurement and mark for calibration */
        qtm_enable_sensor_node(&qtlib_acq_set1, sensor_nodes);
        qtm_calibrate_sensor_node(&qtlib_acq_set1, sensor_nodes);
    }

    /* Enable sensor keys and assign nodes */
    for (sensor_nodes = 0U; sensor_nodes < DEF_NUM_CHANNELS; sensor_nodes++) {
        qtm_init_sensor_key(&qtlib_key_set1, sensor_nodes, &ptc_qtlib_node_stat1[sensor_nodes]);
    }
    return (touch_ret);
}

/*============================================================================
static void qtm_measure_complete_callback( void )
------------------------------------------------------------------------------
Purpose: Callback function from binding layer called after the completion of
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

/*============================================================================
void qtouch_init(void)
------------------------------------------------------------------------------
Purpose: Initialization of touch library. PTC, timer, binding layer and
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
}

/*============================================================================
void qtouch_process(void)
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
            qtouch_process_scroller_positions();
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
void qtouch_timer_handler(void)
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

static void qtouch_timer_task_cb(const struct timer_task* const timer_task)
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
    Timer_task.cb = qtouch_timer_task_cb;
    Timer_task.mode = TIMER_TASK_REPEAT;

    timer_add_task(&TIMER_0, &Timer_task);
    timer_task_added = 1;
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

/* Holds preceding unfiltered scroller positions */
static uint16_t sensor_previous_filtered_reading[DEF_NUM_SENSORS][DEF_SENSOR_NUM_PREV_POS] = {0};

uint16_t qtouch_get_sensor_node_signal_filtered(uint16_t sensor_node)
{
    // Filter the sensor signal.
    //
    // Smooth it out and saturate it so that values never go beyond DEF_SENSOR_CEILING.
    // This helps to mitigate 'jumpy' channels that exist at higher sensor readings when
    // in noisy environments.
    //
    uint16_t X;
    uint16_t sensor_raw = qtouch_get_sensor_node_signal(sensor_node);
    uint16_t sensor_reference = qtouch_get_sensor_node_reference(sensor_node);

    if (sensor_reference == 0) {
        // If a sensor reference is 0, it means that the sensor is not yet calibrated (or dead).
        // The signal can be high anyway, which makes it look like the sensor is being touched when
        // it isn't.
        return 0;
    }
    X = sensor_raw < sensor_reference ? 0 : sensor_raw - sensor_reference;
    // Add more weight to edge buttons because they are physically smaller (smaller readings).
    if ((sensor_node == DEF_SCROLLER_OFFSET_0) || (sensor_node == DEF_SCROLLER_OFFSET_1) ||
        (sensor_node == DEF_SCROLLER_OFFSET_0 + DEF_SCROLLER_NUM_CHANNELS - 1) ||
        (sensor_node == DEF_SCROLLER_OFFSET_1 + DEF_SCROLLER_NUM_CHANNELS - 1)) {
        X = (uint16_t)((double)X * (1 + DEF_SENSOR_EDGE_WEIGHT));
    }
    // Saturate out-of-range readings.
    X = (X > DEF_SENSOR_CEILING) ? DEF_SENSOR_CEILING : X;

    // Calculate sensor readout using a moving average
    // The moving average wieghts previous N readings twice current reading
    uint16_t moving_average_cummulative_weight = 1; // Add one for current reading calculated above
    uint16_t X_ave = X;
    for (size_t j = 0; j < DEF_SENSOR_NUM_PREV_POS; j++) {
        moving_average_cummulative_weight += 2;
        X_ave += sensor_previous_filtered_reading[sensor_node][j] * 2;
    }
    X_ave = X_ave / moving_average_cummulative_weight;

    // Update recorded previous positions
    for (size_t j = 0; j < DEF_SENSOR_NUM_PREV_POS - 1; j++) {
        sensor_previous_filtered_reading[sensor_node][j] =
            sensor_previous_filtered_reading[sensor_node][j + 1];
    }
    sensor_previous_filtered_reading[sensor_node][DEF_SENSOR_NUM_PREV_POS - 1] = X;

    return X_ave;
}

bool qtouch_is_scroller_active(uint16_t scroller)
{
    return scroller_active[scroller];
}

uint16_t qtouch_get_scroller_position(uint16_t sensor_node)
{
    return scroller_position[sensor_node];
}

void qtouch_process_scroller_positions(void)
{
    for (uint8_t scroller = 0; scroller < DEF_NUM_SCROLLERS; scroller++) {
        uint8_t i, j;
        uint16_t sum = 0;
        uint16_t max_sensor_reading = 0;
        uint16_t min_sensor_reading = DEF_SENSOR_CEILING;
        uint16_t weighted_sum = 0;
        uint16_t filtered_readings[DEF_SCROLLER_NUM_CHANNELS] = {0};
        uint16_t sensor_location[DEF_SCROLLER_NUM_CHANNELS] = {
            1, // Offset by `1` because a `0` location cannot be weight-averaged
            DEF_SCROLLER_RESOLUTION / 3,
            DEF_SCROLLER_RESOLUTION / 3 * 2,
            DEF_SCROLLER_RESOLUTION};

        for (i = 0; i < DEF_SCROLLER_NUM_CHANNELS; i++) {
            filtered_readings[i] = qtouch_get_sensor_node_signal_filtered(
                i + (scroller ? DEF_SCROLLER_OFFSET_1 : DEF_SCROLLER_OFFSET_0));
            min_sensor_reading = (filtered_readings[i] < min_sensor_reading) ? filtered_readings[i]
                                                                             : min_sensor_reading;
            max_sensor_reading = (filtered_readings[i] > max_sensor_reading) ? filtered_readings[i]
                                                                             : max_sensor_reading;
        }

        // Read filterd data and weight by sensor physical location
        // Reduce the value by the min_sensor_reading to improve positional accuracy.
        // Touch position is calculated with a weighted average of the sensor readings.
        // If properly calibrated, sensors on the opposite end of a finger touch would
        // be zero and thus make no contribution to the weighted average. If the baseline
        // sensor readings are elevated, the sensors on the opposite edge DO contribute
        // to the weighted average making a positional artifact (i.e. the position is more
        // central than it should be in reality). This artifact is higher when the finger
        // is a bit distant while approaching and lower/negligible when the finger is
        // fully touching the device. This can cause the position to move enough to enter
        // "slide" mode and disable "tap" events being emitted.
        for (i = 0; i < DEF_SCROLLER_NUM_CHANNELS; i++) {
            sum += filtered_readings[i] - min_sensor_reading;
            weighted_sum += (filtered_readings[i] - min_sensor_reading) * sensor_location[i];
        }

        // Compensate for deadband (i.e. when only a single edge button gives a reading and
        // neighbors do not)
        uint16_t scaled_value = weighted_sum / sum;
        scaled_value =
            (scaled_value < DEF_SCROLLER_DEADBAND) ? DEF_SCROLLER_DEADBAND : (weighted_sum / sum);
        scaled_value = (scaled_value > (DEF_SCROLLER_RESOLUTION - DEF_SCROLLER_DEADBAND))
                           ? (DEF_SCROLLER_RESOLUTION - DEF_SCROLLER_DEADBAND)
                           : scaled_value;
        scaled_value = ((scaled_value - DEF_SCROLLER_DEADBAND) * (DEF_SCROLLER_RESOLUTION - 1)) /
                       (DEF_SCROLLER_RESOLUTION - 2 * DEF_SCROLLER_DEADBAND);

        // Calculate scroller position using a moving average
        // The moving average wieghts previous N readings twice current reading
        if (sum >= DEF_SCROLLER_DET_THRESHOLD) {
            uint16_t moving_average_cummulative_weight = 0;
            scroller_position[scroller] = 0;
            for (j = 0; j < DEF_SCROLLER_NUM_PREV_POS; j++) {
                if (scroller_previous_position[scroller][j] != DEF_SCROLLER_OFF) {
                    moving_average_cummulative_weight += 2;
                    scroller_position[scroller] += scroller_previous_position[scroller][j] * 2;
                }
            }
            scroller_position[scroller] +=
                scaled_value; // Most recent signal is half weight of others to help avoid bounce
                              // when finger is released
            scroller_position[scroller] =
                scroller_position[scroller] / (moving_average_cummulative_weight + 1);
        }

        // Update recorded previous positions and scroller active state
        for (j = 0; j < DEF_SCROLLER_NUM_PREV_POS - 1; j++) {
            scroller_previous_position[scroller][j] = scroller_previous_position[scroller][j + 1];
        }
        if (sum >= DEF_SCROLLER_DET_THRESHOLD) {
            scroller_previous_position[scroller][DEF_SCROLLER_NUM_PREV_POS - 1] = scaled_value;
        } else {
            scroller_previous_position[scroller][DEF_SCROLLER_NUM_PREV_POS - 1] = DEF_SCROLLER_OFF;
        }
        // Use the maximum value of all sensor readings as an estimate of pressure.
        // Put a threshold on this to detect whether we're touching or not.
        if (max_sensor_reading >= DEF_SCROLLER_TOUCH_THRESHOLD) {
            scroller_active[scroller] = true;
        } else if (max_sensor_reading <= DEF_SCROLLER_UNTOUCH_THRESHOLD) {
            scroller_active[scroller] = false;
        }
    }
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
    ADC0->INTFLAG.reg |= 1U;
    qtm_samd51_ptc_handler();
}

#endif /* TOUCH_C */
