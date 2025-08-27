#include "u2f_app.h"

#include <hardfault.h>
#include <rust/rust.h>
#include <ui/screen_process.h>
#include <util.h>

#include <stddef.h>
#include <stdio.h>

#define APPID_BOGUS_CHROMIUM "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
#define APPID_BOGUS_FIREFOX "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"

static struct {
    /** Type of outstanding async operation. */
    enum u2f_app_confirm_t outstanding_confirm;
    /** App ID of the outstanding async operation. */
    uint8_t app_id[32];
} _state = {0};

// appid: 32 byte appid
// out: string,
static void _app_string(const uint8_t* app_id, char* out, size_t out_len)
{
    rust_u2f_app_string(rust_util_bytes(app_id, 32), rust_util_bytes_mut((uint8_t*)out, out_len));
}

static bool _is_app_id_bogus(const uint8_t* app_id)
{
    return MEMEQ(app_id, APPID_BOGUS_CHROMIUM, U2F_APPID_SIZE) ||
           MEMEQ(app_id, APPID_BOGUS_FIREFOX, U2F_APPID_SIZE);
}

void u2f_app_confirm_start(enum u2f_app_confirm_t type, const uint8_t* app_id)
{
    char app_string[100] = {0};
    const char* title;
    switch (type) {
    case U2F_APP_REGISTER:
        if (!_is_app_id_bogus(app_id)) {
            title = "U2F register";
            _app_string(app_id, app_string, sizeof(app_string));
        } else {
            // If the authentication fails with the "Bad key handle" the browser will execute bogus
            // registrations to make the device blink.
            title = "";
            snprintf(app_string, sizeof(app_string), "%s", "Use U2F?");
        }
        break;
    case U2F_APP_AUTHENTICATE:
        title = "U2F auth";
        _app_string(app_id, app_string, sizeof(app_string));
        break;
    default:
        Abort("u2f_app_confirm: Internal error");
    }
    _state.outstanding_confirm = type;
    memcpy(_state.app_id, app_id, 32);
    rust_workflow_spawn_confirm(title, app_string);
}

async_op_result_t u2f_app_confirm_retry(enum u2f_app_confirm_t type, const uint8_t* app_id)
{
    if (_state.outstanding_confirm != type || !MEMEQ(app_id, _state.app_id, 32)) {
        // CID used a new app_id, clearly invalid
        // TODO: Clear u2f state
        return ASYNC_OP_NOT_READY;
    }
    bool result = false;
    if (!rust_workflow_confirm_poll(&result)) {
        return ASYNC_OP_NOT_READY;
    }
    _state.outstanding_confirm = U2F_APP_NONE;
    return result ? ASYNC_OP_TRUE : ASYNC_OP_FALSE;
}

void u2f_app_confirm_abort(void)
{
    if (_state.outstanding_confirm == U2F_APP_NONE) {
        Abort("Invalid abort call in U2F app.");
    }
    rust_workflow_abort_current();
    _state.outstanding_confirm = U2F_APP_NONE;
}
