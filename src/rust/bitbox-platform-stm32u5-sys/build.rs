// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

const ARM_NONE_EABI_GCC: &str = "arm-none-eabi-gcc";

const ST_SOURCES: &[&str] = &[
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_adc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_adc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dma.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dma_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_i2c.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_i2c_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_rcc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_rcc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_cortex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_hash.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_hash_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_flash.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_xspi.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_flash_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gpio.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_exti.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pwr.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pwr_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gtzc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_icache.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_xspi.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_ll_dlyb.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_ospi.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_ll_sdmmc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_sd.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_sd_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_mmc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_mmc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_sdio.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_uart.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_uart_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_hcd.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pcd.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_pcd_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_ll_usb.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_gpu2d.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_ltdc.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_ltdc_ex.c",
    "Drivers/STM32U5xx_HAL_Driver/Src/stm32u5xx_hal_dsi.c",
];

const ST_DEFINES: &[&str] = &[
    "USE_HAL_DRIVER",
    "STM32U5A9xx",
    "UX_INCLUDE_USER_DEFINE_FILE",
];
const ST_DEBUG_DEFINES: &[(&str, &str)] = &[("USE_FULL_ASSERT", "1U")];

const ST_INCLUDES: &[&str] = &[
    "Common/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc",
    "Drivers/STM32U5xx_HAL_Driver/Inc/Legacy",
    "Drivers/CMSIS/Device/ST/STM32U5xx/Include",
    "Drivers/CMSIS/Include",
    "USBX/App",
    "USBX/Target",
    "Middlewares/ST/usbx/common/core/inc",
    "Middlewares/ST/usbx/common/usbx_device_classes/inc",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers",
    "Middlewares/ST/usbx/ports/generic/inc",
];

const USBX_SOURCES: &[&str] = &[
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_address_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_endpoint_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_endpoint_destroy.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_endpoint_reset.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_endpoint_stall.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_endpoint_status.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_frame_number_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_function.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_initialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_initialize_complete.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_state_change.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_transfer_abort.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_transfer_request.c",
    "Middlewares/ST/usbx/common/core/src/ux_dcd_sim_slave_transfer_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_activate.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_change.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_deactivate.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_entry.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_initialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_read.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_read_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_thread.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_write.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_class_dpump_write_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_alternate_setting_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_alternate_setting_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_class_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_class_unregister.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_clear_feature.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_configuration_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_configuration_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_control_request_process.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_descriptor_send.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_disconnect.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_endpoint_stall.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_get_status.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_host_wakeup.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_initialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_interface_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_interface_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_interface_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_interface_start.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_microsoft_extension_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_set_feature.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_tasks_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_transfer_abort.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_transfer_all_request_abort.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_transfer_request.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_transfer_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_device_stack_uninitialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_asynch_queue_process.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_asynch_schedule.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_asynchronous_endpoint_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_asynchronous_endpoint_destroy.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_controller_disable.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_ed_obtain.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_ed_td_clean.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_endpoint_reset.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_entry.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_frame_number_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_frame_number_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_initialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_interrupt_endpoint_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_iso_queue_process.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_iso_schedule.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_isochronous_endpoint_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_isochronous_td_obtain.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_least_traffic_list_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_periodic_endpoint_destroy.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_periodic_schedule.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_periodic_tree_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_port_reset.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_port_status_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_regular_td_obtain.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_request_bulk_transfer.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_request_control_transfer.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_request_interupt_transfer.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_request_isochronous_transfer.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_request_transfer.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_timer_function.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_transaction_schedule.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_transfer_abort.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_transfer_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_hcd_sim_host_uninitialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_activate.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_configure.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_deactivate.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_endpoints_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_entry.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_ioctl.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_read.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_class_dpump_write.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_bandwidth_check.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_bandwidth_claim.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_bandwidth_release.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_call.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_device_scan.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_instance_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_instance_destroy.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_instance_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_instance_verify.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_interface_scan.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_class_unregister.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_descriptor_parse.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_enumerate.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_instance_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_instance_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_interface_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_interface_scan.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_configuration_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_delay_ms.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_address_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_configuration_activate.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_configuration_deactivate.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_configuration_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_configuration_reset.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_configuration_select.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_descriptor_read.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_remove.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_resources_free.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_device_string_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_endpoint_instance_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_endpoint_instance_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_endpoint_reset.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_endpoint_transfer_abort.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_enum_thread_entry.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_hcd_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_hcd_thread_entry.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_hcd_transfer_request.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_hcd_unregister.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_hnp_polling_thread_entry.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_initialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_interface_endpoint_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_interface_instance_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_interface_instance_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_interface_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_interface_setting_select.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_interfaces_scan.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_new_configuration_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_new_device_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_new_device_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_new_endpoint_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_new_interface_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_rh_change_process.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_rh_device_extraction.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_rh_device_insertion.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_role_swap.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_tasks_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_transfer_request.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_transfer_request_abort.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_transfer_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_host_stack_uninitialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_system_error_handler.c",
    "Middlewares/ST/usbx/common/core/src/ux_system_initialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_system_tasks_run.c",
    "Middlewares/ST/usbx/common/core/src/ux_system_uninitialize.c",
    "Middlewares/ST/usbx/common/core/src/ux_trace_event_insert.c",
    "Middlewares/ST/usbx/common/core/src/ux_trace_event_update.c",
    "Middlewares/ST/usbx/common/core/src/ux_trace_object_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_trace_object_unregister.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_debug_callback_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_debug_log.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_delay_ms.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_descriptor_pack.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_descriptor_parse.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_error_callback_register.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_event_flags_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_event_flags_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_event_flags_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_event_flags_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_long_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_long_get_big_endian.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_long_put.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_long_put_big_endian.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_allocate.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_allocate_add_safe.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_allocate_mulc_safe.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_allocate_mulv_safe.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_byte_pool_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_byte_pool_search.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_compare.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_copy.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_free.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_free_block_best_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_memory_set.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_mutex_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_mutex_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_mutex_off.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_mutex_on.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_pci_class_scan.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_pci_read.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_pci_write.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_physical_address.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_semaphore_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_semaphore_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_semaphore_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_semaphore_put.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_set_interrupt_handler.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_short_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_short_get_big_endian.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_short_put.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_short_put_big_endian.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_string_length_check.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_string_length_get.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_string_to_unicode.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_identify.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_relinquish.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_resume.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_schedule_other.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_sleep.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_thread_suspend.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_timer_create.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_timer_delete.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_unicode_to_string.c",
    "Middlewares/ST/usbx/common/core/src/ux_utility_virtual_address.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_callback.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_endpoint_create.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_endpoint_destroy.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_endpoint_reset.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_endpoint_stall.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_endpoint_status.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_frame_number_get.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_function.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_initialize.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_initialize_complete.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_interrupt_handler.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_transfer_abort.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_transfer_request.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_transfer_run.c",
    "Middlewares/ST/usbx/common/usbx_stm32_device_controllers/ux_dcd_stm32_uninitialize.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_activate.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_control_request.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_deactivate.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_descriptor_send.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_entry.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_event_get.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_event_set.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_initialize.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_interrupt_thread.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_read.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_read_run.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_receiver_event_free.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_receiver_event_get.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_receiver_initialize.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_receiver_tasks_run.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_receiver_thread.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_receiver_uninitialize.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_report_get.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_report_set.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_tasks_run.c",
    "Middlewares/ST/usbx/common/usbx_device_classes/src/ux_device_class_hid_uninitialize.c",
];

fn run_bindgen(wrapper: &Path, output: &Path, clang_args: &[String]) -> Result<(), &'static str> {
    let res = Command::new("bindgen")
        .arg("--output")
        .arg(output)
        .arg("--use-core")
        .arg("--with-derive-default")
        .arg("--no-layout-tests")
        .arg("--rustified-enum")
        .arg(".*")
        .arg(wrapper)
        .arg("--")
        .args(clang_args)
        .output()
        .expect("failed to run bindgen");

    if !res.status.success() {
        println!(
            "bindgen-out:\n{}\n\nbindgen-err:\n{}",
            std::str::from_utf8(&res.stdout).unwrap_or("invalid utf8"),
            std::str::from_utf8(&res.stderr).unwrap_or("invalid utf8"),
        );
        return Err("bindgen failed");
    }
    Ok(())
}

fn arm_none_eabi_sysroot() -> Result<String, &'static str> {
    let output = Command::new(ARM_NONE_EABI_GCC)
        .arg("--print-sysroot")
        .output()
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                "`arm-none-eabi-gcc` executable was not found. Check your PATH."
            } else {
                "failed to execute `arm-none-eabi-gcc --print-sysroot`"
            }
        })?;
    if !output.status.success() {
        return Err("`arm-none-eabi-gcc --print-sysroot` failed");
    }
    let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if sysroot.is_empty() {
        return Err("`arm-none-eabi-gcc --print-sysroot` returned an empty sysroot");
    }
    Ok(sysroot)
}

fn is_release_profile() -> bool {
    env::var("PROFILE").expect("PROFILE not set") == "release"
}

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-changed=wrapper.h");
    println!("cargo::rerun-if-env-changed=PROFILE");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let target = env::var("TARGET").expect("TARGET not set");

    // These bindings describe STM32U5 firmware headers and are only valid for the embedded target.
    // Host builds only need the crate to exist as a dependency, so skip generating/importing them.
    if !target.starts_with("thumb") {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join("bindings.rs");

    let repo_root = manifest_dir.join("../../..");
    let st_root = repo_root.join("external/ST");

    if !st_root.is_dir() {
        return Err("external/ST not found");
    }

    for source in ST_SOURCES.iter().chain(USBX_SOURCES.iter()) {
        println!(
            "cargo::rerun-if-changed={}",
            st_root.join(source).as_path().display()
        );
    }
    for include in ST_INCLUDES {
        println!(
            "cargo::rerun-if-changed={}",
            st_root.join(include).as_path().display()
        );
    }

    if let Err(err) = Command::new("bindgen").arg("--version").output() {
        if err.kind() == ErrorKind::NotFound {
            return Err("`bindgen` executable was not found. Check your PATH.");
        }
        return Err("failed to execute `bindgen --version`");
    }

    let include_paths: Vec<PathBuf> = ST_INCLUDES.iter().map(|p| st_root.join(p)).collect();

    let release_profile = is_release_profile();
    let mut clang_args: Vec<String> = ST_DEFINES.iter().map(|d| format!("-D{d}")).collect();
    if !release_profile {
        clang_args.extend(
            ST_DEBUG_DEFINES
                .iter()
                .map(|(key, value)| format!("-D{key}={value}")),
        );
    }
    clang_args.extend(
        include_paths
            .iter()
            .map(|p| format!("-I{}", p.as_path().display())),
    );
    let sysroot = arm_none_eabi_sysroot()?;

    // Generate bindings for the active firmware target ABI, not the host ABI.
    clang_args.push(format!("--target={target}"));
    clang_args.push(format!("--sysroot={sysroot}"));

    let wrapper = manifest_dir.join("wrapper.h");
    if !wrapper.is_file() {
        return Err("wrapper.h not found");
    }
    run_bindgen(&wrapper, &out_path, &clang_args)?;

    let source_paths: Vec<PathBuf> = ST_SOURCES
        .iter()
        .chain(USBX_SOURCES.iter())
        .map(|p| st_root.join(p))
        .collect();
    let mut build = cc::Build::new();
    build.files(&source_paths);
    for def in ST_DEFINES {
        build.define(def, None);
    }
    if !release_profile {
        for (key, value) in ST_DEBUG_DEFINES {
            build.define(key, Some(*value));
        }
    }
    build.includes(&include_paths);
    build.flag(format!("--sysroot={sysroot}"));
    // Suppress warnings in third-party sources.
    build.flag_if_supported("-w");
    build.compile("st_drivers");
    Ok(())
}
