#include "u2f_app.h"

#include <hardfault.h>
#include <ui/screen_process.h>
#include <ui/workflow_stack.h>
#include <util.h>
#include <workflow/confirm.h>

#include <stddef.h>
#include <stdio.h>

#define APPID_BOGUS_CHROMIUM "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
#define APPID_BOGUS_FIREFOX "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"

typedef struct {
    uint8_t app_id[32];
    const char* name;
} app_t;

static const app_t _apps[] = {
    {
        // sha256('https://github.com/u2f/trusted_facets')
        .app_id = "\x70\x61\x7d\xfe\xd0\x65\x86\x3a\xf4\x7c\x15\x55\x6c\x91\x79\x88\x80\x82\x8c\xc4"
                  "\x07\xfd\xf7\x0a\xe8\x50\x11\x56\x94\x65\xa0\x75",
        .name = "GitHub",
    },
    {
        // sha256('https://www.gstatic.com/securitykey/origins.json')
        .app_id = "\xa5\x46\x72\xb2\x22\xc4\xcf\x95\xe1\x51\xed\x8d\x4d\x3c\x76\x7a\x6c\xc3\x49\x43"
                  "\x59\x43\x79\x4e\x88\x4f\x3d\x02\x3a\x82\x29\xfd",
        .name = "Google",
    },
};

struct {
    /** Type of outstanding async operation. */
    enum u2f_app_confirm_t outstanding_confirm;
    /** App ID of the outstanding async operation. */
    uint8_t app_id[32];
    /** Whether the outstanding async operation has been confirmed. */
    bool confirmed;
    /** Whether the confirmation step has finished. */
    bool confirmed_done;
} _state;

// appid: 32 byte appid
// out: string,
static void _app_string(const uint8_t* app_id, char* out, size_t out_len)
{
    for (size_t i = 0; i < sizeof(_apps) / sizeof(app_t); i++) {
        const app_t* app = &_apps[i];
        if (MEMEQ(app_id, app->app_id, 32)) {
            snprintf(out, out_len, "%s", app->name);
            return;
        }
    }
    char appid_hex[32 * 2 + 1] = {0};
    util_uint8_to_hex(app_id, 32, appid_hex);
    snprintf(out, out_len, "Unknown site:\n%.16s\n%.16s", appid_hex, appid_hex + 16);
}

static void _confirm_cb(bool result, void* param)
{
    (void)param;
    _state.confirmed = result;
    _state.confirmed_done = true;
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
        title = "U2F authenticate";
        _app_string(app_id, app_string, sizeof(app_string));
        break;
    default:
        Abort("u2f_app_confirm: Internal error");
    }
    confirm_params_t params = {
        .title = title,
        .body = app_string,
    };
    _state.confirmed_done = false;
    _state.outstanding_confirm = type;
    memcpy(_state.app_id, app_id, 32);
    workflow_stack_start_workflow(workflow_confirm(&params, _confirm_cb, NULL));
}

async_op_result_t u2f_app_confirm_retry(enum u2f_app_confirm_t type, const uint8_t* app_id)
{
    if (_state.outstanding_confirm != type || !MEMEQ(app_id, _state.app_id, 32)) {
        Abort("Arbitration failed for U2F confirmation.");
    }
    if (!_state.confirmed_done) {
        return ASYNC_OP_NOT_READY;
    }
    _state.outstanding_confirm = U2F_APP_NONE;
    return _state.confirmed ? ASYNC_OP_TRUE : ASYNC_OP_FALSE;
}
