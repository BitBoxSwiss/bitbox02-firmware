#include "graphics.h"

#include <screen.h>
#include <ui/ugui/ugui.h>

void graphics_draw_image(
    const position_t* position,
    const dimension_t* dimension,
    const in_buffer_t* image)
{
    int x = position->left;
    int y = position->top;
    for (size_t i = 0; i < image->len; i++) {
        uint8_t b = image->data[i];
        for (int j = 0; j < 8; j++) {
            if (b & 0x80) {
                UG_DrawPixel(x, y, screen_front_color);
            }
            b <<= 1;
            x++;
            if (((x - position->left) % dimension->width) == 0) {
                x = position->left;
                y++;
            }
        }
    }
}
