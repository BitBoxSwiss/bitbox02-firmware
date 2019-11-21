# Enums

arm-none-eabi-gcc compiles with -fshort-enums by defualt. This means that if all variants of an
enum fits in a small type (such as `uint8_t`), then the enum will be backed by such a small type.


With `-fno-short-enums` (the default on other platforms) all enums will be `int32_t` sized as long
as they fit, otherwise `int64_t`.  `repr(C)` in rust also follows this and can therefore not be
used. Instead all enums that are exported must have an explicit size using `repr(u8)` for example.
