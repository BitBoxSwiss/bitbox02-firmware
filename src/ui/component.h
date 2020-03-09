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

#ifndef _COMPONENT_H_
#define _COMPONENT_H_

#include <stdbool.h>
#include <stdint.h>

#include "ui/event.h"
#include "ui/graphics/graphics.h"

typedef struct component_t component_t;

// possibly needs to be adjusted
#define MAX_NUM_SUBCOMPONENTS 35

typedef struct {
    uint8_t amount;
    component_t* sub_components[MAX_NUM_SUBCOMPONENTS];
} sub_components_t;

typedef struct {
    /* must be called when the component is removed. */
    void (*const cleanup)(component_t*);
    /* After retrieving width and height of the component, the
     * parent component calls render with the positions that it calculated.
     * Positions are measured relative to the screen, not relative to its parent component. */
    void (*const render)(component_t*);
    void (*const on_event)(const event_t*, component_t*);
} component_functions_t;

struct component_t {
    /* Pointer to the component functions. */
    const component_functions_t* f;
    dimension_t dimension;
    position_t position;
    /* Additional data specific for the component. */
    void* data;
    /* Non-null pointer to the Sub_Components struct.
     * If there are no sub-components, the num_sub_components is 0. */
    sub_components_t sub_components;
    /* Pointer to the parent. Root components do not have a parent. */
    struct component_t* parent;
    /* Whether or not to require touch release before emitting touch events. */
    bool emit_without_release;
    /**
     * Whether the component is disabled/invisible.
     *
     * On creation, every component is enabled.
     * If a component is disabled, it won't be rendered unless
     * its render function is called explicitely, and it won't
     * receive events unless they're passed to it explicitely.
     */
    bool disabled;
};

#endif
