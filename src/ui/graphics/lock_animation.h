#ifndef _LOCK_ANIMATION_H
#define _LOCK_ANIMATION_H

/** ~3800ms unlock time, measured using this timer. */
#define LOCK_ANIMATION_N_FRAMES (38)

#define LOCK_ANIMATION_FRAME_WIDTH (28)
#define LOCK_ANIMATION_FRAME_HEIGHT (25)
#define LOCK_ANIMATION_FRAME_SIZE \
    ((LOCK_ANIMATION_FRAME_WIDTH * LOCK_ANIMATION_FRAME_HEIGHT + 7) / 8)

#include <stdint.h>

/**
 * Gets a frame of the lock animation.
 */
const uint8_t* lock_animation_get_frame(int frame_idx);

#endif // _LOCK_ANIMATION_H
