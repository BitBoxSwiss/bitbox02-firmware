// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <cmocka.h>

#include <ui/screen_process.h>
#include <ui/screen_stack.h>

typedef struct {
    int renders;
    int cleanups;
    bool free_on_cleanup;
} component_state_t;

static void _component_cleanup(component_t* component)
{
    component_state_t* state = component->data;
    state->cleanups++;
    if (state->free_on_cleanup) {
        free(component);
    }
}

static void _component_render(component_t* component)
{
    component_state_t* state = component->data;
    state->renders++;
}

static const component_functions_t COMPONENT_FUNCTIONS = {
    .cleanup = _component_cleanup,
    .render = _component_render,
    .on_event = NULL,
};

static component_t* _component_create(component_state_t* state)
{
    component_t* component = calloc(1, sizeof(component_t));
    assert_non_null(component);
    component->f = &COMPONENT_FUNCTIONS;
    component->data = state;
    return component;
}

static component_t* _screensaver = NULL;
static component_state_t _waiting_state = {0};
static component_t _waiting_component = {
    .f = &COMPONENT_FUNCTIONS,
    .data = &_waiting_state,
};
static int _waiting_creates = 0;

component_t* __wrap_screen_saver_get(void)
{
    return _screensaver;
}

void __wrap_screen_saver_process(void) {}

component_t* __wrap_waiting_create(void)
{
    _waiting_creates++;
    return &_waiting_component;
}

static void test_screen_process_frame_hold(void** state)
{
    (void)state;

    // Popped components must still be cleaned while the framebuffer is held.
    component_state_t popped_state = {.free_on_cleanup = true};
    component_t* popped = _component_create(&popped_state);
    ui_screen_stack_push(popped);
    ui_screen_stack_pop();
    screen_process(true);
    assert_int_equal(popped_state.cleanups, 1);
    assert_int_equal(_waiting_creates, 0);

    // An explicit component takes precedence over the framebuffer hold.
    component_state_t stacked_state = {.free_on_cleanup = true};
    component_t* stacked = _component_create(&stacked_state);
    ui_screen_stack_push(stacked);
    screen_process(true);
    screen_process(true);
    assert_true(stacked_state.renders > 0);
    assert_int_equal(_waiting_creates, 0);
    ui_screen_stack_pop();
    screen_process(true);
    assert_int_equal(stacked_state.cleanups, 1);

    // The screensaver also takes precedence over the framebuffer hold.
    component_state_t screensaver_state = {0};
    component_t screensaver = {
        .f = &COMPONENT_FUNCTIONS,
        .data = &screensaver_state,
    };
    _screensaver = &screensaver;
    screen_process(true);
    screen_process(true);
    assert_true(screensaver_state.renders > 0);
    assert_int_equal(_waiting_creates, 0);
    _screensaver = NULL;

    // Without the hold, an empty stack retains the existing waiting-screen behavior.
    screen_process(false);
    screen_process(false);
    assert_int_equal(_waiting_creates, 1);
    assert_true(_waiting_state.renders > 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_screen_process_frame_hold),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
