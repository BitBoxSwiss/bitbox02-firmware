#ifndef _GET_MNEMONIC_PASSPHRASE_H
#define _GET_MNEMONIC_PASSPHRASE_H

#include <stdbool.h>

#include "workflow.h"

workflow_t* workflow_get_mnemonic_passphrase(void (*callback)(char*, void*), void* callback_param);

#endif // _GET_MNEMONIC_PASSPHRASE_H
