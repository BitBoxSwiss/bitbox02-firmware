// SPDX-License-Identifier: Apache-2.0

#include <stddef.h>

#include "hardfault.h"
#include "screen_saver.h"
#include "screen_stack.h"

#define UI_SCREEN_STACK_MAX_ELEMENTS 5

typedef struct {
    component_t* screens[UI_SCREEN_STACK_MAX_ELEMENTS];
    int size;
} ui_stack_t;

static ui_stack_t _screen_stack = {0};

// when we pop a component from the stack, we don't destroy it immediately, but
// defer that, so that all recursive operations on the component can finish.
static ui_stack_t _pop_stack = {0};

component_t* ui_screen_stack_top(void)
{
    if (_screen_stack.size > 0) {
        component_t* top = _screen_stack.screens[_screen_stack.size - 1];
        return top;
    }
    return NULL;
}

void ui_screen_stack_pop(void)
{
    if (_screen_stack.size > 0) {
        component_t* top = _screen_stack.screens[_screen_stack.size - 1];
        _screen_stack.size--;

        // Put on pop stack, to clean up later.
        if (_pop_stack.size < UI_SCREEN_STACK_MAX_ELEMENTS) {
            _pop_stack.screens[_pop_stack.size] = top;
            _pop_stack.size++;
        } else {
            Abort("Abort: ui_screen_stack_pop");
        }

        screen_saver_reset();
    }
}

void ui_screen_stack_pop_and_clean(void)
{
    if (_screen_stack.size > 0) {
        component_t* top = _screen_stack.screens[_screen_stack.size - 1];
        _screen_stack.size--;

        top->f->cleanup(top);
    }
}

void ui_screen_stack_pop_all(void)
{
    while (_screen_stack.size) {
        ui_screen_stack_pop();
    }
}

void ui_screen_stack_push(component_t* component)
{
    if (_screen_stack.size < UI_SCREEN_STACK_MAX_ELEMENTS) {
        _screen_stack.screens[_screen_stack.size] = component;
        _screen_stack.size++;
        screen_saver_reset();
    } else {
        Abort("Abort: ui_screen_stack_push");
    }
}

void ui_screen_stack_cleanup(void)
{
    while (_pop_stack.size) {
        component_t* top = _pop_stack.screens[_pop_stack.size - 1];
        _pop_stack.size--;
        top->f->cleanup(top);
    }
}
