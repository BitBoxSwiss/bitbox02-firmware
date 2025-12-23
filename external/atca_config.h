// SPDX-License-Identifier: Apache-2.0

// This file is based on the snippet from cryptoauthlib/README.md:
// https://github.com/BitBoxSwiss/cryptoauthlib/tree/v3.2.5#configuration

/* Cryptoauthlib Configuration File */
#ifndef ATCA_CONFIG_H
#define ATCA_CONFIG_H

/* Include HALS */
// Shift: we currently use a custom HAL config, see `ATCAIfaceCfg cfg` in securechip.c.
#define ATCA_HAL_CUSTOM

/* Included device support */
#define ATCA_ATECC608_SUPPORT

/* \brief How long to wait after an initial wake failure for the POST to
 *         complete.
 * If Power-on self test (POST) is enabled, the self test will run on waking
 * from sleep or during power-on, which delays the wake reply.
 */
#ifndef ATCA_POST_DELAY_MSEC
#define ATCA_POST_DELAY_MSEC 25
#endif

#endif // ATCA_CONFIG_H
