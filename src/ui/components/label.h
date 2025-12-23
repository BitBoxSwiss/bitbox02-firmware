// SPDX-License-Identifier: Apache-2.0

#ifndef _LABEL_H_
#define _LABEL_H_

#include <ui/component.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

// Max size of text shown (excl. null terminator). The current size of 640 is chosen to be able to
// show up to 320 bytes of Ethereum tx data in hex format.
#define MAX_LABEL_SIZE 640

/**
 * Creates a label with the given font and positions it in the center.
 * @param[in] component The component to update.
 * @param[in] text The new text of the label.
 */
void label_update(component_t* component, const char* text);

/**
 * Creates a label with the given font. If the text is longer than MAX_LABEL_SIZE, it is truncated
 * and suffixed with '...'.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] parent The parent component.
 */
component_t* label_create(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent);

/**
 * Creates a label with the given font.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] xoffset The horizontal offset.
 * @param[in] yoffset The vertical offset.
 * @param[in] parent The parent component.
 */
component_t* label_create_offset(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t xoffset,
    uint8_t yoffset,
    component_t* parent);

/**
 * Creates a label with the given font that can be scrolled.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] parent The parent component.
 */
component_t* label_create_scrollable(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent);

/**
 * Creates a label with the given font that can be scrolled.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] xoffset The horizontal offset.
 * @param[in] yoffset The vertical offset.
 * @param[in] parent The parent component.
 */
component_t* label_create_scrollable_offset(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t xoffset,
    uint8_t yoffset,
    component_t* parent);

#endif
