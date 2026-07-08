// SPDX-License-Identifier: Apache-2.0

#include <sam.h>
#include <stdint.h>

extern uint32_t _estack;
extern uint32_t _sfixed;
extern uint32_t _etext;
extern uint32_t _srelocate;
extern uint32_t _erelocate;
extern uint32_t _szero;
extern uint32_t _ezero;

int main(void);
void Dummy_Handler(void);

typedef void (*stage0_handler_t)(void);

typedef union {
    void* ptr;
    stage0_handler_t handler;
    uintptr_t reserved;
} stage0_vector_t;

__attribute__((section(".vectors"), used)) const stage0_vector_t exception_table[] = {
    {.ptr = &_estack},
    {.handler = Reset_Handler},
    {.handler = Dummy_Handler},
    {.handler = Dummy_Handler},
    {.handler = Dummy_Handler},
    {.handler = Dummy_Handler},
    {.handler = Dummy_Handler},
    {.reserved = 0},
    {.reserved = 0},
    {.reserved = 0},
    {.reserved = 0},
    {.handler = Dummy_Handler},
    {.handler = Dummy_Handler},
    {.reserved = 0},
    {.handler = Dummy_Handler},
    {.handler = Dummy_Handler},
};

// GCC LTO needs externally_visible; clang-tidy parses with Clang and does not support it.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void __attribute__((noreturn, used, externally_visible)) Reset_Handler(void)
{
    uint32_t* src = &_etext;
    for (uint32_t* dst = &_srelocate; dst < &_erelocate;) {
        // NOLINTNEXTLINE(clang-analyzer-security.ArrayBound)
        *dst++ = *src++;
    }

    for (uint32_t* dst = &_szero; dst < &_ezero;) {
        // NOLINTNEXTLINE(clang-analyzer-security.ArrayBound)
        *dst++ = 0;
    }

    SCB->VTOR = ((uint32_t)&_sfixed & SCB_VTOR_TBLOFF_Msk);

#if __FPU_USED
    SCB->CPACR |= (0xFU << 20);
    __DSB();
    __ISB();
#endif

    main();

    while (1) {
    }
}

// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void __attribute__((noreturn, used, externally_visible)) Dummy_Handler(void)
{
    while (1) {
    }
}
