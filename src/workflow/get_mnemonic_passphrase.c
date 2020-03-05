#include "get_mnemonic_passphrase.h"

#include <string.h>

#include <hardfault.h>
#include <ui/components/trinary_input_string.h>
#include <ui/fonts/password_11X12.h>

#include "blocking.h"
#include "confirm.h"
#include "password_enter.h"
#include "status.h"

bool get_mnemonic_passphrase(char* passphrase_out)
{
    if (passphrase_out == NULL) {
        Abort("_get_mnemonic_passphrase");
    }
    while (true) {
        password_enter_blocking("Enter\noptional passphrase", true, passphrase_out);
        if (strlen(passphrase_out) == 0) {
            // No need to confirm the empty passphrase.
            break;
        }
        const confirm_params_t params = {
            .title = "",
            .body = "You will be asked to\nvisually confirm your\npassphrase now.",
            .accept_only = true,
        };
        if (!workflow_confirm_blocking(&params)) {
            return false;
        }
        if (workflow_confirm_scrollable_longtouch_blocking(
                "Confirm", passphrase_out, &font_password_11X12)) {
            break;
        }
        workflow_status_blocking("Please try again", false);
    }
    return true;
}
