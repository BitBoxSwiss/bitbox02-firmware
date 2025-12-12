// SPDX-License-Identifier: Apache-2.0

#ifndef _FAKE_COMPONENT_H_
#define _FAKE_COMPONENT_H_

#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a label with the given font either upside down or normal.
 * @param[in] text The text of the label.
 * @param[in] upside_down Whether the text should be rotated 180 degree or not.
 * @param[in] font The font of the label.
 */
component_t* fake_component_create(void);

#endif
