#ifndef _GET_MNEMONIC_PASSPHRASE_H
#define _GET_MNEMONIC_PASSPHRASE_H

#include <stdbool.h>

#include "workflow.h"

workflow_t* workflow_get_mnemonic_passphrase(void (*callback)(char*, void*), void* callback_param);
void get_mnemonic_passphrase_blocking(char* passphrase_out);

#endif // _GET_MNEMONIC_PASSPHRASE_H
