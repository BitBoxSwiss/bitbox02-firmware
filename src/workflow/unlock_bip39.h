#ifndef _UNLOCK_BIP39_H
#define _UNLOCK_BIP39_H

#include "workflow.h"

#include <util.h>

/**
 * Unlocks BIP39 with the default empty passphrase, or with a user provided passphrase if
 * mnemonic passphrase support is enabled.
 * Displays a simple unlock animation.
 */
USE_RESULT
workflow_t* workflow_unlock_bip39(void (*callback)(void* param), void* callback_param);

/**
 * Blocking wrapper around workflow_unlock_bip39.
 */
void workflow_unlock_bip39_blocking(void);

#endif //  _UNLOCK_BIP39_H
