#include "firmware_main_loop.h"

#include "hardfault.h"
#include "ui/screen_process.h"
#include "usb/usb_processing.h"

#include <stdbool.h>

/*
 * "is_done" callback that only spins the UI
 * once.
 */
static bool _is_done_run_once(void* param)
{
    if (!param) {
        Abort("_is_done_run_once called\nwith NULL param.");
    }
    bool* already_run = (bool*)param;
    if (!(*already_run)) {
        *already_run = true;
        return false;
    }
    return true;
}

void firmware_main_loop(void)
{
    bool already_run;
    while (1) {
        already_run = false;
        ui_screen_process(_is_done_run_once, &already_run);
        usb_processing_process(usb_processing_hww());
#if defined(APP_U2F)
        usb_processing_process(usb_processing_u2f());
#endif
    }
}
