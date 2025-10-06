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

#include <stdbool.h>

#include <hardfault.h>
#include <screen.h>
#include <ui/ui_util.h>

/**
 * A utility function that adds a child component to a parent component.
 * Ensures that the maximum number of sub-components, which can be associated with one component, is
 * not overstepped.
 * @param[out] parent The given child component is added to the parent's sub components.
 * @param[in] child The added child component.
 */
void ui_util_add_sub_component(component_t* parent, component_t* child)
{
    if (parent->sub_components.amount + 1 >= MAX_NUM_SUBCOMPONENTS) {
        Abort("Not enough memory to add sub component");
    }
    parent->sub_components.sub_components[parent->sub_components.amount] = child;
    parent->sub_components.amount++;
    child->parent = parent;
}

/**
 * A utility function that renders all sub-components.
 * @param[in] component The rendered component.
 */
void ui_util_component_render_subcomponents(component_t* component)
{
    for (int i = 0; i < component->sub_components.amount; i++) {
        component_t* comp = component->sub_components.sub_components[i];
        if (!comp->disabled) {
            comp->f->render(comp);
        }
    }
}

/**
 * A utility function that cleans up the current component and all sub-components.
 * @param[in] component The cleaned up component.
 */
void ui_util_component_cleanup(component_t* component)
{
    for (int i = 0; i < component->sub_components.amount; i++) {
        component->sub_components.sub_components[i]->f->cleanup(
            component->sub_components.sub_components[i]);
    }
    free(component->data);
    free(component);
}

/**
 * Positions the child component in the center (vertical and horizontal) of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_center(component_t* parent, component_t* child)
{
    child->position.top =
        parent->position.top + parent->dimension.height / 2 - child->dimension.height / 2;
    child->position.left =
        parent->position.left + parent->dimension.width / 2 - child->dimension.width / 2;
}

/**
 * Positions the child component in the top center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_center_top(component_t* parent, component_t* child)
{
    child->position.top = parent->position.top;
    child->position.left =
        parent->position.left + parent->dimension.width / 2 - child->dimension.width / 2;
}

/**
 * Positions the child component in the bottom center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_center_bottom(component_t* parent, component_t* child)
{
    child->position.top = parent->position.top + parent->dimension.height - child->dimension.height;
    child->position.left =
        parent->position.left + parent->dimension.width / 2 - child->dimension.width / 2;
}

static void _position_left(component_t* parent, component_t* child)
{
    child->position.left = parent->position.left;
    // The frame of some SSD1312 screens slightly covers around 0.5px on each side, cutting off the
    // text a bit, especially when looking at the device at an angle. We move by one pixel to
    // improve visibility.
    if (child->position.left == 0) {
        child->position.left = 1;
    }
}

static void _position_right(component_t* parent, component_t* child)
{
    child->position.left = parent->position.left + parent->dimension.width - child->dimension.width;
    // The frame of some SSD1312 screens slightly covers around 0.5px on each side, cutting off the
    // text a bit, especially when looking at the device at an angle. We move by one pixel to
    // improve visibility.
    if (parent->position.left + parent->dimension.width == SCREEN_WIDTH) {
        child->position.left -= 1;
    }
}

/**
 * Positions the child component on the left bottom of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_left_bottom(component_t* parent, component_t* child)
{
    child->position.top = parent->position.top + parent->dimension.height - child->dimension.height;
    _position_left(parent, child);
}

/**
 * Positions the child component on the left center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_left_center(component_t* parent, component_t* child)
{
    child->position.top =
        parent->position.top + parent->dimension.height / 2 - child->dimension.height / 2;
    _position_left(parent, child);
}

/**
 * Positions the child component on the right center of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_right_center(component_t* parent, component_t* child)
{
    child->position.top =
        parent->position.top + parent->dimension.height / 2 - child->dimension.height / 2;
    _position_right(parent, child);
}

/**
 * Positions the child component on the right bottom of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_right_bottom(component_t* parent, component_t* child)
{
    child->position.top = parent->position.top + parent->dimension.height - child->dimension.height;
    _position_right(parent, child);
}

/**
 * Positions the child component on the left top of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_left_top(component_t* parent, component_t* child)
{
    child->position.top = parent->position.top;
    _position_left(parent, child);
}

/**
 * Positions the child component on the right bottom of the
 * parent component.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 */
void ui_util_position_right_top(component_t* parent, component_t* child)
{
    child->position.top = parent->position.top;
    _position_right(parent, child);
}

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
    uint8_t top_offset)
{
    child->position.top = parent->position.top + top_offset;
    child->position.left = parent->position.left + left_offset;
}

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
    uint8_t bottom_offset)
{
    child->position.top =
        parent->position.top + parent->dimension.height - child->dimension.height - bottom_offset;
    child->position.left = parent->position.left + left_offset;
}

/**
 * Positions the child component on the left center of the
 * parent component with the given offset.
 * @param[in] parent The parent component.
 * @param[in] child The child/sub-component.
 * @param[in] left_offset The horizontal offset to the parent component's position.
 */
void ui_util_position_left_center_offset(
    component_t* parent,
    component_t* child,
    int16_t left_offset)
{
    child->position.top =
        parent->position.top + (parent->dimension.height - child->dimension.height) / 2;
    child->position.left = parent->position.left + left_offset;
}
