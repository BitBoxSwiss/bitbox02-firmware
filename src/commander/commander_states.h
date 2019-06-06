// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef _COMMANDER_STATES_H_
#define _COMMANDER_STATES_H_

#include <stdbool.h>
#include <stdint.h>

#include <pb.h>

typedef pb_size_t commander_states_endpoint_id;

// We have three main states:
// Creating a wallet on an uninitialized device goes through those states in order.
// Restoring a backup skips the seeded state and goes straight to `initialized`.
// Each state has a set of valid api calls associated.
typedef enum {
    // uninitialized (reset)
    COMMANDER_STATES_UNINITIALIZED,
    // seeded (password defined, seed created/loaded)
    COMMANDER_STATES_SEEDED,
    // initialized (seed backuped up on SD card)
    COMMANDER_STATES_INITIALIZED,
} commander_states_state_t;

/*
 * @return the state the device is in.
 */
commander_states_state_t commander_states_state(void);

/**
 * Makes the device only accept the given endpoint until it is processed. The restriction is lifted
 * by calling `commander_states_reset()`.
 */
void commander_states_force_next(commander_states_endpoint_id id);

/**
 * Lifts any additional restriction imposed by `commander_states_force_next`. This is meant to be
 * called right before processing a valid api call.
 */
void commander_states_clear_force_next(void);

/**
 * Checks if the device is ready to accept/handle an api endpoint.
 * @param[in] id id of the endpoint.
 * @return true if the device is in the right state to handle this api call, false otherwise.
 */
bool commander_states_can_call(commander_states_endpoint_id id);

#endif
