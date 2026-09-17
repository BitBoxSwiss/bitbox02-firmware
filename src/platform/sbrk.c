// SPDX-License-Identifier: Apache-2.0

#include <errno.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

extern uint8_t _heap_start;
extern uint8_t _heap_end;

caddr_t _sbrk(int incr);

// Unlike ASF4's implementation, this confines the program break to the
// linker-defined heap and checks growth and shrink without overflowing.
caddr_t _sbrk(int incr)
{
    static uint8_t* heap;
    const uintptr_t heap_start = (uintptr_t)&_heap_start;
    const uintptr_t heap_end = (uintptr_t)&_heap_end;

    if (heap == NULL) {
        heap = &_heap_start;
    }

    const uintptr_t current = (uintptr_t)heap;
    if (incr >= 0) {
        const uintptr_t increment = (uintptr_t)incr;
        if (current > heap_end || increment > heap_end - current) {
            errno = ENOMEM;
            return (caddr_t)-1;
        }
        heap = (uint8_t*)(current + increment);
    } else {
        const uintptr_t decrement = (uintptr_t)(-(int64_t)incr);
        if (current < heap_start || decrement > current - heap_start) {
            errno = EINVAL;
            return (caddr_t)-1;
        }
        heap = (uint8_t*)(current - decrement);
    }

    return (caddr_t)current;
}
