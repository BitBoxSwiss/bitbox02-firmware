# Changelog

## [Unreleased]

## [v0.4.0] 2024-12-30

* Upgrade to embedded-hal v1.0 [#13]

## [v0.3.2] 2024-08-04

* Fix: Add import for IS42S32400F

## [v0.3.1] 2024-08-04

* Add IS42S32400F to devices [#14]

## [v0.3.0] 2022-12-29

* Parallel NAND Flash is supported with the `nand ` feature flag [#9]
* Add S34ML08G3 NAND [#8]
* Add support for defmt logging. Add support for dumping register contents at the end of init function. [#7]

## [v0.2.4] 2021-10-05

* Add AS4C4M16SA-6 device [#5]

## [v0.2.3] 2021-05-25

* Fix the number of columns for the MT48LC4M32B2 device [#4]

## [v0.2.2] 2021-03-27

* Implement AS4C16M32MSA-6BIN device [#3]

## [v0.2.1] 2020-11-07

* Export SdramConfiguration and SdramTiming structs to fix implementing
  SdramChip outside the crate https://github.com/stm32-rs/stm32-fmc/pull/2

## [v0.2.0] 2020-08-28

* *Breaking*: Use a generic type to support pin checking on SDRAMs with 11 and
  13 address lines. `PinsSdram` now has two generic types.

## [v0.1.2] 2020-08-05

* Don't require type to be `Sync` in order to implement FmcPeripheral
* Begin Changelog

[Unreleased]: https://github.com/stm32-rs/stm32-fmc/compare/v0.4.0...HEAD
[v0.3.2]: https://github.com/stm32-rs/stm32-fmc/compare/v0.3.2...v0.4.0
[v0.3.2]: https://github.com/stm32-rs/stm32-fmc/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/stm32-rs/stm32-fmc/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/stm32-rs/stm32-fmc/compare/v0.2.4...v0.3.0
[v0.2.4]: https://github.com/stm32-rs/stm32-fmc/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/stm32-rs/stm32-fmc/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/stm32-rs/stm32-fmc/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/stm32-rs/stm32-fmc/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/stm32-rs/stm32-fmc/compare/v0.1.2...v0.2.0

[#3]: https://github.com/stm32-rs/stm32-fmc/pull/3
[#4]: https://github.com/stm32-rs/stm32-fmc/pull/4
[#5]: https://github.com/stm32-rs/stm32-fmc/pull/5
[#7]: https://github.com/stm32-rs/stm32-fmc/pull/7
[#8]: https://github.com/stm32-rs/stm32-fmc/pull/8
[#9]: https://github.com/stm32-rs/stm32-fmc/pull/9
[#13]: https://github.com/stm32-rs/stm32-fmc/pull/13
[#14]: https://github.com/stm32-rs/stm32-fmc/pull/14
