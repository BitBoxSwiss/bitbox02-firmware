// SPDX-License-Identifier: Apache-2.0

#include <errno.h>
#include <limits.h>
#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#include <cmocka.h>

uint8_t test_heap[16];

__asm__(
    ".global _heap_start\n"
    "_heap_start = test_heap\n"
    ".global _heap_end\n"
    "_heap_end = test_heap + 16\n");

extern caddr_t _sbrk(int incr);

// Host malloc/free do not use the firmware's _sbrk implementation.
static void test_sbrk_bounds(void** state)
{
    (void)state;

    assert_ptr_equal(_sbrk(0), test_heap);
    assert_ptr_equal(_sbrk(sizeof(test_heap)), test_heap);

    errno = 0;
    assert_ptr_equal(_sbrk(1), (caddr_t)-1);
    assert_int_equal(errno, ENOMEM);
    assert_ptr_equal(_sbrk(0), test_heap + sizeof(test_heap));

    assert_ptr_equal(_sbrk(-(int)sizeof(test_heap)), test_heap + sizeof(test_heap));

    errno = 0;
    assert_ptr_equal(_sbrk(-1), (caddr_t)-1);
    assert_int_equal(errno, EINVAL);
    assert_ptr_equal(_sbrk(0), test_heap);

    errno = 0;
    assert_ptr_equal(_sbrk(INT_MIN), (caddr_t)-1);
    assert_int_equal(errno, EINVAL);
    assert_ptr_equal(_sbrk(0), test_heap);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_sbrk_bounds),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
