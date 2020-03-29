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
 * Start unlock animation via a timer that draws frames. To be calle before bip39 unlock.
 * Must call lock_animation_stop after the unlock is finished.
 */
void lock_animation_start(void);
void lock_animation_stop(void);

#endif // _LOCK_ANIMATION_H
