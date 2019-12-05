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

#include "orientation_arrows.h"
#include "button.h"
#include "image.h"
#include "ui_images.h"

#include <hardfault.h>
#include <screen.h>
#include <util.h>

#include <string.h>

#if !defined(TESTING)
#include <qtouch.h>
#else
#include <mock_qtouch.h>
#endif

#define SCALE 2 // Divide `count` by scale to slow down motion
#define TEXT "Tap this side"
#define COUNT_CHANGE_DIRECTION (SCREEN_WIDTH / 2 + IMAGE_DEFAULT_ARROW_HEIGHT - 2)
#define COUNT_SHOW_TEXT (COUNT_CHANGE_DIRECTION + SCREEN_HEIGHT / 2 + 24)

/**
 * The orientation data.
 */
typedef struct {
    bool enable_touch;
    uint16_t screen_count;
    void (*done_callback)(bool, void*);
    void* cb_param;
} orientation_data_t;

/**
 * Calls the done callback with the flip flag enabled.
 */
static void _flip(component_t* component)
{
    orientation_data_t* data = (orientation_data_t*)component->parent->data;
    if (data->enable_touch) {
        data->done_callback(true, data->cb_param);
    }
}

/**
 * Calls the done callback with the flip flag disabled.
 */
static void _stay(component_t* component)
{
    orientation_data_t* data = (orientation_data_t*)component->parent->data;
    if (data->enable_touch) {
        data->done_callback(false, data->cb_param);
    }
}

static void _render(component_t* component)
{
    orientation_data_t* data = (orientation_data_t*)component->data;
    int16_t x;
    int16_t y;
    int16_t height = IMAGE_DEFAULT_ARROW_HEIGHT;
    int16_t position = data->screen_count / SCALE;
    if (position < COUNT_CHANGE_DIRECTION) {
        // Horizontal motion
        x = position;
        y = SCREEN_HEIGHT / 2 - height;
        image_arrow(x - height + 2, y, height, ARROW_RIGHT);
        image_arrow(SCREEN_WIDTH - x - 2, y, height, ARROW_LEFT);
    } else if (position < COUNT_SHOW_TEXT) {
        // Vertical motion
        x = SCREEN_WIDTH / 2 - height;
        y = position - COUNT_CHANGE_DIRECTION + SCREEN_HEIGHT / 2;
        image_arrow(x, y, height, ARROW_DOWN);
        image_arrow(x, SCREEN_HEIGHT - y - height, height, ARROW_UP);
    } else if (position < COUNT_SHOW_TEXT + SCALE * 12) {
        // Zoom in to rotate arrow
        uint8_t r;
        r =
            MIN(IMAGE_ROTATE_H / 2,
                MAX(0, (position - COUNT_SHOW_TEXT - (12 - IMAGE_ROTATE_H / 2)) / SCALE));
        UG_DrawCircle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, r, screen_front_color);
        // Raise text
        y = (position - COUNT_SHOW_TEXT) / SCALE; // Slower movement
        for (int i = 0; i < 2; i++) {
            component_t* sc = component->sub_components.sub_components[i];
            sc->position.top =
                i ? MIN(0, y - 12) : SCREEN_HEIGHT - sc->dimension.height - MIN(0, y - 12);
            sc->f->render(sc);
        }
    } else {
        // Render sub-components
        uint8_t bounce = 5;
        int16_t period = 512;
        for (int i = 0; i < 2; i++) {
            // Calculate bounce position
            y = ((data->screen_count - period / 4) % period) - (i ? 0 : period / 2);
            if (y > bounce * SCALE * 4 || y < 0) {
                // No bounce
                y = 0;
            } else if (y > bounce * SCALE * 2) {
                y = bounce * SCALE * 4 - y;
            } else if (y > 0) {
                // y = y;
            }
            y = y / SCALE / 4;
            // Bounce text
            component_t* sc = component->sub_components.sub_components[i];
            sc->position.top = i ? y : SCREEN_HEIGHT - sc->dimension.height - y;
            sc->f->render(sc);
        }
        if ((data->screen_count - period / 4) % period < period / 2) {
            component_t* sc_rotate = component->sub_components.sub_components[2];
            sc_rotate->f->render(sc_rotate);
        } else {
            component_t* sc_rotate_reverse = component->sub_components.sub_components[3];
            sc_rotate_reverse->f->render(sc_rotate_reverse);
        }
        data->enable_touch = true;
    }
    qtouch_force_calibrate();
    data->screen_count++;
}

/********************************** Component Functions **********************************/

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = ui_util_on_event_noop,
};

/********************************** Create Instance **********************************/

component_t* orientation_arrows_create(void (*done_callback)(bool, void*), void* cb_param)
{
    component_t* orientation = malloc(sizeof(component_t));
    if (!orientation) {
        Abort("Error: malloc orientation");
    }
    orientation_data_t* data = malloc(sizeof(orientation_data_t));
    if (!data) {
        Abort("Error: malloc orientation data");
    }
    memset(data, 0, sizeof(orientation_data_t));
    memset(orientation, 0, sizeof(component_t));

    data->done_callback = done_callback;
    data->cb_param = cb_param;
    data->screen_count = 0;
    data->enable_touch = false;

    orientation->data = data;
    orientation->f = &_component_functions;
    orientation->dimension.width = SCREEN_WIDTH;
    orientation->dimension.height = SCREEN_HEIGHT;
    orientation->position.top = 0;
    orientation->position.left = 0;

    component_t* button_normal = button_create_wide(TEXT, bottom_slider, _stay, orientation);
    component_t* button_upside_down =
        button_create_wide_upside_down(TEXT, top_slider, _flip, orientation);
    component_t* rotate = image_create(
        IMAGE_ROTATE, sizeof(IMAGE_ROTATE), IMAGE_ROTATE_W, IMAGE_ROTATE_H, CENTER, orientation);
    component_t* rotate_reverse = image_create(
        IMAGE_ROTATE_REVERSE,
        sizeof(IMAGE_ROTATE),
        IMAGE_ROTATE_W,
        IMAGE_ROTATE_H,
        CENTER,
        orientation);

    // Order/presence is important and affects rendering `sc->f->render(sc)`;
    ui_util_add_sub_component(orientation, button_normal);
    ui_util_add_sub_component(orientation, button_upside_down);
    ui_util_add_sub_component(orientation, rotate);
    ui_util_add_sub_component(orientation, rotate_reverse);

    return orientation;
}
