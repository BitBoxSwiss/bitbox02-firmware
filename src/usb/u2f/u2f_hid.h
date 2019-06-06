// Copyright 2014 Google Inc. All rights reserved.
// Copyright 2017-2019 Shift Cryptosecurity Ag
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef _U2F_HID_H_
#define _U2F_HID_H_

#include <stdint.h>

#define U2FHID_CID_BROADCAST 0xffffffff // Broadcast channel id

// General constants
#define U2FHID_IF_VERSION 2 // Current interface implementation version
#define U2FHID_FRAME_TIMEOUT 500 // Default frame timeout in ms
#define U2FHID_TRANS_TIMEOUT 3000 // Default message timeout in ms

// U2FHID native commands
#define U2FHID_PING (FRAME_TYPE_INIT | 0x01) // Echo data
#define U2FHID_MSG FRAME_MSG // Send U2F message frame
#define U2FHID_LOCK (FRAME_TYPE_INIT | 0x04) // Send lock channel command
#define U2FHID_INIT (FRAME_TYPE_INIT | 0x06) // Channel initialization
#define U2FHID_WINK (FRAME_TYPE_INIT | 0x08) // Send device identification wink
#define U2FHID_SYNC (FRAME_TYPE_INIT | 0x3c) // Send sync command
#define U2FHID_ERROR FRAME_ERROR // Error response

// U2FHID_INIT command defines
#define U2FHID_INIT_NONCE_SIZE 8
#define U2FHID_CAPFLAG_WINK 0x01 // Device supports WINK command
#define U2FHID_CAPFLAG_LOCK 0x02 // Device supports LOCK command

// Error codes; return as negatives
#define U2FHID_ERR_NONE 0x00
#define U2FHID_ERR_INVALID_CMD 0x01
#define U2FHID_ERR_INVALID_PAR 0x02
#define U2FHID_ERR_INVALID_LEN FRAME_ERR_INVALID_LEN
#define U2FHID_ERR_INVALID_SEQ FRAME_ERR_INVALID_SEQ
#define U2FHID_ERR_MSG_TIMEOUT 0x05
#define U2FHID_ERR_CHANNEL_BUSY FRAME_ERR_CHANNEL_BUSY
#define U2FHID_ERR_LOCK_REQUIRED 0x0a
#define U2FHID_ERR_INVALID_CID 0x0b
#define U2FHID_ERR_OTHER FRAME_ERR_OTHER

typedef struct {
    uint8_t nonce[U2FHID_INIT_NONCE_SIZE];
} U2FHID_INIT_REQ;

typedef struct {
    uint8_t nonce[U2FHID_INIT_NONCE_SIZE];
    uint32_t cid;
    uint8_t versionInterface;
    uint8_t versionMajor;
    uint8_t versionMinor;
    uint8_t versionBuild;
    uint8_t capFlags; // Capabilities flags
} U2FHID_INIT_RESP;

#define U2FHID_INIT_RESP_SIZE sizeof(U2FHID_INIT_RESP)

#endif
