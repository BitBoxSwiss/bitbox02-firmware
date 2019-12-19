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

#ifndef _UI_UTIL_
#define _UI_UTIL_

#include <ui/component.h>
#include <ui/ugui/ugui.h>

#define UI_UTIL_VISIBLE_SPACE_WIDTH 5

enum screen_position_t {
    CENTER,
    CENTER_TOP,
    CENTER_BOTTOM,
    LEFT_TOP,
    LEFT_BOTTOM,
    LEFT_CENTER,
    RIGHT_TOP,
    RIGHT_BOTTOM,
    RIGHT_CENTER,
    CUSTOM_OFFSET
};

/**
 * A utility function that adds a child component to a parent component.
 * Ensures that the maximum number of sub-components, which can be associated with one component, is
 * not overstepped.
 * @param[OUT] parent The given child component is added to the parent's sub components.
 * @param[in] child The added child component.
 */
void ui_util_add_sub_component(component_t* parent, component_t* child);

/**
 * A utility function that renders all sub-components.
 * @param[in] component The rendered component.
 */
void ui_util_component_render_subcomponents(component_t* component);

/**
 * A utility function that cleans up the current component and all sub-components.
 * @param[in] component The cleaned up component.
 */
void ui_util_component_cleanup(component_t* component);

/**
 * A no-op function for components that do not handle events.
 * @param[in] event The emitted event.
 * @param[in] component The component that receives the event.
 */
void ui_util_on_event_noop(const event_t* event, component_t* component);

/**
 * Positions the child component in the center (vertical and horizontal) of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_center(component_t* parent, component_t* child);

/**
 * Positions the child component in the top center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_center_top(component_t* parent, component_t* child);

/**
 * Positions the child component in the bottom center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_center_bottom(component_t* parent, component_t* child);

/**
 * Positions the child component on the left bottom of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_left_bottom(component_t* parent, component_t* child);

/**
 * Positions the child component on the right bottom of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_right_bottom(component_t* parent, component_t* child);

/**
 * Positions the child component on the left top of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_left_top(component_t* parent, component_t* child);

/**
 * Positions the child component on the right top of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_right_top(component_t* parent, component_t* child);

/**
 * Positions the child component on the left top of the
 * parent component with the given offsets.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 * @param[in] left_offset The horizontal offset to the parent component's position.
 * @param[in] top_offset The vertical offset to the parent component's position.
 */
void ui_util_position_left_top_offset(
    component_t* parent,
    component_t* child,
    int16_t left_offset,
    uint8_t top_offset);

/**
 * Positions the child component on the left bottom of the
 * parent component with the given offsets.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 * @param[in] left_offset The horizontal offset to the parent component's position.
 * @param[in] bottom_offset The vertical offset to the parent component's position from the bottom.
 */
void ui_util_position_left_bottom_offset(
    component_t* parent,
    component_t* child,
    int16_t left_offset,
    uint8_t bottom_offset);

/**
 * Positions the child component on the left center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_left_center(component_t* parent, component_t* child);

/**
 * Positions the child component on the right center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_right_center(component_t* parent, component_t* child);

/**
 * Positions the child component on the left center of the
 * parent component with the given offsets.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 * @param[in] left_offset The horizontal offset to the parent component's position.
 */
void ui_util_position_left_center_offset(
    component_t* parent,
    component_t* child,
    int16_t left_offset);

/**
 * Positions the child component on the right center of the
 * parent component with the given offsets.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 * @param[in] right_offset The horizontal offset to the parent component's position.
 */
void ui_util_position_right_center_offset(
    component_t* parent,
    component_t* child,
    int16_t right_offset);

#endif
